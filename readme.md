# Cryptocurrency rest api
## Direct copy of [this repo](https://github.com/fritsvt/go-crypto). I made it in Rust to learn it.

Cryptocurrency Price REST API written in Rust. Data is scraped from CoinMarketCap.

*Please note that this is an unofficial API and is __not__ supported or controlled by CoinMarketCap itself.*

## Usage
#### `GET /coins.json`

**Output:** JSON
Response:
```json
  [
     {
      "success": true,
      "timestamp": 1515959618,
      "amount_of_coins": 1433,
      "coins": [
          {
          "name": "bitcoin",
          "ticker": "BTC",
          "btc": "1.0",
          "price": "13615.6",
          "currency": "usd"
          },
          {
          "name": "ethereum",
          "ticker": "ETH",
          "btc": "0.0978496",
          "price": "1332.52",
          "currency": "usd"
          }
      ]
    }
  ]
  ...
```

## Run Locally
```sh
$ cargo run
```

## License
[WTFPL License](LICENSE)