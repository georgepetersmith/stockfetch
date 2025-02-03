use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use phf::{self, phf_map};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct Stock {
    pub symbol: String,
    pub asset_type: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "CIK")]
    pub cik: String,
    pub exchange: String,
    pub currency: String,
    pub country: String,
    pub sector: String,
    pub industry: String,
    pub address: String,
    pub official_site: String,
    pub fiscal_year_end: String,
    pub latest_quarter: String,
    #[serde(deserialize_with = "parse_string_to_i64")]
    pub market_capitalization: i64,
    #[serde(rename = "EBITDA")]
    pub ebitda: Decimal,
    #[serde(rename = "PERatio")]
    pub pe_ratio: Decimal,
    #[serde(rename = "PEGRatio")]
    pub peg_ratio: Decimal,
    pub book_value: Decimal,
    pub dividend_per_share: Decimal,
    pub dividend_yield: Decimal,
    #[serde(rename = "EPS")]
    pub eps: Decimal,
    #[serde(rename = "RevenuePerShareTTM")]
    pub revenue_per_share_ttm: Decimal,
    pub profit_margin: Decimal,
    #[serde(rename = "OperatingMarginTTM")]
    pub operating_margin_ttm: Decimal,
    #[serde(rename = "ReturnOnAssetsTTM")]
    pub return_on_assets_ttm: Decimal,
    #[serde(rename = "ReturnOnEquityTTM")]
    pub return_on_equity_ttm: Decimal,
    #[serde(rename = "RevenueTTM")]
    pub revenue_ttm: Decimal,
    #[serde(rename = "GrossProfitTTM")]
    pub gross_profit_ttm: Decimal,
    #[serde(rename = "DilutedEPSTTM")]
    pub diluted_eps_ttm: Decimal,
    #[serde(rename = "QuarterlyEarningsGrowthYOY")]
    pub quarterly_earnings_growth_yoy: Decimal,
    #[serde(rename = "QuarterlyRevenueGrowthYOY")]
    pub quarterly_revenue_growth_yoy: Decimal,
    pub analyst_target_price: Decimal,
    #[serde(deserialize_with = "parse_string_to_i32")]
    pub analyst_rating_strong_buy: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    pub analyst_rating_buy: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    pub analyst_rating_hold: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    pub analyst_rating_sell: i32,
    #[serde(deserialize_with = "parse_string_to_i32")]
    pub analyst_rating_strong_sell: i32,
    #[serde(rename = "TrailingPE")]
    pub trailing_pe: Decimal,
    #[serde(rename = "ForwardPE")]
    pub forward_pe: Decimal,
    #[serde(rename = "PriceToSalesRatioTTM")]
    pub price_to_sales_ratio_ttm: Decimal,
    pub price_to_book_ratio: Decimal,
    #[serde(rename = "EVToRevenue")]
    pub ev_to_revenue: Decimal,
    #[serde(rename = "EVToEBITDA")]
    pub ev_to_ebitda: Decimal,
    pub beta: Decimal,
    #[serde(rename = "52WeekHigh")]
    pub week_high_52: Decimal,
    #[serde(rename = "52WeekLow")]
    pub week_low_52: Decimal,
    #[serde(rename = "50DayMovingAverage")]
    pub day_moving_average_50: Decimal,
    #[serde(rename = "200DayMovingAverage")]
    pub day_moving_average_200: Decimal,
    pub shares_outstanding: Decimal,
    pub dividend_date: String,
    pub ex_dividend_date: String,
}

// Custom deserialisation function to parse strings into i32
fn parse_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}

fn parse_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i64>().map_err(serde::de::Error::custom)
}

static CURRENCIES: phf::Map<&'static str, &'static str> = phf_map! {
    // Major Fiat Currencies (ISO 4217)
    "USD" => "$",   // US Dollar
    "EUR" => "€",   // Euro
    "JPY" => "¥",   // Japanese Yen
    "GBP" => "£",   // British Pound
    "AUD" => "A$",  // Australian Dollar
    "CAD" => "CA$", // Canadian Dollar
    "CHF" => "CHF", // Swiss Franc
    "CNY" => "¥",   // Chinese Yuan
    "HKD" => "HK$", // Hong Kong Dollar
    "NZD" => "NZ$", // New Zealand Dollar
    "SEK" => "kr",  // Swedish Krona
    "KRW" => "₩",   // South Korean Won
    "SGD" => "S$",  // Singapore Dollar
    "NOK" => "kr",  // Norwegian Krone
    "MXN" => "MX$", // Mexican Peso
    "INR" => "₹",   // Indian Rupee
    "RUB" => "₽",   // Russian Ruble
    "ZAR" => "R",   // South African Rand
    "TRY" => "₺",   // Turkish Lira
    "BRL" => "R$",  // Brazilian Real
    "DKK" => "kr",  // Danish Krone
    "PLN" => "zł",  // Polish Złoty
    "THB" => "฿",   // Thai Baht
    "IDR" => "Rp",  // Indonesian Rupiah
    "HUF" => "Ft",  // Hungarian Forint
    "CZK" => "Kč",  // Czech Koruna
    "ILS" => "₪",   // Israeli New Shekel
    "CLP" => "$",   // Chilean Peso
    "PHP" => "₱",   // Philippine Peso
    "AED" => "د.إ", // UAE Dirham
    "SAR" => "ر.س", // Saudi Riyal

    // Cryptocurrencies (Digital)
    "BTC" => "₿",   // Bitcoin
    "ETH" => "Ξ",   // Ethereum
    "XRP" => "XRP", // Ripple
    "LTC" => "Ł",   // Litecoin
    "BCH" => "Ƀ",   // Bitcoin Cash
    "ADA" => "ADA", // Cardano
    "DOT" => "DOT", // Polkadot
    "LINK" => "LINK", // Chainlink
    "DOGE" => "Ð",  // Dogecoin
    "USDT" => "₮",  // Tether
    "BNB" => "BNB", // Binance Coin

    // Precious Metals (Non-ISO)
    "XAU" => "XAU", // Gold (troy ounce)
    "XAG" => "XAG", // Silver (troy ounce)
    "XPT" => "XPT", // Platinum
    "XPD" => "XPD", // Palladium

    // Other Common Alpha Vantage Codes
    "BTCUSD" => "BTC/USD", // Bitcoin to USD (hybrid pairs)
    "ETHUSD" => "ETH/USD", // Ethereum to USD
    "USDEUR" => "USD/EUR", // USD to Euro (forex pair)
    // ... Add more pairs as needed
};

pub fn get_symbol(code: &str) -> Option<&'static str> {
    CURRENCIES.get(code.to_uppercase().as_str()).copied()
}
