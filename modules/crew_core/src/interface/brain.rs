use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ThinkRequest {
    input: String,
    // context: Option<Vec<...>> // Future expansion
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ThoughtResponse {
    pub thoughts: String,
    pub response: String,
}

pub struct ChopperBrain {
    client: Client,
    base_url: String,
}

impl ChopperBrain {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://10.0.0.215:8001".to_string(),
        }
    }

    pub async fn think(&self, input: &str) -> Result<ThoughtResponse> {
        let url = format!("{}/think", self.base_url);
        let request = ThinkRequest {
            input: input.to_string(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        let status = response.status();
        let body_text = response.text().await?;

        println!("[ChopperBrain] Status: {}", status);
        // println!("[ChopperBrain] Raw Body: {}", body_text); // Uncomment for full dump if needed

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Brain Service Error ({}): {}",
                status,
                body_text
            ));
        }

        let parsed: ThoughtResponse = serde_json::from_str(&body_text)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}. Body: {}", e, body_text))?;

        Ok(parsed)
    }
}
