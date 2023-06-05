// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};

// std
use std::{collections::BTreeMap, sync::Arc};
// darwinia
use dc_primitives::*;
// moonbeam
use moonbeam_rpc_debug::{Debug, DebugServer};
use moonbeam_rpc_trace::{Trace, TraceServer};
use moonbeam_rpc_txpool::{TxPool, TxPoolServer};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P, A: sc_transaction_pool::ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance.
	pub graph: Arc<sc_transaction_pool::Pool<A>>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: sc_rpc::DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Network service
	pub network: Arc<sc_network::NetworkService<Block, Hash>>,
	/// Chain syncing service
	pub sync: Arc<sc_network_sync::SyncingService<Block>>,
	/// EthFilterApi pool.
	pub filter_pool: Option<fc_rpc_core::types::FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// Fee history cache.
	pub fee_history_cache: fc_rpc_core::types::FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: fc_rpc_core::types::FeeHistoryCacheLimit,
	/// Ethereum data access overrides.
	pub overrides: Arc<fc_rpc::OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<fc_rpc::EthBlockDataCacheTask<Block>>,
	/// Mandated parent hashes for a given block hash.
	pub forced_parent_hashes: Option<BTreeMap<sp_core::H256, sp_core::H256>>,
}

/// EVM tracing rpc server config
pub struct TracingConfig {
	pub tracing_requesters: crate::frontier_service::RpcRequesters,
	pub trace_filter_max_count: u32,
}

/// Default Ethereum RPC config
pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);
impl<C, BE> fc_rpc::EthConfig<Block, C> for DefaultEthConfig<C, BE>
where
	C: sc_client_api::StorageProvider<Block, BE> + Sync + Send + 'static,
	BE: sc_client_api::Backend<Block> + 'static,
{
	type EstimateGasAdapter = ();
	type RuntimeStorageOverride =
		fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<Block, C, BE>;
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, BE, A, EC>(
	deps: FullDeps<C, P, A>,
	subscription_task_executor: sc_rpc::SubscriptionTaskExecutor,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<Block>,
		>,
	>,
	maybe_tracing_config: Option<TracingConfig>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	BE: 'static + sc_client_api::backend::Backend<Block>,
	BE::State: sc_client_api::backend::StateBackend<Hashing>,
	C: 'static
		+ Send
		+ Sync
		+ sc_client_api::backend::AuxStore
		+ sc_client_api::backend::StorageProvider<Block, BE>
		+ sc_client_api::BlockchainEvents<Block>
		+ sp_api::CallApiAt<Block>
		+ sp_api::ProvideRuntimeApi<Block>
		+ sp_blockchain::HeaderBackend<Block>
		+ sp_blockchain::HeaderMetadata<Block, Error = sp_blockchain::Error>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ moonbeam_rpc_primitives_txpool::TxPoolRuntimeApi<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ sp_block_builder::BlockBuilder<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	P: 'static + Sync + Send + sc_transaction_pool_api::TransactionPool<Block = Block>,
	A: 'static + sc_transaction_pool::ChainApi<Block = Block>,
	EC: fc_rpc::EthConfig<Block, C>,
{
	// frontier
	use fc_rpc::{
		Eth, EthApiServer, EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, Net,
		NetApiServer, Web3, Web3ApiServer,
	};
	use fp_rpc::NoTransactionConverter;
	// substrate
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcExtension::new(());
	let FullDeps {
		client,
		pool,
		graph,
		deny_unsafe,
		is_authority,
		network,
		sync,
		filter_pool,
		backend,
		max_past_logs,
		fee_history_cache,
		fee_history_cache_limit,
		overrides,
		block_data_cache,
		forced_parent_hashes,
	} = deps;

	module.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph.clone(),
			<Option<NoTransactionConverter>>::None,
			sync.clone(),
			vec![],
			overrides.clone(),
			backend.clone(),
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_cache_limit,
			10,
			forced_parent_hashes,
		)
		.replace_config::<EC>()
		.into_rpc(),
	)?;

	if let Some(filter_pool) = filter_pool {
		module.merge(
			EthFilter::new(
				client.clone(),
				backend,
				filter_pool,
				500_usize, // max stored filters
				max_past_logs,
				block_data_cache,
			)
			.into_rpc(),
		)?;
	}

	module.merge(
		EthPubSub::new(
			pool,
			client.clone(),
			sync,
			subscription_task_executor,
			overrides,
			pubsub_notification_sinks,
		)
		.into_rpc(),
	)?;
	module.merge(
		Net::new(
			client.clone(),
			network,
			// Whether to format the `peer_count` response as Hex (default) or not.
			true,
		)
		.into_rpc(),
	)?;
	module.merge(Web3::new(client.clone()).into_rpc())?;
	module.merge(TxPool::new(client.clone(), graph).into_rpc())?;

	if let Some(tracing_config) = maybe_tracing_config {
		if let Some(trace_filter_requester) = tracing_config.tracing_requesters.trace {
			module.merge(
				Trace::new(client, trace_filter_requester, tracing_config.trace_filter_max_count)
					.into_rpc(),
			)?;
		}

		if let Some(debug_requester) = tracing_config.tracing_requesters.debug {
			module.merge(Debug::new(debug_requester).into_rpc())?;
		}
	}

	Ok(module)
}
