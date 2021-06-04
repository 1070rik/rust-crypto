extern crate serde_json;
extern crate reqwest;
extern crate scraper;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use serde_json::{Value};
use scraper::{Html, Selector};
use tide::prelude::*;

const COIN_FILE: &str = "static/coins.json";
const WEB_URL: &str = "https://coinmarketcap.com/all/views/all/";

#[derive(Deserialize, Serialize)]
struct Coin {
    name: String,
    ticker: String,
    price: String,
    currency: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    refresh_coins(60 * 1000);

    app.at("/").get(|_| async {
        Ok(json!({
            "success": true,
            "message": "Welcome to the crypto-prices api, /coins.json for the data",
        }))
    });

    app.at("/coins.json").serve_file(COIN_FILE)?;

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn refresh_coins(interval: u64) {
    thread::spawn(move || {
        loop {
            println!("Fetching new coins....");

            let mut coins_vec: Vec<Coin> = vec![];
            let response = reqwest::blocking::get(WEB_URL);
            let body: String = response.unwrap().text().unwrap();
            // parses string of HTML as a document
            let fragment: Html = Html::parse_document(&body);
            // parses based on a CSS selector
            let coins: Selector = Selector::parse(".cmc-table-row").unwrap();

            for coin in fragment.select(&coins) {
                let coin_fragment: Html = Html::parse_fragment(coin.html().as_str());

                let name_selector: Selector = Selector::parse(".cmc-table__column-name--name").unwrap();
                let ticker_selector: Selector = Selector::parse(".cmc-table__column-name--symbol").unwrap();
                let price_selector: Selector = Selector::parse(".price___3rj7O > a").unwrap();

                let name = coin_fragment.select(&name_selector).next();
                let ticker = coin_fragment.select(&ticker_selector).next();
                let price = coin_fragment.select(&price_selector).next();

                if name.is_some() && ticker.is_some() && price.is_some() {
                    coins_vec.push(Coin {
                        name: name.unwrap().inner_html(),
                        ticker: ticker.unwrap().inner_html(),
                        price: price.unwrap().inner_html(),
                        currency: "usd".to_string(),
                    });
                }
            }

            let path = Path::new(&COIN_FILE);
            let display = path.display();

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            // We need to define the amount of coins here because it gets borrowed in the next match
            let coin_amount: usize = coins_vec.len();

            match file.write_all(generate_json_coins_result(coins_vec).to_string().as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote {} coins to {}", coin_amount, display),
            }

            thread::sleep(Duration::from_millis(interval));
        }
    });
}

fn get_epoch_in_sec() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("0");

    return since_the_epoch.as_secs().to_string();
}

fn generate_json_coins_result(coins: Vec<Coin>) -> Value {
    return json!({
                "success": true,
                "timestamp": get_epoch_in_sec(),
                "amount": coins.len(),
                "coins": coins,
            });
}