# ALPHA_VANTAGE

**Project status & info:**

|                    License                     |              Crates Version               |                 Docs                 |
| :--------------------------------------------: | :---------------------------------------: | :----------------------------------: |
| [![License: MIT][license_badge]][license_link] | [![Crate][cratesio_badge]][cratesio_link] | [![Docs][docsrs_badge]][docsrs_link] |

Rust Client library built for accessing [Alphavantage][alphavantage_link] API.

### Project Functionality

- [x] Crypto Currency
- [x] Custom function and parameters
- [x] Earning
- [x] Economic Indicator
- [x] Exchange Rate
- [x] Forex
- [x] Quote Endpoint
- [x] Search Endpoint
- [x] Stock Time Series
- [x] Technical Indicators

## Add as dependencies

Edit Cargo.toml file to add alpha_vantage as dependencies

```toml
[dependencies]
alpha_vantage = "0.10.0"
```

**OR**
For bleeding edge development use

```toml
[dependencies]
alpha_vantage = {git = "https://github.com/iamsauravsharma/alpha_vantage"}
```

## Usage

Sample code to find exchange rate between two currency(both physical & digital supported)

```rust
let api_key = alpha_vantage::set_api("YOUR-API-HERE", reqwest::Client::new());
let exchange = api_key.exchange("USD","EUR").json().await.unwrap();
let rate = exchange.rate();
println!("{rate}");
```

See docs for detailed example of other functionality

[license_badge]: https://img.shields.io/github/license/iamsauravsharma/alpha_vantage.svg?style=for-the-badge
[license_link]: LICENSE
[alphavantage_link]: https://www.alphavantage.co
[cratesio_badge]: https://img.shields.io/crates/v/alpha_vantage.svg?style=for-the-badge
[cratesio_link]: https://crates.io/crates/alpha_vantage
[docsrs_badge]: https://img.shields.io/docsrs/alpha_vantage/latest?style=for-the-badge
[docsrs_link]: https://docs.rs/alpha_vantage
