use std::sync::Arc;

use bindings::ierc20_metadata::IERC20Metadata;
use clap::{Parser, Subcommand};
use ethers::prelude::*;
use eyre::Result;

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Get the native token balance of an account")]
    Balance {
        #[clap(help = "The address to get the balance of")]
        address: Address,
    },
    #[clap(about = "Get the erc20 token balance of an account")]
    Erc20Balance {
        #[clap(help = "The token address")]
        token: Address,
        #[clap(help = "The address to get the balance of")]
        address: Address,
    },
}

#[derive(Parser, Debug)]
struct CliOpts {
    #[clap(
        long,
        env = "RPC_URL",
        default_value = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27"
    )]
    rpc_url: String,
    #[clap(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let CliOpts { rpc_url, cmd, .. } = CliOpts::parse();

    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

    match cmd {
        Command::Balance { address } => {
            let balance = provider.get_balance(address, None).await?;
            let balance_str = ethers::utils::format_units(balance, "ether")?;
            println!("Balance: {}", balance_str);
        }
        Command::Erc20Balance { token, address } => {
            let contract = IERC20Metadata::new(token, provider.clone());
            let decimals = contract.decimals().call().await?;
            let balance = contract.balance_of(address).call().await?;
            let symbol = contract.symbol().call().await?;
            let balance_f64 = balance.as_u128() as f64 / 10_f64.powi(decimals.into());
            println!("{} {}", balance_f64, symbol);
        }
    }
    Ok(())
}
