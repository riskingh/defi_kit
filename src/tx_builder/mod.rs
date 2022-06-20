mod models;

pub use models::Transaction;

use log::debug;

const TX_BUILDER_URL: &str = "http://transactions.zerion.io";
const MAX_TRIES: u32 = 3;

#[derive(Debug)]
pub enum Error {
    Parse(reqwest::Error),
    Request(reqwest::Error),
    Response(reqwest::Error),
    Unexpected(Box<dyn std::error::Error>),
}

type Result<T> = std::result::Result<T, Error>;

pub struct TxBuilder {}

impl TxBuilder {
    pub fn new() -> TxBuilder {
        TxBuilder {}
    }

    pub async fn transact(
        &self,
        from: &str,
        input_chain: &str,
        input_token: &str,
        output_token: &str,
        amount: &str,
        slippage: &str,
    ) -> Result<Transaction> {
        let transact_url = {
            let mut u = self.build_url("/swap/transact")?;
            u.query_pairs_mut()
                .append_pair("from", from)
                .append_pair("input_chain", input_chain)
                .append_pair("input_token", input_token)
                .append_pair("output_token", output_token)
                .append_pair("amount", amount)
                .append_pair("slippage", slippage);
            u
        };

        let r = self.get(transact_url).await?;

        match r.json::<models::TransactResponse>().await {
            Ok(tx_response) => Ok(tx_response.transaction),
            Err(e) => Err(Error::Parse(e)),
        }
    }

    // Retries get_once until retries are exhausted or a successful response returned
    async fn get(&self, u: url::Url) -> Result<reqwest::Response> {
        let mut try_ = 0;
        loop {
            try_ += 1;
            debug!("get request try: {}", try_);
            let r = self.get_once(u.clone()).await;
            if try_ >= MAX_TRIES || r.is_ok() {
                return r;
            }
            // TODO: delays
        }
    }

    // Wraps reqwest::get, returns errors for non 2xx codes
    async fn get_once(&self, u: url::Url) -> Result<reqwest::Response> {
        match reqwest::get(u).await {
            Err(e) => Err(Error::Request(e)),
            Ok(r) => match r.error_for_status() {
                Ok(r) => Ok(r),
                Err(e) => Err(Error::Response(e)),
            },
        }
    }

    fn build_url(&self, rel: &str) -> Result<url::Url> {
        let base_url = url::Url::parse(TX_BUILDER_URL).unwrap();
        match url::Url::options().base_url(Some(&base_url)).parse(rel) {
            Ok(u) => Ok(u),
            Err(e) => Err(Error::Unexpected(e.into())),
        }
    }
}
