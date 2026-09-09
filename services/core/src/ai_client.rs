use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AiClient { client: Client, base_url: String }

#[derive(Debug, Serialize)]
pub struct RecommendationRequest { pub product_ids: Vec<String>, pub limit: u8 }

#[derive(Debug, Deserialize)]
pub struct Recommendation { pub product_id: String, pub score: f32, pub reason: String }

impl AiClient {
    pub fn new(base_url: impl Into<String>) -> Self { Self { client: Client::new(), base_url: base_url.into().trim_end_matches('/').to_owned() } }

    pub async fn recommend(&self, product_ids: Vec<String>, limit: u8) -> Result<Vec<Recommendation>, reqwest::Error> {
        self.client.post(format!("{}/v1/recommendations", self.base_url))
            .json(&RecommendationRequest { product_ids, limit })
            .send().await?.error_for_status()?.json().await
    }
}
