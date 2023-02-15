use alpha_vantage::api::ApiClient;
use serde_derive::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct Stock {
    pub ticker: String,
    pub time_stamp: u64,
    pub open_price: f64,
    pub close_price: f64,
}

impl Stock {
    pub fn create_stock(ticker: String) -> Stock {
        Stock {
            ticker,
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            open_price: 0.00,
            close_price: 0.00,
        }
    }

    pub async fn fetch(&mut self, api_client: &ApiClient) -> Stock {
        let stock_time = api_client
            .stock_time(
                alpha_vantage::stock_time::StockFunction::IntraDay,
                &self.ticker.clone(),
            )
            .interval(alpha_vantage::api::TimeSeriesInterval::FifteenMin)
            .output_size(alpha_vantage::api::OutputSize::Compact)
            .json()
            .await
            .unwrap();

        let fetch_result = stock_time.data();
        let latest = fetch_result[fetch_result.len() - 1].clone();

        println!(
            "Ticker symbol{}\nStart: {}\nEnd: {}\n",
            self.ticker,
            latest.open(),
            latest.close()
        );

        Stock {
            ticker: self.ticker.clone(),
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            open_price: (latest.open()) as f64,
            close_price: (latest.close()) as f64,
        }
    }
}
