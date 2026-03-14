use std::pin::Pin;

use anyhow::{bail, Context, Result};
use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;

use crate::models::*;

pub struct ApiClient {
    http: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))
                .context("Invalid API key format")?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(format!("archipelagio-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn check_error(&self, resp: reqwest::Response) -> Result<reqwest::Response> {
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }

        let body = resp.text().await.unwrap_or_default();
        let msg = if let Ok(err) = serde_json::from_str::<ApiError>(&body) {
            err.error.or(err.message).unwrap_or(body)
        } else {
            body
        };

        match status {
            StatusCode::UNAUTHORIZED => bail!("Authentication failed: {msg}. Check your API key."),
            StatusCode::FORBIDDEN => {
                bail!("Access denied: {msg}. Your API key may lack the required scope.")
            }
            StatusCode::NOT_FOUND => bail!("Not found: {msg}"),
            StatusCode::PAYMENT_REQUIRED => bail!("Insufficient credits: {msg}"),
            StatusCode::TOO_MANY_REQUESTS => bail!("Rate limited: {msg}. Please wait and retry."),
            StatusCode::UNPROCESSABLE_ENTITY => bail!("Validation error: {msg}"),
            _ => bail!("API error ({status}): {msg}"),
        }
    }

    // --- Account ---

    pub async fn get_account(&self) -> Result<Account> {
        let resp = self.http.get(self.url("/api/v1/account")).send().await?;
        let resp = self.check_error(resp).await?;
        let body: AccountResponse = resp.json().await?;
        Ok(body.data)
    }

    // --- Jobs ---

    pub async fn list_jobs(&self, limit: u32, offset: u32) -> Result<Vec<Job>> {
        let resp = self
            .http
            .get(self.url("/api/v1/jobs"))
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: JobsResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn get_job(&self, id: &str) -> Result<Job> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/jobs/{id}")))
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: JobResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn submit_job(&self, workload: &str, input: serde_json::Value) -> Result<Job> {
        let body = serde_json::json!({
            "workload": workload,
            "input": input,
        });
        let resp = self
            .http
            .post(self.url("/api/v1/jobs"))
            .json(&body)
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: JobResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn cancel_job(&self, id: &str) -> Result<()> {
        let resp = self
            .http
            .delete(self.url(&format!("/api/v1/jobs/{id}")))
            .send()
            .await?;
        self.check_error(resp).await?;
        Ok(())
    }

    pub async fn stream_job(
        &self,
        id: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/jobs/{id}/stream")))
            .send()
            .await?;
        let resp = self.check_error(resp).await?;

        let stream = resp.bytes_stream();
        let event_stream = stream.eventsource();

        Ok(Box::pin(event_stream.filter_map(|result| async move {
            match result {
                Ok(event) => {
                    if event.data.is_empty() || event.data == "[DONE]" {
                        return None;
                    }
                    match serde_json::from_str::<StreamEvent>(&event.data) {
                        Ok(evt) => Some(Ok(evt)),
                        Err(e) => Some(Err(anyhow::anyhow!("Failed to parse event: {e}"))),
                    }
                }
                Err(e) => Some(Err(anyhow::anyhow!("Stream error: {e}"))),
            }
        })))
    }

    // --- Chat ---

    pub async fn chat(&self, request: &ChatRequest) -> Result<ChatCompletionResponse> {
        let resp = self
            .http
            .post(self.url("/api/v1/chat/completions"))
            .json(request)
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: ChatCompletionResponse = resp.json().await?;
        Ok(body)
    }

    pub async fn chat_stream(
        &self,
        request: &ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionResponse>> + Send>>> {
        let resp = self
            .http
            .post(self.url("/api/v1/chat/completions"))
            .json(request)
            .send()
            .await?;
        let resp = self.check_error(resp).await?;

        let stream = resp.bytes_stream();
        let event_stream = stream.eventsource();

        Ok(Box::pin(event_stream.filter_map(|result| async move {
            match result {
                Ok(event) => {
                    if event.data.is_empty() || event.data == "[DONE]" {
                        return None;
                    }
                    match serde_json::from_str::<ChatCompletionResponse>(&event.data) {
                        Ok(chunk) => Some(Ok(chunk)),
                        Err(e) => Some(Err(anyhow::anyhow!("Failed to parse chunk: {e}"))),
                    }
                }
                Err(e) => Some(Err(anyhow::anyhow!("Stream error: {e}"))),
            }
        })))
    }

    // --- Workloads ---

    pub async fn list_workloads(&self) -> Result<Vec<Workload>> {
        let resp = self.http.get(self.url("/api/v1/workloads")).send().await?;
        let resp = self.check_error(resp).await?;
        let body: WorkloadsResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn get_workload(&self, slug: &str) -> Result<Workload> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/workloads/{slug}")))
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: WorkloadResponse = resp.json().await?;
        Ok(body.data)
    }

    // --- Hosts ---

    pub async fn list_hosts(&self) -> Result<Vec<Host>> {
        let resp = self.http.get(self.url("/api/v1/hosts")).send().await?;
        let resp = self.check_error(resp).await?;
        let body: HostsResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn get_host(&self, id: &str) -> Result<Host> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/hosts/{id}")))
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: HostResponse = resp.json().await?;
        Ok(body.data)
    }

    // --- API Keys ---

    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>> {
        let resp = self.http.get(self.url("/api/v1/api-keys")).send().await?;
        let resp = self.check_error(resp).await?;
        let body: ApiKeysResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn create_api_key(&self, name: &str) -> Result<ApiKeyCreated> {
        let body = serde_json::json!({ "name": name });
        let resp = self
            .http
            .post(self.url("/api/v1/api-keys"))
            .json(&body)
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: ApiKeyCreateResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn delete_api_key(&self, id: &str) -> Result<()> {
        let resp = self
            .http
            .delete(self.url(&format!("/api/v1/api-keys/{id}")))
            .send()
            .await?;
        self.check_error(resp).await?;
        Ok(())
    }

    // --- Market ---

    pub async fn get_market_rates(&self, workload: Option<&str>) -> Result<Vec<MarketRate>> {
        let url = match workload {
            Some(slug) => self.url(&format!("/api/v1/market/rates/{slug}")),
            None => self.url("/api/v1/market/rates"),
        };
        let resp = self.http.get(&url).send().await?;
        let resp = self.check_error(resp).await?;
        let body: MarketRatesResponse = resp.json().await?;
        Ok(body.data)
    }

    pub async fn get_market_history(&self, slug: &str) -> Result<Vec<MarketRate>> {
        let resp = self
            .http
            .get(self.url(&format!("/api/v1/market/history/{slug}")))
            .send()
            .await?;
        let resp = self.check_error(resp).await?;
        let body: MarketHistoryResponse = resp.json().await?;
        Ok(body.data)
    }
}
