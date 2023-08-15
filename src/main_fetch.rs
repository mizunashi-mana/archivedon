use std::error::Error;

use clap::Parser;

mod fetch;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    fetch::run(&cli.input, &cli.output).await?;

    Ok(())
}
