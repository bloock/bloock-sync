use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error as ThisError;

pub mod bloock_http;
mod hyper_http;

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Deserialize)]
struct ApiError {
    pub message: String,
}

#[async_trait]
pub trait Client {
    async fn get<U: ToString + Send + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<Vec<u8>>;
    async fn get_json<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T>;
    async fn post<U: ToString + Send + 'static, T: DeserializeOwned + 'static>(
        &self,
        url: U,
        body: &[u8],
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T>;
    async fn post_json<
        U: ToString + Send + 'static,
        B: Serialize + Send + 'static,
        T: DeserializeOwned + Send + 'static,
    >(
        &self,
        url: U,
        body: B,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<T>;
}

#[derive(ThisError, Debug, PartialEq, Eq, Clone, Serialize)]
pub enum HttpError {
    #[error("API connected by HttpClient found an error: {0}")]
    HttpClientError(String),
    #[error("Serialize error - {0}")]
    SerializeError(String),
    #[error("Deserialize error - {0}")]
    DeserializeError(String),
    #[error("Request error - {0}")]
    RequestError(String),
    #[error("Couldn't write form data to request")]
    WriteFormDataError(),
}