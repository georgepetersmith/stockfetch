use clap::Parser;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

#[tokio::main]
async fn main() {
    let _args = Args::parse();
    let result = reqwest::get("https://www.alphavantage.co/query?function=OVERVIEW&symbol=IBM&apikey=demo")
        .await
        .unwrap()
        .json::<Stock>()
        .await
        .unwrap();

    println!("==========================================================");
    println!("Stockfetch: {} - {}", result.symbol, result.name);
    println!("==========================================================");
    println!("Sector: {} | {}", result.sector, result.industry);
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

// Custom deserialization function to parse strings into i32
fn parse_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Stock {
    symbol: String,
    asset_type: String,
    name: String,
    description: String,
    #[serde(rename = "CIK")]
    cik: String,
    exchange: String,
    currency: String,
    country: String,
    sector: String,
    industry: String,
    address: String,
    official_site: String,
    fiscal_year_end: String,
    latest_quarter: String,
    market_capitalization: Decimal,
    #[serde(rename = "EBITDA")]
    ebitda: Decimal,
    #[serde(rename = "PERatio")]
    pe_ratio: Decimal,
    #[serde(rename = "PEGRatio")]
    peg_ratio: Decimal,
    book_value: Decimal,
    dividend_per_share: Decimal,
    dividend_yield: Decimal,
    #[serde(rename = "EPS")]
    eps: Decimal,
    #[serde(rename = "RevenuePerShareTTM")]
    revenue_per_share_ttm: Decimal,
    profit_margin: Decimal,
    #[serde(rename = "OperatingMarginTTM")]
    operating_margin_ttm: Decimal,
    #[serde(rename = "ReturnOnAssetsTTM")]
    return_on_assets_ttm: Decimal,
    #[serde(rename = "ReturnOnEquityTTM")]
    return_on_equity_ttm: Decimal,
    #[serde(rename = "RevenueTTM")]
    revenue_ttm: Decimal,
    #[serde(rename = "GrossProfitTTM")]
    gross_profit_ttm: Decimal,
    #[serde(rename = "DilutedEPSTTM")]
    diluted_eps_ttm: Decimal,
    #[serde(rename = "QuarterlyEarningsGrowthYOY")]
    quarterly_earnings_growth_yoy: Decimal,
    #[serde(rename = "QuarterlyRevenueGrowthYOY")]
    quarterly_revenue_growth_yoy: Decimal,
    analyst_target_price: Decimal,
    #[serde(deserialize_with = "parse_string_to_i32")]
    analyst_rating_strong_buy: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    analyst_rating_buy: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    analyst_rating_hold: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    analyst_rating_sell: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    analyst_rating_strong_sell: i32,
    #[serde(rename = "TrailingPE")]
    trailing_pe: Decimal,
    #[serde(rename = "ForwardPE")]
    forward_pe: Decimal,
    #[serde(rename = "PriceToSalesRatioTTM")]
    price_to_sales_ratio_ttm: Decimal,
    price_to_book_ratio: Decimal,
    #[serde(rename = "EVToRevenue")]
    ev_to_revenue: Decimal,
    #[serde(rename = "EVToEBITDA")]
    ev_to_ebitda: Decimal,
    beta: Decimal,
    #[serde(rename = "52WeekHigh")]
    week_high_52: Decimal,
    #[serde(rename = "52WeekLow")]
    week_low_52: Decimal,
    #[serde(rename = "50DayMovingAverage")]
    day_moving_average_50: Decimal,
    #[serde(rename = "200DayMovingAverage")]
    day_moving_average_200: Decimal,
    shares_outstanding: Decimal,
    dividend_date: String,
    ex_dividend_date: String,
}
