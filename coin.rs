use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use exitfailure::ExitFailure;
use reqwest;

impl Coin {
  pub async fn fetch(coin: &String) -> Result<Self, ExitFailure> {
    let url = format!("https://api.coingecko.com/api/v3/coins/{id}", id = coin);
    let result = reqwest::get(url).await.unwrap().json::<Coin>().await.unwrap();

    return Ok(result);
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coin {
  pub id: String,
  pub symbol: String,
  pub name: String,
  pub description: HashMap<String, String>,
  pub categories: Vec<String>,
  pub links: Option<Links>,
  pub market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketData {
  pub current_price: HashMap<String, f32>,
  pub ath: HashMap<String, f32>,
  pub ath_change_percentage: HashMap<String, f32>,
  pub atl: HashMap<String, f32>,
  pub atl_change_percentage: HashMap<String, f32>,
  pub market_cap: HashMap<String, f32>,
  pub market_cap_rank: f32,
  pub high_24h: HashMap<String, f32>,
  pub low_24h: HashMap<String, f32>,
  pub price_change_24h: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
  homepage: Vec<String>,
  blockchain_site: Vec<String>,
  official_forum_url: Vec<String>,
  chat_url: Vec<String>,
  announcement_url: Vec<String>,
  twitter_screen_name: Option<String>,
  facebook_username: Option<String>,
  bitcointalk_thread_identifier: Option<i32>,
  telegram_channel_identifier: Option<String>,
  subreddit_url: Option<String>
}

impl Trending {
  pub async fn fetch() -> Result<Trending, ExitFailure> {
    let url = "https://api.coingecko.com/api/v3/search/trending";
    let result = reqwest::get(url).await.unwrap().json::<Trending>().await.unwrap();

    return Ok(result);
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trending {
  pub coins: Vec<CoinTrending>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinTrending {
  pub item: Item
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  pub id: String,
  pub name: String,
  pub symbol: String,
  pub market_cap_rank: i32,
  pub price_btc: f32,
  pub score: i32
}