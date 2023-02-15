use alpha_vantage::api::ApiClient;
use anyhow::Result;
use std::cell::RefCell;

use crate::stock::Stock;

pub struct ApiManager {
    pub key: String,
    pub api_client: RefCell<ApiClient>,
    pub stocks: Vec<Stock>,
}

impl ApiManager {
    pub fn new(key: &str) -> ApiManager {
        let api_client = alpha_vantage::set_api(key, reqwest::Client::new());

        ApiManager {
            key: key.to_string(),
            api_client: RefCell::new(api_client),
            stocks: Vec::new(),
        }
    }

    pub async fn update_stocks(&mut self) -> Result<()> {
        let mut temp_stocks = Vec::new();
        for stock in self.stocks.clone() {
            temp_stocks.push(stock.clone().fetch(&*self.api_client.borrow_mut()).await);
        }
        self.stocks = temp_stocks;
        Ok(())
    }
}
