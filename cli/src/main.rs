use clap::{Parser, Subcommand};
use ethers::prelude::*;
use eyre::Result;

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Get the balance of an account")]
    Balance {
        #[clap(help = "The address to get the balance of")]
        address: Address,
    },
}

#[derive(Parser, Debug)]
struct CliOpts {
    // rpc_url: String,
    #[clap(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliOpts::parse();

    match cli.cmd {
        Command::Balance { address } => {
            let provider = Provider::<Http>::try_from(
                "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
            )?;
            let balance = provider.get_balance(address, None).await?;
            let balance_str = ethers::utils::format_units(balance, "ether")?;
            println!("Balance: {}", balance_str);
        }
    }
    Ok(())
}
