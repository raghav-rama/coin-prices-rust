mod input;

use input::input as scan;

use std::{collections::HashMap, hash::Hash};

use serde_derive::{Deserialize, Serialize};

fn main() {
    println!("Check Real Time Coin Price!\n");
    Coins::print_coin_names();
    let mut coin_prices = CoinPrices::new();
    coin_prices.populate_coin_price_field();
    println!("Enter coin name(CaSe SeNsItIvE):");
    let coin_name = scan::<String>().unwrap();
    let result = coin_prices.get_coin_price(coin_name);
    match result {
        Ok(val) => println!("Price: ${:?}", val),
        Err(err) => println!("Error: {:?}", err),
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Coins {
    coins: Vec<Coin>,
}

impl Coins {
    fn new() -> Self {
        Coins { coins: Vec::new() }
    }
    fn fetch_raw_response() -> String {
        let response =
            ureq::get("https://api.coinstats.app/public/v1/coins?skip=0&limit=5&currency=ethereum")
                .call()
                .unwrap()
                .into_string()
                .unwrap();

        response
    }
    fn deserialize() -> Coins {
        let response = Coins::fetch_raw_response();
        serde_json::from_str(&response).unwrap()
    }
    fn print_coin_names() {
        let coins = Coins::deserialize();
        println!("Available Coins:");
        for coin in coins.coins.iter() {
            println!("{:?}", coin.name);
        }
        println!("\n");
    }
}

struct CoinPrices {
    prices: HashMap<String, f32>,
}

impl CoinPrices {
    fn new() -> Self {
        CoinPrices {
            prices: HashMap::new(),
        }
    }
    fn populate_coin_price_field(&mut self) {
        let coin_data = Coins::deserialize();
        for coin in coin_data.coins.iter() {
            self.prices.insert(coin.name.clone(), coin.price);
        }
    }
    fn get_coin_price(&self, coin_name: String) -> Result<f32, String> {
        if self.prices.contains_key(&coin_name) {
            Ok(self.prices[&coin_name])
        } else {
            Err("Coin not present!".to_owned())
        }
    }
}


// fields for json response
#[derive(Debug, Serialize, Deserialize)]
struct Coin {
    id: String,
    icon: String,
    name: String,
    symbol: String,
    rank: u32,
    price: f32,
    priceBtc: f32,
    volume: f64,
    marketCap: f64,
    availableSupply: f64,
    totalSupply: f64,
    priceChange1h: f32,
    priceChange1d: f32,
    priceChange1w: f32,
    websiteUrl: String,
    twitterUrl: String,
    exp: Vec<String>,
}
