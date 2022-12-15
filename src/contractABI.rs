pub fn abi() -> String {
    r#"[
        {
          "inputs": [
            {
              "internalType": "address",
              "name": "_positionManager",
              "type": "address"
            }
          ],
          "stateMutability": "nonpayable",
          "type": "constructor"
        },
        {
          "inputs": [
            {
              "internalType": "uint256",
              "name": "TokenId",
              "type": "uint256"
            }
          ],
          "name": "Position",
          "outputs": [
            {
              "internalType": "int24",
              "name": "",
              "type": "int24"
            },
            {
              "internalType": "uint160",
              "name": "",
              "type": "uint160"
            },
            {
              "internalType": "uint128",
              "name": "",
              "type": "uint128"
            },
            {
              "internalType": "int24",
              "name": "",
              "type": "int24"
            },
            {
              "internalType": "int24",
              "name": "",
              "type": "int24"
            },
            {
              "internalType": "address",
              "name": "",
              "type": "address"
            },
            {
              "internalType": "address",
              "name": "",
              "type": "address"
            }
          ],
          "stateMutability": "view",
          "type": "function"
        },
        {
          "inputs": [],
          "name": "positionManager",
          "outputs": [
            {
              "internalType": "contract INonfungiblePositionManager",
              "name": "",
              "type": "address"
            }
          ],
          "stateMutability": "view",
          "type": "function"
        },
        {
          "inputs": [
            {
              "internalType": "uint256",
              "name": "tokenId",
              "type": "uint256"
            },
            {
              "internalType": "uint128",
              "name": "liquidity",
              "type": "uint128"
            },
            {
              "internalType": "uint256",
              "name": "amount0Min",
              "type": "uint256"
            },
            {
              "internalType": "uint256",
              "name": "amount1Min",
              "type": "uint256"
            },
            {
              "internalType": "uint256",
              "name": "deadline",
              "type": "uint256"
            }
          ],
          "name": "wid",
          "outputs": [],
          "stateMutability": "nonpayable",
          "type": "function"
        }
      ]"#
    .to_string()
}
