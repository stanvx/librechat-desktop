use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid url: {0}")]
    Url(String),
    #[error("http {status}: {message}")]
    Http { status: StatusCode, message: String },
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct LibreChatClient {
    http: Client,
    base_url: Url,
    default_headers: HeaderMap,
    auth_token: Arc<RwLock<Option<String>>>,
}

impl LibreChatClient {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, ApiError> {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|err| ApiError::Url(err.to_string()))?;
        Ok(Self {
            http,
            base_url,
            default_headers: headers,
            auth_token: Arc::new(RwLock::new(None)),
        })
    }

    pub fn with_http_client(base_url: Url, http: Client) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Self {
            http,
            base_url,
            default_headers: headers,
            auth_token: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_auth_token(&self, token: impl Into<String>) {
        let mut guard = self
            .auth_token
            .write()
            .expect("authorization token lock poisoned");
        *guard = Some(token.into());
    }

    pub fn clear_auth_token(&self) {
        let mut guard = self
            .auth_token
            .write()
            .expect("authorization token lock poisoned");
        *guard = None;
    }

    fn auth_header_value(&self) -> Option<String> {
        self.auth_token
            .read()
            .expect("authorization token lock poisoned")
            .as_ref()
            .map(|token| format!("Bearer {}", token))
    }

    fn prepare_request(&self, method: Method, path: &str) -> Result<RequestBuilder, ApiError> {
        let trimmed = path.trim_start_matches('/');
        let url = self
            .base_url
            .join(trimmed)
            .map_err(|err| ApiError::Url(err.to_string()))?;
        let mut builder = self.http.request(method, url).headers(self.default_headers.clone());
        if let Some(value) = self.auth_header_value() {
            builder = builder.header(AUTHORIZATION, value);
        }
        Ok(builder)
    }

    async fn send(&self, builder: RequestBuilder) -> Result<Response, ApiError> {
        let response = builder.send().await?;
        Ok(response)
    }

    async fn parse_json<R: DeserializeOwned>(&self, response: Response) -> Result<R, ApiError> {
        let status = response.status();
        let bytes = response.bytes().await?;
        if !status.is_success() {
            let message = String::from_utf8_lossy(&bytes).to_string();
            return Err(ApiError::Http { status, message });
        }
        let result = serde_json::from_slice::<R>(&bytes)?;
        Ok(result)
    }

    pub async fn get_json<R: DeserializeOwned>(&self, path: &str) -> Result<R, ApiError> {
        let request = self.prepare_request(Method::GET, path)?;
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn get_json_with_query<Q, R>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<R, ApiError>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let request = self.prepare_request(Method::GET, path)?.query(query);
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn delete_empty(&self, path: &str) -> Result<(), ApiError> {
        let request = self.prepare_request(Method::DELETE, path)?;
        let response = self.send(request).await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let message = response.text().await.unwrap_or_default();
            Err(ApiError::Http { status, message })
        }
    }

    pub async fn delete_json<R: DeserializeOwned>(&self, path: &str) -> Result<R, ApiError> {
        let request = self.prepare_request(Method::DELETE, path)?;
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn delete_with_json<T, R>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R, ApiError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let request = self.prepare_request(Method::DELETE, path)?.json(body);
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn post_json<T, R>(&self, path: &str, body: &T) -> Result<R, ApiError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let request = self.prepare_request(Method::POST, path)?.json(body);
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn put_json<T, R>(&self, path: &str, body: &T) -> Result<R, ApiError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let request = self.prepare_request(Method::PUT, path)?.json(body);
        let response = self.send(request).await?;
        self.parse_json(response).await
    }

    pub async fn get_optional<R: DeserializeOwned>(&self, path: &str) -> Result<Option<R>, ApiError> {
        let request = self.prepare_request(Method::GET, path)?;
        let response = self.send(request).await?;
        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(ApiError::Http { status, message });
        }
        let result = response.json::<R>().await?;
        Ok(Some(result))
    }

    pub async fn post_multipart<R: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<R, ApiError> {
        let request = self.prepare_request(Method::POST, path)?.multipart(form);
        let response = self.send(request).await?;
        self.parse_json(response).await
    }
}
