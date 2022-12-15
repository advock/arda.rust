use ethers::prelude::account::TokenQueryOption;
use ethers::signers::LocalWallet;
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

    let tokenId = 359947;

    let (tick, sqrtPriceX96, liquidity, tickLower, tickUpper, token0, token1): (
        u128,
        u128,
        u128,
        u128,
        u128,
        Address,
        Address,
    ) = contract
        .method::<_, _>("Position", tokenId as u128)
        .unwrap()
        .call()
        .await
        .expect("fail");

    if cal(
        tokenId,
        tick,
        sqrtPriceX96,
        liquidity,
        tickLower,
        tickUpper,
        token0,
        token1,
    ) {
        let provider_service = Provider::<Http>::try_from(
            "https://eth-mainnet.g.alchemy.com/v2/XQaS9eHZxXvAmb-lzIWBfqSe1lx1Aims",
        )
        .expect("fail");

        let private_KEY = "dbdhbfhbdfhbdhfbchbcjdshbdhbdhbhdbhjb";
        let wallet: LocalWallet = private_KEY.parse().expect("fail");

        let abi_original: String = contractABI::abi();
        let abi: Abi = serde_json::from_str(&abi_original).expect("fed");
        let address: Address = (addresses::add()).iAdd;
        let provider = SignerMiddleware::new(provider_service, wallet);

        let contract = Contract::new(address, abi, provider);
        println!("connected contracts");

        let call: bool = contract
            .method::<_, _>("wid", tokenId as u128)
            .unwrap()
            .call()
            .await
            .expect("fail");
    }
}

fn cal(
    tokenID: i32,
    tick: u128,
    sqrtPriceX96: u128,
    liquidity: u128,
    tickLower: u128,
    tickUpper: u128,
    token0: Address,
    token1: Address,
) -> bool {
    //let tickk = tick as i32;
    let current_price = tick_to_price(tick);

    let sa = tick_to_price(tickLower / 2);
    let sb = tick_to_price(tickUpper / 2);

    let final_amount = tick_cal(tick, tickLower, tickUpper, liquidity, sa, sb);

    let mint_price = Minted_Price(tickLower, tickUpper);

    if final_amount <= mint_price / 2 {
        return true;
    } else {
        return false;
    }
}

fn tick_to_price(tick: u128) -> u128 {
    let x = (1001 / 1000) as u128;
    let y = x * tick;
    return y;
}

fn Minted_Price(tickLower: u128, tickUpper: u128) -> u128 {
    let avg = (tickLower + tickUpper) / 2;
    return avg;
}

fn tick_cal(
    tick: u128,
    tickLower: u128,
    tickUpper: u128,
    liquidity: u128,
    sa: u128,
    sb: u128,
) -> u128 {
    if tickUpper <= tick {
        let amount0 = 0;
        let amount1 = liquidity * ((sb - sa) as u128);
        return amount1;
    } else if tick <= tickLower {
        let amount0 = liquidity * ((sb - sa) as u128) / ((sa * sb) as u128);
        let amount1 = 0;
        return amount0;
    } else {
        let amount0 = liquidity * ((sb - tick) as u128) / ((tick * sb) as u128);
        let amount1 = liquidity * ((tick - sa) as u128);
        let z = amount0 + amount1;
        return z;
    }
}
