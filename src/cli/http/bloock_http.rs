use super::hyper_http::SimpleHttpClient;
use super::{Client, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct BloockHttpClient {
    api_key: String,
    api_host: String,
}

#[async_trait]
impl Client for BloockHttpClient {
    async fn get<U: ToString + Send + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<Vec<u8>> {
        let headers = self.set_headers(headers);

        let client = SimpleHttpClient {};
        client.get(url, Some(headers)).await
    }

    async fn get_json<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T> {
        let headers = self.set_headers(headers);

        let client = SimpleHttpClient {};
        client.get_json(url, Some(headers)).await
    }

    async fn post<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        body: &[u8],
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T> {
        let headers = self.set_headers(headers);

        let client = SimpleHttpClient {};
        client.post(url, body, Some(headers)).await
    }

    async fn post_json<
        U: ToString + Send + 'static,
        B: Serialize + Send + 'static,
        T: DeserializeOwned + Send + 'static,
    >(
        &self,
        url: U,
        body: B,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T> {
        let headers = self.set_headers(headers);

        let client = SimpleHttpClient {};
        client.post_json(url, body, Some(headers)).await
    }
}

impl BloockHttpClient {
    pub fn new(api_key: String, api_host: String) -> Self {
        Self {
            api_key,
            api_host,
        }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub fn get_api_host(&self) -> String {
        self.api_host.clone()
    }

    pub fn set_headers(&self, headers: Option<Vec<(String, String)>>) -> Vec<(String, String)> {
        match headers {
            Some(mut h) => {
                h.push(("X-Api-Key".to_string(), self.get_api_key()));
                h
            }
            None => {
                let mut h = vec![("X-Api-Key".to_string(), self.get_api_key())];
                h
            }
        }
    }
}
