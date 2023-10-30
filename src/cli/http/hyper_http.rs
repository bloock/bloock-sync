use super::HttpError;
use super::Result;
use super::Client;
use super::ApiError;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::BufReader;
use std::io::Read;
use ureq::Error;

pub struct SimpleHttpClient {}

#[async_trait]
impl Client for SimpleHttpClient {
    async fn get<U: ToString + Send + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<Vec<u8>> {
        let req = ureq::get(&url.to_string());
        self.request(req, None, headers).await
    }

    async fn get_json<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T> {
        let req = ureq::get(&url.to_string());
        let res = self.request(req, None, headers).await?;
        serde_json::from_slice(&res).map_err(|e| HttpError::DeserializeError(e.to_string()))
    }

    async fn post<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        body: &[u8],
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T> {
        let req = ureq::post(&url.to_string());
        let res = self.request(req, Some(body), headers).await?;
        serde_json::from_slice(&res).map_err(|e| HttpError::DeserializeError(e.to_string()))
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
        let bytes =
            serde_json::to_vec(&body).map_err(|e| HttpError::SerializeError(e.to_string()))?;

        let req = ureq::post(&url.to_string());
        let res = self.request(req, Some(&bytes), headers).await?;
        serde_json::from_slice(&res).map_err(|e| HttpError::DeserializeError(e.to_string()))
    }
}

impl SimpleHttpClient {
    pub fn new() -> Self {
        Self {}
    }

    async fn request(
        &self,
        mut req: ureq::Request,
        body: Option<&[u8]>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<Vec<u8>> {
        if headers.is_some() {
            for header in headers.unwrap() {
                req = req.set(&header.0, &header.1);
            }
        }

        let res = match body {
            Some(b) => req.send_bytes(b),
            None => req.call(),
        }
        .or_else(|e| match e {
            Error::Status(_, r) => Ok(r),
            Error::Transport(te) => Err(te),
        })
        .map_err(|e| {
            HttpError::RequestError(format!("Error while sending request ({})", e.to_string()))
        })?;

        let status = res.status();

        let mut reader = BufReader::new(res.into_reader());
        let mut res_buffer = Vec::new();
        reader
            .read_to_end(&mut res_buffer)
            .map_err(|e| HttpError::DeserializeError(e.to_string()))?;

        if (200..300).contains(&status) {
            Ok(res_buffer)
        } else {
            let response: ApiError = serde_json::from_slice(&res_buffer)
                .map_err(|e| HttpError::DeserializeError(e.to_string()))?;
            Err(HttpError::HttpClientError(response.message))
        }
    }
}

impl Default for SimpleHttpClient {
    fn default() -> Self {
        Self::new()
    }
}
