// This file implements AI-related functionalities for the todo AI assistant.
// It utilizes the modelscope inference API to assist in idea expansion and step organization.

pub mod inference {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct InferenceRequest {
        prompt: String,
        max_tokens: usize,
    }

    #[derive(Serialize, Deserialize)]
    struct InferenceResponse {
        generated_text: String,
    }

    pub struct AiAssistant {
        client: Client,
        api_key: String,
    }

    impl AiAssistant {
        pub fn new(api_key: String) -> Self {
            let client = Client::new();
            AiAssistant { client, api_key }
        }

        pub async fn expand_idea(&self, idea: &str) -> Result<String, Box<dyn std::error::Error>> {
            let request = InferenceRequest {
                prompt: idea.to_string(),
                max_tokens: 100,
            };

            let response = self.client.post("https://api.modelscope.com/inference")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request)
                .send()
                .await?;

            let inference_response: InferenceResponse = response.json().await?;
            Ok(inference_response.generated_text)
        }
    }
}