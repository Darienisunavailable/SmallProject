// Necessary libraries to include
use std::fs::{OpenOptions};
use std::io::Write;
use std::thread;
use std::time::Duration;
use ureq;
use serde_json::Value;

// Define the Pricing trait
// Fetch price will get price from api as f64
// Save to file will save the price to a file
trait Pricing {
    fn fetch_price(&self) -> f64;
    fn save_to_file(&self, price: f64, filename: &str);
}

// Create a struct for Bitcoin
struct Bitcoin;

// Implement the Pricing trait for Bitcoin
// Ureq::get is used to get the price from the api
// The response is then converted to json
// The price is then extracted from the json
// The price is then saved to a file
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd").call().unwrap();
        let json: Value = response.into_json().unwrap();
        json["bitcoin"]["usd"].as_f64().unwrap()
    }

    fn save_to_file(&self, price: f64, filename: &str) {
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();
        writeln!(file, "Bitcoin: {}", price).unwrap();
    }
}

// Create a struct for Ethereum
struct Ethereum;

// Implement the Pricing trait for Ethereum
// Ureq::get is used to get the price from the api
// The response is then converted to json
// The price is then extracted from the json
// The price is then saved to a file
impl Pricing for Ethereum {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd").call().unwrap();
        let json: Value = response.into_json().unwrap();
        json["ethereum"]["usd"].as_f64().unwrap()
    }

    fn save_to_file(&self, price: f64, filename: &str) {
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();
        writeln!(file, "Ethereum: {}", price).unwrap();
    }
}

// Create a struct for SP500
struct SP500;

// Implement the Pricing trait for SP500
// Ureq::get is used to get the price from the api
// The response is then converted to json
// The price is then extracted from the json
// The price is then saved to a file
impl Pricing for SP500 {
    fn fetch_price(&self) -> f64 {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=sp-500&vs_currencies=usd").call().unwrap();
        let json: Value = response.into_json().unwrap();
        json["sp-500"]["usd"].as_f64().unwrap()
    }

    fn save_to_file(&self, price: f64, filename: &str) {
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();
        writeln!(file, "SP500: {}", price).unwrap();
        writeln!(file, "----------------").unwrap();
    }
}

fn main() {
    // Create a vector of assets
    // Bitcoin, Ethereum, and SP500 are added to the vector
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    // Loop through the assets
    loop {
        for asset in &assets {
            let price = asset.fetch_price();
            println!("Price: {}", price);
            asset.save_to_file(price, "price.txt");
        }

        thread::sleep(Duration::from_secs(10));
    }
}


