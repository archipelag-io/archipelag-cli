#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// --- Account ---

#[derive(Debug, Deserialize)]
pub struct AccountResponse {
    pub data: Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub credits: f64,
    pub created_at: Option<String>,
}

// --- Jobs ---

#[derive(Debug, Deserialize)]
pub struct JobsResponse {
    pub data: Vec<Job>,
}

#[derive(Debug, Deserialize)]
pub struct JobResponse {
    pub data: Job,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub workload_id: Option<i64>,
    pub workload_slug: Option<String>,
    pub status: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub duration_ms: Option<u64>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
    pub credits_used: Option<f64>,
}

// --- Workloads ---

#[derive(Debug, Deserialize)]
pub struct WorkloadsResponse {
    pub data: Vec<Workload>,
}

#[derive(Debug, Deserialize)]
pub struct WorkloadResponse {
    pub data: Workload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workload {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub runtime_type: Option<String>,
    pub required_vram_mb: Option<u64>,
    pub required_ram_mb: Option<u64>,
    pub price_per_job: Option<f64>,
    pub is_enabled: Option<bool>,
}

// --- Hosts ---

#[derive(Debug, Deserialize)]
pub struct HostsResponse {
    pub data: Vec<Host>,
}

#[derive(Debug, Deserialize)]
pub struct HostResponse {
    pub data: Host,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub id: String,
    pub name: Option<String>,
    pub status: Option<String>,
    pub region: Option<String>,
    pub capabilities: Option<serde_json::Value>,
    pub karma_score: Option<f64>,
    pub last_heartbeat_at: Option<String>,
}

// --- API Keys ---

#[derive(Debug, Deserialize)]
pub struct ApiKeysResponse {
    pub data: Vec<ApiKey>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyCreateResponse {
    pub data: ApiKeyCreated,
}

#[derive(Debug, Deserialize)]
pub struct ApiKeyCreated {
    pub api_key: ApiKey,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub prefix: Option<String>,
    pub created_at: Option<String>,
    pub last_used_at: Option<String>,
}

// --- Market ---

#[derive(Debug, Deserialize)]
pub struct MarketRatesResponse {
    pub data: Vec<MarketRate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketRate {
    pub workload_slug: String,
    pub avg_price: Option<f64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub num_hosts: Option<u64>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MarketHistoryResponse {
    pub data: Vec<MarketRate>,
}

// --- Chat (OpenAI-compatible) ---

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: Option<ChatMessageResponse>,
    pub delta: Option<ChatDelta>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatDelta {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

// --- Stream Events ---

#[derive(Debug, Deserialize)]
pub struct StreamEvent {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub content: Option<String>,
    pub usage: Option<Usage>,
    pub error: Option<String>,
}

// --- Generic error ---

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: Option<String>,
    pub message: Option<String>,
}

