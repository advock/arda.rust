use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    types::U256,
};
use std::convert::TryFrom;
mod contractABI;
use ethers::{abi::Abi, contract::Contract, types::Address};
mod addresses;

#[tokio::main]
async fn main() {
    let provider_service = Provider::<Http>::try_from(
        "https://eth-goerli.g.alchemy.com/v2/kH74ovyVPFoFpi-ddofXYKhlVow_uy_S",
    )
    .expect("fail");
    let abi_original: String = contractABI::abi();
    let abi: Abi = serde_json::from_str(&abi_original).expect("fed");
    let address: Address = (addresses::add()).iAdd;

    let contract = Contract::new(address, abi, provider_service);
    println!("connected contracts");

    let id: u128 = 359947;

    let (tick, sqrtPriceX96, liquidity, tickLower, tickUpper, token0, token1) = contract
        .method::<_, (i128, i128, i128, i128, i128, Address, Address)>("Position", id)
        .unwrap()
        .call()
        .await
        .expect("fadbdbhbdjhbil");
}

// fn cal(
//     tick: i32,
//     sqrtPriceX96: u128,
//     liquidity: u128,
//     tickLower: i32,
//     tickUpper: i32,
//     token0: Address,
//     token1: Address,
// ) {
//     //let tickk = tick as i32;
//     let current_price = tick_to_price(tick);

//     let sa = tick_to_price(tickLower / 2);
//     let sb = tick_to_price(tickUpper / 2);

//     let final_amount = tick_cal(tick, tickLower, tickUpper, liquidity, sa, sb);

//     let mint_price = (Minted_Price(tickLower, tickUpper) as u128);

//     // if final_amount <= mint_price / 2 {
//     //     Widraw()
//     // }
// }

// fn Widraw() {
//     let provider_service = Provider::<Http>::try_from(
//         "https://eth-mainnet.g.alchemy.com/v2/XQaS9eHZxXvAmb-lzIWBfqSe1lx1Aims",
//     )
//     .expect("fail");
//     let abi_original: String = contractABI::abi();
//     let abi: Abi = serde_json::from_str(&abi_original).expect("failed");
//     let address: Address = (addresses::add()).iAdd;

//     let contract = Contract::new(address, abi, provider_service);
//     println!("connected contracts");

//     let init_value: String = contract
//         .method::<_, String>("name", ())
//         .unwrap()
//         .call()
//         .await
//         .expect("fd");
// }

// fn tick_to_price(tick: i32) -> i32 {
//     let x = 1001 / 1000;
//     let y = x * tick;
//     return y;
// }

// fn Minted_Price(tickLower: i32, tickUpper: i32) -> i32 {
//     let avg = (tickLower + tickUpper) / 2;
//     return avg;
// }

// fn tick_cal(tick: i32, tickLower: i32, tickUpper: i32, liquidity: u128, sa: i32, sb: i32) -> u128 {
//     if tickUpper <= tick {
//         let amount0 = 0;
//         let amount1 = liquidity * ((sb - sa) as u128);
//         return amount1;
//     } else if tick <= tickLower {
//         let amount0 = liquidity * ((sb - sa) as u128) / ((sa * sb) as u128);
//         let amount1 = 0;
//         return amount0;
//     } else {
//         let amount0 = liquidity * ((sb - tick) as u128) / ((tick * sb) as u128);
//         let amount1 = liquidity * ((tick - sa) as u128);
//         let z = amount0 + amount1;
//         return z;
//     }
// }

// int24,
// uint160,
// uint128,
// int24,
// int24,
// address,
// address
