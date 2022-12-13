use ethers::providers::{Http, Provider};
use std::convert::TryFrom;
mod contractABI;
use ethers::{abi::Abi, contract::Contract, types::Address};
mod addresses;

#[tokio::main]
async fn main() {
    let provider_service = Provider::<Http>::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/XQaS9eHZxXvAmb-lzIWBfqSe1lx1Aims",
    )
    .expect("fail");
    let abi_original: String = contractABI::abi();
    let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
    let address: Address = (addresses::add()).iAdd;

    let contract = Contract::new(address, abi, provider_service);
    println!("connected contracts");

    let init_value: String = contract
        .method::<_, String>("name", ())
        .unwrap()
        .call()
        .await
        .expect("fail");
    print!("{init_value}")
}

fn cal(
    tick: u32,
    sqrtPriceX96: u128,
    liquidity: u128,
    tickLower: i32,
    tickUpper: i32,
    token0: Address,
    token1: Address,
) -> u128 {
    let tickk = tick as i32;
    let current_price = tick_to_price(tickk);

    let sa = tick_to_price(tickLower / 2);
    let sb = tick_to_price(tickUpper / 2);

    if tickUpper <= tickk {
        let amount0 = 0;
        let amount1 = liquidity * ((sb - sa) as u128);
        return amount1;
    } else if tickk <= tickLower {
        let amount0 = liquidity * ((sb - sa) as u128) / ((sa * sb) as u128);
        let amount1 = 0;
        return amount0;
    } else {
        let amount0 = liquidity * ((sb - tickk) as u128) / ((tickk * sb) as u128);
        let amount1 = liquidity * ((tickk - sa) as u128);
        let z = amount0 + amount1;
        return z;
    }
}

fn tick_to_price(tick: i32) -> i32 {
    let x = 1001 / 1000;
    let y = x * tick;
    return y;
}

// int24,
// uint160,
// uint128,
// int24,
// int24,
// address,
// address
