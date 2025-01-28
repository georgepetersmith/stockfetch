use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:?}", args.ticker);
}

/// Stockfetch: Your Command-Line Stock Companion
///
/// Stockfetch is a lightweight, terminal-based tool for fetching real-time stock data. 
/// Inspired by neofetch, it provides a clean, colorful snapshot of stock performance, 
/// key metrics, and company details. Perfect for traders, developers, and anyone who 
/// loves the command line.
#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
struct Args {
    /// The ticker to fetch information for
    #[arg(required = true)]
    ticker: String
}
