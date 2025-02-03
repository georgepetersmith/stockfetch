use clap::Parser;
use colored::Colorize;
use news::News;
use company_info::{Stock, get_symbol};

mod news;
mod company_info;

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    let company_info_future = async { reqwest::get("https://www.alphavantage.co/query?function=OVERVIEW&symbol=IBM&apikey=demo")
        .await?
        .json::<Stock>()
        .await
    };

    let news_future = async { reqwest::get("https://www.alphavantage.co/query?function=NEWS_SENTIMENT&tickers=AAPL&apikey=demo")
        .await?
        .json::<News>()
        .await
    };

    let (company_info, news) = match tokio::try_join!(company_info_future, news_future) {
        Ok(vals) => vals,
        Err(e) => {
            eprintln!("Request to Alpha Vantage failed: {e}");
            return;
        }
    };

    let (market_cap_amount, market_cap_suffix) = match company_info.market_capitalization {
        1000000..1000000000 => (company_info.market_capitalization as f64 / 1000000000.0, "M"),
        1000000000..1000000000000 => (company_info.market_capitalization as f64 / 1000000000.0, "B"),
        1000000000000..1000000000000000 => (company_info.market_capitalization as f64 / 1000000000000.0, "T"),
        _ => (company_info.market_capitalization as f64, company_info.currency.as_str())
    };

    let currency_symbol = get_symbol(&company_info.currency);
    let market_cap = match currency_symbol {
        Some(symbol) => format!("{symbol}{market_cap_amount:.2}{market_cap_suffix}"),
        None => format!("{market_cap_amount:.2}{market_cap_suffix} {}", company_info.currency)
    };

    let eps = match currency_symbol {
        Some(symbol) => format!("{symbol}{}", company_info.eps),
        None => format!("{} {}", company_info.eps, company_info.currency)
    };

    println!("==========================================================");
    println!("{} - {}", company_info.symbol, company_info.name);
    println!("==========================================================");
    println!("{}: {} | {}: {}", "Sector".green(), company_info.sector, "Industry".green(), company_info.industry);
    println!("{}: {}", "Market Cap".green(), market_cap);
    println!("{}: {} | {}: {}", "P/E Ratio".green(), company_info.pe_ratio, "EPS".green(), eps);
    if let Some(n) = news.feed.first() {
        println!("----------------------------------------------------------");
        println!("{}: {}", "Latest News".green(), n.title);
        println!("{}: {}", "Sentiment".green(), n.overall_sentiment_label);
    }
    println!("----------------------------------------------------------");
    println!("{}:", "Chart".green());
    println!(" ▲");
    println!(" |");
    println!(" |");
    println!(" |");
    println!(" |");
    println!(" ---------------------------> (30 days)");
    println!("==========================================================");
}

/// Stockfetch: Your Command-Line Stock Companion
///
/// Stockfetch is a lightweight, terminal-based tool for fetching real-time stock data. 
/// Inspired by Neofetch, it provides a clean, colourful snapshot of stock performance, 
/// key metrics, and company details. Perfect for traders, developers, and anyone who 
/// loves the command line.
#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
struct Args {
    /// The ticker to fetch information for
    #[arg(required = true)]
    ticker: String
}

