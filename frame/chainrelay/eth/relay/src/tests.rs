//! Tests for eth-relay.

use std::str::FromStr;

use eth_primitives::{
	receipt::{LogEntry, TransactionOutcome},
	Bloom, EthAddress, H64, U128,
};
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;
use hex_literal::hex;
use rustc_hex::FromHex;

use crate::{mock::*, mock_headers::*, *};

#[test]
fn verify_receipt_proof() {
	new_test_ext().execute_with(|| {
			System::inc_account_nonce(&2);

			assert_ok!(EthRelay::set_number_of_blocks_safe(RawOrigin::Root.into(), 0));

			// https://ropsten.etherscan.io/tx/0xce62c3d1d2a43cfcc39707b98de53e61a7ef7b7f8853e943d85e511b3451aa7e#eventlog
			let log_entries = vec![LogEntry {
				address: EthAddress::from_str("ad52e0f67b6f44cd5b9a6f4fbc7c0f78f37e094b").unwrap(),
				topics: vec![
					H256::from(hex!("6775ce244ff81f0a82f87d6fd2cf885affb38416e3a04355f713c6f008dd126a")),
					H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000006")),
					H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000000")),
				],
				data: "00000000000000000000000074241db5f3ebaeecf9506e4ae9881860933416048eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000000000000000000000000000000000000000000000000002386f26fc10000".from_hex().unwrap(),
			}];

			let receipt = Receipt::new(
				TransactionOutcome::StatusCode(1),
//				TransactionOutcome::StateRoot(H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863"))),
				U256::from(U128::from(1123401)),
				log_entries
			);


			let proof_record = EthReceiptProof {
				index: 25,
				proof: "f904c4f904c1b8b3f8b1a0c75e4fe93609c5f088e180e294577ba0f991fcad25e6163523adba4bfc65cfa8a008d8d33daaf581590c70f28317e5a48c33786ee092d7d9a9b4faae64fd05339ba0562b932c3332c149c7449d68be351f41c947c5f4b6d336906970f361dc905c67a0da77a1e9b271dcaaf156d5528be7e6c586930feab5d0e644208c0b8e54eed21780808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080b90214f90211a08fd1196d29f53e148b7cd38b1143b132d8f9bd4a9c5a2ad51244de514b5b5f19a0a6d91f439a4b87ec5861732d4900baa7df91c8b2f0f02eb9c0e640269adcae3da00cbe602772266b03258721442dd7327eb996fb2eef54b4fbe77c9b57053dd3f5a0e412c05734ae17fa87154402c9737bfd800f44aa3df0ef32fe56092214868b87a0a60ac628f42d20e1dee3d479c192b74ceacbb7d571a93750132c536328b031a6a03518806a81c734f33fe971a22721c12f2f3cc60d7f9b3bc89403d7cfdb5d0895a0d130ed44f0def9f86a53d3e3720615cec6f6f0aedecd4fc0cb2649c766ca1a17a0d421bfc8d9f46e123e432b8582c49629a969547a8ef40b231659b8385c7c1b81a09a62e4ae73121a710ba5353172874f248df38f39ceaef351522c4a9b1cffb1c3a09f4604347f9ba2c30703cce323c9f9705e0edecf5c1061e634a792de9a854e00a015421788d874414ca073e71d99c5fab4acd350b46551a48aa29891d322651071a0a1f624aded3a70996b4117dc609e5fbdd1bbdc819935be31a395904a1f85982aa0a69eb11de6f2d70d0ab095da5ba88f38cd9a60569839ecf35103360603d9aa2da02564a45d7661a773b13f984a47c63017fcea8599b39f42df99d1132d9cf2c159a0ff8b9f7b23ffe706af9188e74da6ad7ead36ba7d75c47ef915541689cc025194a094974e354978838330aeefefe0e29fa2e86cab1f4503b1b895f889514f48aa0e80b901f2f901ef20b901ebf901e80183112449b9010000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000820000000000000020000000000000000000800000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000200000000020000000000000000000000000000080000000000000800000000000000000000000f8def8dc94ad52e0f67b6f44cd5b9a6f4fbc7c0f78f37e094bf863a06775ce244ff81f0a82f87d6fd2cf885affb38416e3a04355f713c6f008dd126aa00000000000000000000000000000000000000000000000000000000000000006a00000000000000000000000000000000000000000000000000000000000000000b86000000000000000000000000074241db5f3ebaeecf9506e4ae9881860933416048eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48000000000000000000000000000000000000000000000000002386f26fc10000".from_hex().unwrap(),
				header_hash: H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))
			};

//			let proof: Proof = rlp::decode(&proof_record.proof).unwrap();

			let mixh = H256::from(hex!("1e2fc5a540b8f1cdaf50de52c388b1f53856cc61eb3ad20d91b9fcc2de3e3e2a"));
			let nonce = H64::from(hex!("339140bca72c49cd"));

			let header = EthHeader {
				parent_hash: H256::from(hex!("91553997d11a1d978f2ea363f230f5f525aee914a726d01e1deb4ea51de315cd")),
				timestamp: 1573560715,
				number: 6760579,
				author: EthAddress::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
				transactions_root: H256::from(hex!("c2b9e612bdac9d73d53ab38cafa959e5703dc078a9d5b184c65ee38bc471b5bf")),
				uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
				extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
				state_root: H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863")),
				receipts_root: H256::from(hex!("4c573edd96da310fefc3ced2d70831173e4684728c963330d990cf360aed8550")),
				log_bloom: Bloom::from_str("040000411080018200400100100020100808080020130000004000000a80040000001000000400004010800004811000000000800604002004000000002300820008181000000a820142010c0000010418030040080010080010280018200408000020800208120100000000001828000000000200000800000080511508c0008004100482000800040080000411409000000d20400000056000000802400006420002801000108140202100000804109008000150800140000020290028404000040102800000002000020000811004020080008000100411300100422420060210100100110124080000800084022021000200808005500000000000012000").unwrap(),
				gas_used: 0x220d13.into(),
				gas_limit: 0x7a121d.into(),
				difficulty: 0x269921540_u64.into(),
				seal: vec![rlp::encode(&mixh), rlp::encode(&nonce)],
				hash: Some(H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))),
			};

			assert_ok!(EthRelay::init_genesis_header(&header, 0x624c22d93f8e59_u64));

			assert_eq!(EthRelay::verify_receipt(&proof_record), Ok(receipt));
		});
}

#[test]
fn relay_header() {
	new_test_ext().execute_with(|| {
		// 6760579
		let mixh1 = H256::from(hex!("1e2fc5a540b8f1cdaf50de52c388b1f53856cc61eb3ad20d91b9fcc2de3e3e2a"));
		let nonce1 = H64::from(hex!("339140bca72c49cd"));

		let header1 = EthHeader {
			parent_hash: H256::from(hex!("91553997d11a1d978f2ea363f230f5f525aee914a726d01e1deb4ea51de315cd")),
			timestamp: 1573560715,
			number: 6760579,
			author: EthAddress::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
			transactions_root: H256::from(hex!("c2b9e612bdac9d73d53ab38cafa959e5703dc078a9d5b184c65ee38bc471b5bf")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
			state_root: H256::from(hex!("a21cdf375ebef58f606c298d6211f4edee58f2dd6430edbdd0ed3cd886a16863")),
			receipts_root: H256::from(hex!("4c573edd96da310fefc3ced2d70831173e4684728c963330d990cf360aed8550")),
			log_bloom: Bloom::from_str("040000411080018200400100100020100808080020130000004000000a80040000001000000400004010800004811000000000800604002004000000002300820008181000000a820142010c0000010418030040080010080010280018200408000020800208120100000000001828000000000200000800000080511508c0008004100482000800040080000411409000000d20400000056000000802400006420002801000108140202100000804109008000150800140000020290028404000040102800000002000020000811004020080008000100411300100422420060210100100110124080000800084022021000200808005500000000000012000").unwrap(),
			gas_used: 0x220d13.into(),
			gas_limit: 0x7a121d.into(),
			difficulty: 0x269921540_u64.into(),
			seal: vec![rlp::encode(&mixh1), rlp::encode(&nonce1)],
			hash: Some(H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9"))),
		};

		// #6890091
		// https://api-ropsten.etherscan.io/api?module=proxy&action=eth_getBlockByNumber&tag=0x69226b&boolean=true&apikey=YourApiKeyToken
		// https://jsoneditoronline.org/

		// 6760580
		let mixh2 = H256::from(hex!("e06f0c107dcc91e9e82de0b42d0e22d5c2cfae5209422fda88cff4f810f4bffb"));
		let nonce2 = H64::from(hex!("9348d06003756cff"));

		let header2 = EthHeader {
			parent_hash: H256::from(hex!("f1a5bc27877e219b859b0bb1f2f440134553019f9bb5a2eca7a4703263e736c9")),
			timestamp: 0x5dcaa1a3,
			number: 6760580,
			author: EthAddress::from(hex!("4ccfb3039b78d3938588157564c9ad559bafab94")),
			transactions_root: H256::from(hex!("bd4f8075fcdf01d3be2b8ae4a0a7195107429f34361e278e8760cc0f08e35d7a")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "d983010906846765746889676f312e31312e3133856c696e7578".from_hex().unwrap(),
			state_root: H256::from(hex!("694af9f7dc9866ec99dd83ef846778552cb60659e9cbd6e77e800816da83c3c9")),
			receipts_root: H256::from(hex!("729394331d204a175e4c1938ae19cc905107d8fd5562ee5283c323cde6b82e23")),
			log_bloom: Bloom::from_str("0400000000000100001000100000040000000100000000000000000002040080002004000000000200000000000210000080000002000080000000040014000000000000040020000000000800020040080110000004008800000000000000000100000002000000000000000000080040000000000004000010801101000000000000000000000000000000020060000000001000020000200002000000100000000000000000001000010000000000000001000080000000011000002040401000001280000000000021000800000800000000000010000000000040006000000400200000000000000000000000000000000000c000100000400000800100").unwrap(),
			gas_used: 0x17231e.into(),
			gas_limit: 0x7a1200.into(),
			difficulty: 0x2694562fe_u64.into(),
			seal: vec![rlp::encode(&mixh2), rlp::encode(&nonce2)],
			hash: Some(H256::from(hex!("12734378d3e4ad7050f7baf629d6eda161e911865d77c10e44c1f7e8e31fd7a7"))),
		};


		assert_ok!(EthRelay::init_genesis_header(&header1, 0x624c22d93f8e59_u64));

//		let light_dag2 = DAG::new(header2.number().into());
//		let partial_header_hash2 = header2.bare_hash();
//
//		println!("partial_header_hash2: {:?}", partial_header_hash2);
//
//		let mixhash2 = light_dag2
//			.hashimoto(partial_header_hash2, nonce2)
//			.0;
//		assert_eq!(
//			mixhash2,
//			mixh2
//		);

		assert_ok!(EthRelay::verify_header(&header2));

		assert_ok!(EthRelay::maybe_store_header(&header2));


		// 6760581
		let mixh3 = H256::from(hex!("019b6a52120a8769d34fe6348bdfa400ab4886576287f5ef11d9105875280c7e"));
		let nonce3 = H64::from(hex!("f43d6b58a23b7065"));

		let header3 = EthHeader {
			parent_hash: H256::from(hex!("12734378d3e4ad7050f7baf629d6eda161e911865d77c10e44c1f7e8e31fd7a7")),
			timestamp: 0x5dcaa1ae,
			number: 6760581,
			author: EthAddress::from(hex!("d7a15baeb7ea05c9660cbe03fb7999c2c2e57625")),
			transactions_root: H256::from(hex!("aaccb1d4b2dc847eefa50681d3096522a41f7c27031ead7a0ad51b50632218dc")),
			uncles_hash: H256::from(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347")),
			extra_data: "41746c616e7469632043727970746f".from_hex().unwrap(),
			state_root: H256::from(hex!("8106951604cc1305eedb3b7df1c2cf9c2d0ba9e792f645386d3a2fdffd2e9d96")),
			receipts_root: H256::from(hex!("e39a6c035914d6544db6d3653101740625e7608c747ea87b9784261e5d94a7ea")),
			log_bloom: Bloom::from_str("00000000000001000000000000000000000000000000000000000000000000000000000000000020000000000000000000200020400000000000000000000000000000000000000000000008000000000000080000000000000000000200000000000000000000000000000000008100000000000000000000000010010000000000020000000000000000000000040000000010040000002000204000000000000000000000000000000100000000000000000000000050002000000000000000800002800000000400000000000000000040000000100000000200000000080000000400002000000000000000000000002000000000000000000002020000").unwrap(),
			gas_used: 0x3ea15.into(),
			gas_limit: 0x7a121d.into(),
			difficulty: 0x26945e2fe_u64.into(),
			seal: vec![rlp::encode(&mixh3), rlp::encode(&nonce3)],
			hash: Some(H256::from(hex!("c86b090d12fa61c34f075530618e40a89654d8d85ac6aaa26149fb56b596a15a"))),
		};

		assert_ok!(EthRelay::verify_header(&header3));

		assert_ok!(EthRelay::maybe_store_header(&header3));
	});
}

#[test]
fn build_genesis_header() {
	let genesis_header = EthHeader {
			parent_hash: hex!("6e1494ad8e02a126fb86cf82b13627c45e6e8f1eab05e14270a69a05d7fe5e26").into(),
			timestamp: 0x5e78a9f5,
			number: 0x946eef,
			author: hex!("5a0b54d5dc17e0aadc383d2db43b0a0d3e029c4c").into(),
			transactions_root: hex!("661aff2eca6c57b3cb0f0b55d44761d4269c6548ed12bdebaa37df36d2e5b758").into(),
			uncles_hash: hex!("8652a5fec23f8bd33f9898c05ccd953c2c1faeb0d3de918c967a24b709254814").into(),
			extra_data: "737061726b706f6f6c2d6574682d636e2d687a33".from_hex().unwrap(),
			state_root: hex!("53fceb8f891e321c5124414a9bfd97ee39abb975066936bd220597a578c7655a").into(),
			receipts_root: hex!("4b57744cd97d237a8751a96317aecbe7db52f302ded36246d41782face81c17c").into(),
			log_bloom: Bloom::from_str("7890cc80915ca44051c6e0c101505084edc980e151012010b46c00623e4020a83a581359d08b095ead0116a408da0b9c3782605088210826133440ea6824981c78250060f64aa3a6c890102800c235e7204164252648a4e5a240e6e72068000030320104045c412de0ae448a126c10400e244864500b2c249e00aeb061143064b7b810d4e601a018542542c095880c521b89853b45840018616b0816ce90f2c01a642124b20c3d008cfbe08702607ba4f268200c294e1c2002b3280f3aae9312119421e2570840bb40233131064b408d3a003378994005c1090a8073c1501493b053ecc480ca50185e8105d240762a670ca43a6036408ab46204d21e0c923d1a").unwrap(),
			gas_used: 0x983707.into(),
			gas_limit: 0x9883be.into(),
			difficulty: 0x7db16f1a4402eu64.into(),
			seal: vec![
				rlp::encode(&H256::from(hex!("1cf81d78588bedf4ef8a0db007bba31b17c1086bead9b9badeca8d34b15db420"))), 
				rlp::encode(&H64::from(hex!("a98d18400422c64e"))),
			],
			hash: Some(hex!("034cd83d9150808de742592f57e302dc9eccc71af270639dc5f236e5bdd7d3e3").into()),
	};

	println!("{:?}", rlp::encode(&genesis_header));
}

/// # Check Receipt Safety
///
/// ## Family Tree
///
/// | pos     | height  | tx                                                                 |
/// |---------|---------|--------------------------------------------------------------------|
/// | origin  | 7575765 |                                                                    |
/// | grandpa | 7575766 | 0xc56be493f656f1c8222006eda5cd3392be5f0c096e8b7fb1c5542088c0f0c889 |
/// | uncle   | 7575766 |                                                                    |
/// | parent  | 7575767 |                                                                    |
/// | current | 7575768 | 0xfc836bf547f1e035e837bf0a8d26e432aa26da9659db5bf6ba69b0341d818778 |
///
/// To help reward miners for when duplicate block solutions are found
/// because of the shorter block times of Ethereum (compared to other cryptocurrency).
/// An uncle is a smaller reward than a full block.
///
/// ## Note:
///
/// check receipt should
/// - succeed when we relayed the correct header
/// - failed when canonical hash was re-orged by the block which contains our tx's brother block
#[test]
fn check_receipt_safety() {
	new_test_ext().execute_with(|| {
		assert_ok!(EthRelay::add_authority(RawOrigin::Root.into(), 0));
		assert_ok!(EthRelay::set_number_of_blocks_safe(RawOrigin::Root.into(), 0));

		// family tree
		let [o, g, u, _p, _c] = mock_canonical_relationship().unwrap();
		let [origin, grandpa, uncle] = [o.unwrap(), g.unwrap(), u.unwrap()];
		assert_ok!(EthRelay::init_genesis_header(&origin, 0x624c22d93f8e59_u64));

		let receipt = mock_canonical_receipt().unwrap();
		assert_ne!(grandpa.hash, uncle.hash);
		assert_eq!(grandpa.number, uncle.number);

		// check receipt should succeed when we relayed the correct header
		assert_ok!(EthRelay::relay_header(Origin::signed(0), grandpa.clone()));
		assert_ok!(EthRelay::check_receipt(Origin::signed(0), receipt.clone()));

		// check should fail when canonical hash was re-orged by
		// the block which contains our tx's brother block
		assert_ok!(EthRelay::relay_header(Origin::signed(0), uncle));
		assert_err!(
			EthRelay::check_receipt(Origin::signed(0), receipt.clone()),
			<Error<Test>>::HeaderNC
		);
	});
}

#[test]
fn canonical_reorg_uncle_should_succeed() {
	new_test_ext().execute_with(|| {
		assert_ok!(EthRelay::add_authority(RawOrigin::Root.into(), 0));
		assert_ok!(EthRelay::set_number_of_blocks_safe(RawOrigin::Root.into(), 0));

		let [o, g, u, _p, _c] = mock_canonical_relationship().unwrap();
		let [origin, grandpa, uncle] = [o.unwrap(), g.unwrap(), u.unwrap()];
		assert_ok!(EthRelay::init_genesis_header(&origin, 0x624c22d93f8e59_u64));

		// check relationship
		assert_ne!(grandpa.hash, uncle.hash);
		assert_eq!(grandpa.number, uncle.number);

		let (gh, uh) = (grandpa.hash, uncle.hash);
		let number = grandpa.number;

		// relay uncle header
		assert_ok!(EthRelay::relay_header(Origin::signed(0), uncle));
		assert_eq!(EthRelay::canonical_header_hash_of(number), uh.unwrap());

		// relay grandpa and re-org uncle
		assert_ok!(EthRelay::relay_header(Origin::signed(0), grandpa));
		assert_eq!(EthRelay::canonical_header_hash_of(number), gh.unwrap());
	});
}

#[test]
fn test_safety_block() {
	new_test_ext().execute_with(|| {
		assert_ok!(EthRelay::add_authority(RawOrigin::Root.into(), 0));
		assert_ok!(EthRelay::set_number_of_blocks_safe(RawOrigin::Root.into(), 2));

		// family tree
		let [o, g, p, u, c] = mock_canonical_relationship().unwrap();
		let [origin, grandpa, parent, uncle, current] = [o.unwrap(), g.unwrap(), p.unwrap(), u.unwrap(), c.unwrap()];

		let receipt = mock_canonical_receipt().unwrap();

		// not safety after 0 block
		assert_ok!(EthRelay::init_genesis_header(&origin, 0x624c22d93f8e59_u64));
		assert_ok!(EthRelay::relay_header(Origin::signed(0), grandpa));
		assert_err!(
			EthRelay::check_receipt(Origin::signed(0), receipt.clone()),
			<Error<Test>>::HeaderNS
		);

		// not safety after 2 blocks
		assert_ok!(EthRelay::relay_header(Origin::signed(0), parent));
		assert_ok!(EthRelay::relay_header(Origin::signed(0), uncle));
		assert_err!(
			EthRelay::check_receipt(Origin::signed(0), receipt.clone()),
			<Error<Test>>::HeaderNS
		);

		// safety after 3 blocks
		assert_ok!(EthRelay::relay_header(Origin::signed(0), current));
		assert_ok!(EthRelay::check_receipt(Origin::signed(0), receipt));
	});
}
