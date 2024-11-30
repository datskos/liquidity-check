use crate::cli::{App, Command};
use crate::position_manager::PositionManager;
use crate::position_manager::PositionManager::DecreaseLiquidityParams;
use alloy_primitives::{address, Address, U256};
use alloy_provider::ProviderBuilder;
use clap::Parser;
use eyre::eyre;

mod cli;
mod position_manager;

const DEFAULT_RPC: &str = "https://eth.llamarpc.com";
const POSITION_MGR_ADDRESS: Address = address!("C36442b4a4522E871399CD717aBDD847Ab11FE88");

async fn simulate_burn(token_id: U256, block: u64, rpc_url: &str) -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;
    let contract = PositionManager::new(POSITION_MGR_ADDRESS, provider.clone());
    let position = contract.positions(token_id).block(block.into()).call().await.map_err(|e| {
        eprintln!("Failed to fetch position for token {}: {}", token_id, e);
        eyre!("Position fetch failed")
    })?;

    if position.liquidity == 0 {
        println!("Token {} has zero liquidity at block {}", token_id, block);
        return Ok(());
    }

    let res = contract
        .decreaseLiquidity(DecreaseLiquidityParams {
            tokenId: token_id,
            liquidity: position.liquidity,
            amount0Min: U256::from(0),
            amount1Min: U256::from(0),
            deadline: U256::from(2e9),
        })
        .block(block.into())
        .call()
        .await?;

    println!("===== Amounts received upon burning token = {token_id} ====");
    println!("{} = {}", position.token0, res.amount0);
    println!("{} = {}", position.token1, res.amount1);

    Ok(())
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match App::parse().command {
        Command::SimBurn { token, block, rpc } => {
            simulate_burn(U256::from(token), block, &rpc.unwrap_or(DEFAULT_RPC.to_string()))
                .await?;
        }
    }

    Ok(())
}
