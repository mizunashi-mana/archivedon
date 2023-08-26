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
    #[arg(long, default_value_t = false)]
    fetch_outbox: bool,
    #[arg(long, default_value_t = 1000)]
    default_max_pages: usize,
    #[arg(long, default_value_t = 5)]
    page_items_count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    fetch::run(
        &cli.input,
        &cli.output,
        cli.fetch_outbox,
        cli.default_max_pages,
        cli.page_items_count,
    ).await?;

    Ok(())
}
