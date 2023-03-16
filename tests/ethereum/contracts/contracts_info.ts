export const incrementerInfo = {
	bytecode:
		"608060405234801561001057600080fd5b5060405161018c38038061018c8339818101604052602081101561003357600080fd5b810190808051906020019092919050505080600081905550506101318061005b6000396000f3fe6080604052348015600f57600080fd5b5060043610603c5760003560e01c80637cf5dab01460415780638381f58a14606c578063d826f88f146088575b600080fd5b606a60048036036020811015605557600080fd5b81019080803590602001909291905050506090565b005b607260ec565b6040518082815260200191505060405180910390f35b608e60f2565b005b80600054016000819055503373ffffffffffffffffffffffffffffffffffffffff167fb182275171042022ff972a26edbd0171bccc74463bd22e56dbbeba4e93b7a668826040518082815260200191505060405180910390a250565b60005481565b6000808190555056fea2646970667358221220e20e87bcf3085e9302330bfcfc42af9eeff3a82cf3d5bb636c607ae9363c595a64736f6c634300060c0033",
	opcodes:
		"PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x40 MLOAD PUSH2 0x18C CODESIZE SUB DUP1 PUSH2 0x18C DUP4 CODECOPY DUP2 DUP2 ADD PUSH1 0x40 MSTORE PUSH1 0x20 DUP2 LT ISZERO PUSH2 0x33 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 MLOAD SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP DUP1 PUSH1 0x0 DUP2 SWAP1 SSTORE POP POP PUSH2 0x131 DUP1 PUSH2 0x5B PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH1 0xF JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH1 0x3C JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0x7CF5DAB0 EQ PUSH1 0x41 JUMPI DUP1 PUSH4 0x8381F58A EQ PUSH1 0x6C JUMPI DUP1 PUSH4 0xD826F88F EQ PUSH1 0x88 JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH1 0x6A PUSH1 0x4 DUP1 CALLDATASIZE SUB PUSH1 0x20 DUP2 LT ISZERO PUSH1 0x55 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 CALLDATALOAD SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP PUSH1 0x90 JUMP JUMPDEST STOP JUMPDEST PUSH1 0x72 PUSH1 0xEC JUMP JUMPDEST PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 RETURN JUMPDEST PUSH1 0x8E PUSH1 0xF2 JUMP JUMPDEST STOP JUMPDEST DUP1 PUSH1 0x0 SLOAD ADD PUSH1 0x0 DUP2 SWAP1 SSTORE POP CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH32 0xB182275171042022FF972A26EDBD0171BCCC74463BD22E56DBBEBA4E93B7A668 DUP3 PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG2 POP JUMP JUMPDEST PUSH1 0x0 SLOAD DUP2 JUMP JUMPDEST PUSH1 0x0 DUP1 DUP2 SWAP1 SSTORE POP JUMP INVALID LOG2 PUSH5 0x6970667358 0x22 SLT KECCAK256 0xE2 0xE DUP8 0xBC RETURN ADDMOD 0x5E SWAP4 MUL CALLER SIGNEXTEND 0xFC 0xFC TIMESTAMP 0xAF SWAP15 0xEF RETURN 0xA8 0x2C RETURN 0xD5 0xBB PUSH4 0x6C607AE9 CALLDATASIZE EXTCODECOPY MSIZE GAS PUSH5 0x736F6C6343 STOP MOD 0xC STOP CALLER ",
	abi: [
		{
			inputs: [
				{
					internalType: "uint256",
					name: "_initialNumber",
					type: "uint256",
				},
			],
			stateMutability: "nonpayable",
			type: "constructor",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: true,
					internalType: "address",
					name: "sender",
					type: "address",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Increment",
			type: "event",
		},
		{
			inputs: [
				{
					internalType: "uint256",
					name: "_value",
					type: "uint256",
				},
			],
			name: "increment",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "number",
			outputs: [
				{
					internalType: "uint256",
					name: "",
					type: "uint256",
				},
			],
			stateMutability: "view",
			type: "function",
		},
		{
			inputs: [],
			name: "reset",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
	],
};

export const opcodesInfo = {
	bytecode:
		"608060405234801561001057600080fd5b5060405161001d9061007e565b604051809103906000f080158015610039573d6000803e3d6000fd5b506000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555061008b565b6101438061052283390190565b6104888061009a6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806355313dea146100515780636d3d14161461005b578063b9d1e5aa14610065578063f8a8fd6d1461006f575b600080fd5b610059610079565b005b61006361007b565b005b61006d610080565b005b610077610082565b005b005b600080fd5bfe5b600160021a6002f35b600581101561009f5760018101905061008b565b5060065b60058111156100b7576001810190506100a3565b5060015b60058112156100cf576001810190506100bb565b5060065b60058113156100e7576001810190506100d3565b506002156100f457600051505b60405160208101602060048337505060405160208101602060048339505060405160208101602060048360003c50503660005b8181101561013e5760028152600181019050610127565b505060008020506000602060403e6010608060106040610123612710fa506020610123600af05060008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050600060405180807f697353616d654164647265737328616464726573732c61646472657373290000815250601e01905060405180910390209050600033905060405182815281600482015281602482015260648101604052602081604483600088611388f1505060405182815281600482015281602482015260648101604052602081604483600088611388f250506040518281528160048201528160248201526064810160405260208160448387611388f4505060006242004290507f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b6040518082815260200191505060405180910390a07f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b6040518082815260200191505060405180910390a13373ffffffffffffffffffffffffffffffffffffffff1660001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b6040518082815260200191505060405180910390a28060001b3373ffffffffffffffffffffffffffffffffffffffff1660001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b6040518082815260200191505060405180910390a38060001b8160001b3373ffffffffffffffffffffffffffffffffffffffff1660001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b7f50cb9fe53daa9737b786ab3646f04d0150dc50ef4e75f59509d83667ad5adb2060001b6040518082815260200191505060405180910390a46002fffea265627a7a72315820da4feb2af5051e773c61e531dc7c451208bd898210e40f606667d91689c23c7c64736f6c63430005110032608060405234801561001057600080fd5b50610123806100206000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063161e715014602d575b600080fd5b608c60048036036040811015604157600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff16906020019092919050505060a6565b604051808215151515815260200191505060405180910390f35b60008173ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff16141560e3576001905060e8565b600090505b9291505056fea265627a7a7231582082d761506d74e3b44f3c332693f36afc64d261352ea6bd6c457883eea792919064736f6c63430005110032",
	opcodes:
		"PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x40 MLOAD PUSH2 0x1D SWAP1 PUSH2 0x7E JUMP JUMPDEST PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 PUSH1 0x0 CREATE DUP1 ISZERO DUP1 ISZERO PUSH2 0x39 JUMPI RETURNDATASIZE PUSH1 0x0 DUP1 RETURNDATACOPY RETURNDATASIZE PUSH1 0x0 REVERT JUMPDEST POP PUSH1 0x0 DUP1 PUSH2 0x100 EXP DUP2 SLOAD DUP2 PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF MUL NOT AND SWAP1 DUP4 PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND MUL OR SWAP1 SSTORE POP PUSH2 0x8B JUMP JUMPDEST PUSH2 0x143 DUP1 PUSH2 0x522 DUP4 CODECOPY ADD SWAP1 JUMP JUMPDEST PUSH2 0x488 DUP1 PUSH2 0x9A PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH2 0x4C JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0x55313DEA EQ PUSH2 0x51 JUMPI DUP1 PUSH4 0x6D3D1416 EQ PUSH2 0x5B JUMPI DUP1 PUSH4 0xB9D1E5AA EQ PUSH2 0x65 JUMPI DUP1 PUSH4 0xF8A8FD6D EQ PUSH2 0x6F JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH2 0x59 PUSH2 0x79 JUMP JUMPDEST STOP JUMPDEST PUSH2 0x63 PUSH2 0x7B JUMP JUMPDEST STOP JUMPDEST PUSH2 0x6D PUSH2 0x80 JUMP JUMPDEST STOP JUMPDEST PUSH2 0x77 PUSH2 0x82 JUMP JUMPDEST STOP JUMPDEST STOP JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST INVALID JUMPDEST PUSH1 0x1 PUSH1 0x2 BYTE PUSH1 0x2 RETURN JUMPDEST PUSH1 0x5 DUP2 LT ISZERO PUSH2 0x9F JUMPI PUSH1 0x1 DUP2 ADD SWAP1 POP PUSH2 0x8B JUMP JUMPDEST POP PUSH1 0x6 JUMPDEST PUSH1 0x5 DUP2 GT ISZERO PUSH2 0xB7 JUMPI PUSH1 0x1 DUP2 ADD SWAP1 POP PUSH2 0xA3 JUMP JUMPDEST POP PUSH1 0x1 JUMPDEST PUSH1 0x5 DUP2 SLT ISZERO PUSH2 0xCF JUMPI PUSH1 0x1 DUP2 ADD SWAP1 POP PUSH2 0xBB JUMP JUMPDEST POP PUSH1 0x6 JUMPDEST PUSH1 0x5 DUP2 SGT ISZERO PUSH2 0xE7 JUMPI PUSH1 0x1 DUP2 ADD SWAP1 POP PUSH2 0xD3 JUMP JUMPDEST POP PUSH1 0x2 ISZERO PUSH2 0xF4 JUMPI PUSH1 0x0 MLOAD POP JUMPDEST PUSH1 0x40 MLOAD PUSH1 0x20 DUP2 ADD PUSH1 0x20 PUSH1 0x4 DUP4 CALLDATACOPY POP POP PUSH1 0x40 MLOAD PUSH1 0x20 DUP2 ADD PUSH1 0x20 PUSH1 0x4 DUP4 CODECOPY POP POP PUSH1 0x40 MLOAD PUSH1 0x20 DUP2 ADD PUSH1 0x20 PUSH1 0x4 DUP4 PUSH1 0x0 EXTCODECOPY POP POP CALLDATASIZE PUSH1 0x0 JUMPDEST DUP2 DUP2 LT ISZERO PUSH2 0x13E JUMPI PUSH1 0x2 DUP2 MSTORE PUSH1 0x1 DUP2 ADD SWAP1 POP PUSH2 0x127 JUMP JUMPDEST POP POP PUSH1 0x0 DUP1 KECCAK256 POP PUSH1 0x0 PUSH1 0x20 PUSH1 0x40 RETURNDATACOPY PUSH1 0x10 PUSH1 0x80 PUSH1 0x10 PUSH1 0x40 PUSH2 0x123 PUSH2 0x2710 STATICCALL POP PUSH1 0x20 PUSH2 0x123 PUSH1 0xA CREATE POP PUSH1 0x0 DUP1 PUSH1 0x0 SWAP1 SLOAD SWAP1 PUSH2 0x100 EXP SWAP1 DIV PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND SWAP1 POP PUSH1 0x0 PUSH1 0x40 MLOAD DUP1 DUP1 PUSH32 0x697353616D654164647265737328616464726573732C61646472657373290000 DUP2 MSTORE POP PUSH1 0x1E ADD SWAP1 POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 KECCAK256 SWAP1 POP PUSH1 0x0 CALLER SWAP1 POP PUSH1 0x40 MLOAD DUP3 DUP2 MSTORE DUP2 PUSH1 0x4 DUP3 ADD MSTORE DUP2 PUSH1 0x24 DUP3 ADD MSTORE PUSH1 0x64 DUP2 ADD PUSH1 0x40 MSTORE PUSH1 0x20 DUP2 PUSH1 0x44 DUP4 PUSH1 0x0 DUP9 PUSH2 0x1388 CALL POP POP PUSH1 0x40 MLOAD DUP3 DUP2 MSTORE DUP2 PUSH1 0x4 DUP3 ADD MSTORE DUP2 PUSH1 0x24 DUP3 ADD MSTORE PUSH1 0x64 DUP2 ADD PUSH1 0x40 MSTORE PUSH1 0x20 DUP2 PUSH1 0x44 DUP4 PUSH1 0x0 DUP9 PUSH2 0x1388 CALLCODE POP POP PUSH1 0x40 MLOAD DUP3 DUP2 MSTORE DUP2 PUSH1 0x4 DUP3 ADD MSTORE DUP2 PUSH1 0x24 DUP3 ADD MSTORE PUSH1 0x64 DUP2 ADD PUSH1 0x40 MSTORE PUSH1 0x20 DUP2 PUSH1 0x44 DUP4 DUP8 PUSH2 0x1388 DELEGATECALL POP POP PUSH1 0x0 PUSH3 0x420042 SWAP1 POP PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG0 PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG1 CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG2 DUP1 PUSH1 0x0 SHL CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG3 DUP1 PUSH1 0x0 SHL DUP2 PUSH1 0x0 SHL CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH32 0x50CB9FE53DAA9737B786AB3646F04D0150DC50EF4E75F59509D83667AD5ADB20 PUSH1 0x0 SHL PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG4 PUSH1 0x2 SELFDESTRUCT INVALID LOG2 PUSH6 0x627A7A723158 KECCAK256 0xDA 0x4F 0xEB 0x2A CREATE2 SDIV 0x1E PUSH24 0x3C61E531DC7C451208BD898210E40F606667D91689C23C7C PUSH5 0x736F6C6343 STOP SDIV GT STOP ORIGIN PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH2 0x123 DUP1 PUSH2 0x20 PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH1 0xF JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH1 0x28 JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0x161E7150 EQ PUSH1 0x2D JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH1 0x8C PUSH1 0x4 DUP1 CALLDATASIZE SUB PUSH1 0x40 DUP2 LT ISZERO PUSH1 0x41 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST DUP2 ADD SWAP1 DUP1 DUP1 CALLDATALOAD PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 DUP1 CALLDATALOAD PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND SWAP1 PUSH1 0x20 ADD SWAP1 SWAP3 SWAP2 SWAP1 POP POP POP PUSH1 0xA6 JUMP JUMPDEST PUSH1 0x40 MLOAD DUP1 DUP3 ISZERO ISZERO ISZERO ISZERO DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 RETURN JUMPDEST PUSH1 0x0 DUP2 PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND DUP4 PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND EQ ISZERO PUSH1 0xE3 JUMPI PUSH1 0x1 SWAP1 POP PUSH1 0xE8 JUMP JUMPDEST PUSH1 0x0 SWAP1 POP JUMPDEST SWAP3 SWAP2 POP POP JUMP INVALID LOG2 PUSH6 0x627A7A723158 KECCAK256 DUP3 0xD7 PUSH2 0x506D PUSH21 0xE3B44F3C332693F36AFC64D261352EA6BD6C457883 0xEE 0xA7 SWAP3 SWAP2 SWAP1 PUSH5 0x736F6C6343 STOP SDIV GT STOP ORIGIN ",
	abi: [
		{
			inputs: [],
			payable: false,
			stateMutability: "nonpayable",
			type: "constructor",
		},
		{
			constant: false,
			inputs: [],
			name: "test",
			outputs: [],
			payable: false,
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			constant: true,
			inputs: [],
			name: "test_invalid",
			outputs: [],
			payable: false,
			stateMutability: "view",
			type: "function",
		},
		{
			constant: true,
			inputs: [],
			name: "test_revert",
			outputs: [],
			payable: false,
			stateMutability: "view",
			type: "function",
		},
		{
			constant: true,
			inputs: [],
			name: "test_stop",
			outputs: [],
			payable: false,
			stateMutability: "view",
			type: "function",
		},
	],
};

export const eventInfo = {
	bytecode:
		"608060405234801561001057600080fd5b5061031b806100206000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c8063a67808571161005b578063a6780857146100b5578063b61c0503146100bf578063e8beef5b146100c9578063f38b0600146100d357610088565b8063102accc11461008d5780634e7ad3671461009757806365538c73146100a157806376bc21d9146100ab575b600080fd5b6100956100dd565b005b61009f610132565b005b6100a961014f565b005b6100b3610189565b005b6100bd6101bd565b005b6100c76101d6565b005b6100d1610214565b005b6100db61026c565b005b3373ffffffffffffffffffffffffffffffffffffffff16600115157f0e216b62efbb97e751a2ce09f607048751720397ecfb9eef1e48a6644948985b602a6040518082815260200191505060405180910390a3565b60011515602a6040518082815260200191505060405180910390a1565b7f65c9ac8011e286e89d02a269890f41d67ca2cc597b2c76c7c69321ff492be580602a6040518082815260200191505060405180910390a1565b3373ffffffffffffffffffffffffffffffffffffffff1660011515602a6040518082815260200191505060405180910390a2565b602a6040518082815260200191505060405180910390a0565b600115157f81933b308056e7e85668661dcd102b1f22795b4431f9cf4625794f381c271c6b602a6040518082815260200191505060405180910390a2565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60001b3373ffffffffffffffffffffffffffffffffffffffff1660011515602a6040518082815260200191505060405180910390a3565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60001b3373ffffffffffffffffffffffffffffffffffffffff16600115157f317b31292193c2a4f561cc40a95ea0d97a2733f14af6d6d59522473e1f3ae65f602a6040518082815260200191505060405180910390a456fea2646970667358221220153d53b462c5c7cf5d26f62987030b4c78e85c6c87a3b3e742769581438308c864736f6c634300060c0033",
	opcode: "PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH2 0x31B DUP1 PUSH2 0x20 PUSH1 0x0 CODECOPY PUSH1 0x0 RETURN INVALID PUSH1 0x80 PUSH1 0x40 MSTORE CALLVALUE DUP1 ISZERO PUSH2 0x10 JUMPI PUSH1 0x0 DUP1 REVERT JUMPDEST POP PUSH1 0x4 CALLDATASIZE LT PUSH2 0x88 JUMPI PUSH1 0x0 CALLDATALOAD PUSH1 0xE0 SHR DUP1 PUSH4 0xA6780857 GT PUSH2 0x5B JUMPI DUP1 PUSH4 0xA6780857 EQ PUSH2 0xB5 JUMPI DUP1 PUSH4 0xB61C0503 EQ PUSH2 0xBF JUMPI DUP1 PUSH4 0xE8BEEF5B EQ PUSH2 0xC9 JUMPI DUP1 PUSH4 0xF38B0600 EQ PUSH2 0xD3 JUMPI PUSH2 0x88 JUMP JUMPDEST DUP1 PUSH4 0x102ACCC1 EQ PUSH2 0x8D JUMPI DUP1 PUSH4 0x4E7AD367 EQ PUSH2 0x97 JUMPI DUP1 PUSH4 0x65538C73 EQ PUSH2 0xA1 JUMPI DUP1 PUSH4 0x76BC21D9 EQ PUSH2 0xAB JUMPI JUMPDEST PUSH1 0x0 DUP1 REVERT JUMPDEST PUSH2 0x95 PUSH2 0xDD JUMP JUMPDEST STOP JUMPDEST PUSH2 0x9F PUSH2 0x132 JUMP JUMPDEST STOP JUMPDEST PUSH2 0xA9 PUSH2 0x14F JUMP JUMPDEST STOP JUMPDEST PUSH2 0xB3 PUSH2 0x189 JUMP JUMPDEST STOP JUMPDEST PUSH2 0xBD PUSH2 0x1BD JUMP JUMPDEST STOP JUMPDEST PUSH2 0xC7 PUSH2 0x1D6 JUMP JUMPDEST STOP JUMPDEST PUSH2 0xD1 PUSH2 0x214 JUMP JUMPDEST STOP JUMPDEST PUSH2 0xDB PUSH2 0x26C JUMP JUMPDEST STOP JUMPDEST CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x1 ISZERO ISZERO PUSH32 0xE216B62EFBB97E751A2CE09F607048751720397ECFB9EEF1E48A6644948985B PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG3 JUMP JUMPDEST PUSH1 0x1 ISZERO ISZERO PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG1 JUMP JUMPDEST PUSH32 0x65C9AC8011E286E89D02A269890F41D67CA2CC597B2C76C7C69321FF492BE580 PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG1 JUMP JUMPDEST CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x1 ISZERO ISZERO PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG2 JUMP JUMPDEST PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG0 JUMP JUMPDEST PUSH1 0x1 ISZERO ISZERO PUSH32 0x81933B308056E7E85668661DCD102B1F22795B4431F9CF4625794F381C271C6B PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG2 JUMP JUMPDEST PUSH32 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF PUSH1 0x0 SHL CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x1 ISZERO ISZERO PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG3 JUMP JUMPDEST PUSH32 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF PUSH1 0x0 SHL CALLER PUSH20 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF AND PUSH1 0x1 ISZERO ISZERO PUSH32 0x317B31292193C2A4F561CC40A95EA0D97A2733F14AF6D6D59522473E1F3AE65F PUSH1 0x2A PUSH1 0x40 MLOAD DUP1 DUP3 DUP2 MSTORE PUSH1 0x20 ADD SWAP2 POP POP PUSH1 0x40 MLOAD DUP1 SWAP2 SUB SWAP1 LOG4 JUMP INVALID LOG2 PUSH5 0x6970667358 0x22 SLT KECCAK256 ISZERO RETURNDATASIZE MSTORE8 0xB4 PUSH3 0xC5C7CF 0x5D 0x26 0xF6 0x29 DUP8 SUB SIGNEXTEND 0x4C PUSH25 0xE85C6C87A3B3E742769581438308C864736F6C634300060C00 CALLER ",
	abi: [
		{
			inputs: [],
			stateMutability: "nonpayable",
			type: "constructor",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log0",
			type: "event",
		},
		{
			anonymous: true,
			inputs: [
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log0Anonym",
			type: "event",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log1",
			type: "event",
		},
		{
			anonymous: true,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log1Anonym",
			type: "event",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: true,
					internalType: "address",
					name: "aAddress",
					type: "address",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log2",
			type: "event",
		},
		{
			anonymous: true,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: true,
					internalType: "address",
					name: "aAddress",
					type: "address",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log2Anonym",
			type: "event",
		},
		{
			anonymous: false,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: true,
					internalType: "address",
					name: "aAddress",
					type: "address",
				},
				{
					indexed: true,
					internalType: "bytes32",
					name: "aBytes32",
					type: "bytes32",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log3",
			type: "event",
		},
		{
			anonymous: true,
			inputs: [
				{
					indexed: true,
					internalType: "bool",
					name: "aBool",
					type: "bool",
				},
				{
					indexed: true,
					internalType: "address",
					name: "aAddress",
					type: "address",
				},
				{
					indexed: true,
					internalType: "bytes32",
					name: "aBytes32",
					type: "bytes32",
				},
				{
					indexed: false,
					internalType: "uint256",
					name: "value",
					type: "uint256",
				},
			],
			name: "Log3Anonym",
			type: "event",
		},
		{
			inputs: [],
			name: "fireEventLog0",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog0Anonym",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog1",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog1Anonym",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog2",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog2Anonym",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog3",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
		{
			inputs: [],
			name: "fireEventLog3Anonym",
			outputs: [],
			stateMutability: "nonpayable",
			type: "function",
		},
	],
};

export const blsInfo = {
	abi: [
		{
			inputs: [
				{
					internalType: "bytes[]",
					name: "pubkeys",
					type: "bytes[]",
				},
				{
					internalType: "bytes",
					name: "message",
					type: "bytes",
				},
				{
					internalType: "bytes",
					name: "sig",
					type: "bytes",
				},
			],
			name: "fast_aggregate_verify",
			outputs: [
				{
					internalType: "bool",
					name: "",
					type: "bool",
				},
			],
			stateMutability: "nonpayable",
			type: "function",
		},
	],
};
