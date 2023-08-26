use clap::{Parser, Subcommand};
use std::error::Error;

mod server;

use server::cmd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(long, env = "ADDR")]
        addr: Option<String>,

        #[arg(short, long, env = "PORT")]
        port: u16,

        #[arg(long, env = "RESOURCE_DIR")]
        resource_dir: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve {
            addr,
            port,
            resource_dir,
            ..
        } => cmd::serve::run(addr, *port, resource_dir),
    }
    .await?;

    Ok(())
}
