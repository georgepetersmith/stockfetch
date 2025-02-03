use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct News {
    pub items: String,
    pub sentiment_score_definition: String,
    pub relevance_score_definition: String,
    pub feed: Vec<FeedItem>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct FeedItem {
    pub title: String,
    pub url: String,
    pub time_published: String,
    pub authors: Vec<String>,
    pub summary: String,
    pub banner_image: String,
    pub source: String,
    pub category_within_source: String,
    pub source_domain: String,
    pub topics: Vec<Topic>,
    pub overall_sentiment_score: f64,
    pub overall_sentiment_label: String,
    pub ticker_sentiment: Vec<TickerSentiment>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Topic {
    pub topic: String,
    pub relevance_score: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TickerSentiment {
    pub ticker: String,
    pub relevance_score: String,
    pub ticker_sentiment_score: String,
    pub ticker_sentiment_label: String,
}
