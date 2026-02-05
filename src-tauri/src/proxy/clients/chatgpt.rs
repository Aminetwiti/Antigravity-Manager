use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct ConversationRequest {
    action: String,
    messages: Vec<Message>,
    parent_message_id: String,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct Message {
    id: String,
    author: Author,
    content: Content,
}

#[derive(Debug, Serialize)]
struct Author {
    role: String,
}

#[derive(Debug, Serialize)]
struct Content {
    content_type: String,
    parts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ConversationResponse {
    #[serde(default)]
    message: Option<ResponseMessage>,
    #[serde(default)]
    conversation_id: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    content: Option<ResponseContent>,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    #[serde(default)]
    parts: Option<Vec<String>>,
}

pub struct ChatGPTClient {
    client: Client,
    access_token: String,
}

impl ChatGPTClient {
    pub fn new(access_token: String) -> Result<Self, String> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .map_err(|e| format!("Failed to create client: {}", e))?;
        
        Ok(Self {
            client,
            access_token,
        })
    }
    
    /// Send a message to ChatGPT backend API
    pub async fn send_message(
        &self,
        user_message: String,
        model: Option<String>,
        conversation_id: Option<String>,
    ) -> Result<(String, Option<String>), String> {
        let request = ConversationRequest {
            action: "next".to_string(),
            messages: vec![Message {
                id: Uuid::new_v4().to_string(),
                author: Author {
                    role: "user".to_string(),
                },
                content: Content {
                    content_type: "text".to_string(),
                    parts: vec![user_message],
                },
            }],
            parent_message_id: Uuid::new_v4().to_string(),
            model: model.unwrap_or_else(|| "text-davinci-002-render-sha".to_string()),
            conversation_id,
        };
        
        let response = self
            .client
            .post("https://chatgpt.com/backend-api/conversation")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error: HTTP {} - {}", status, error_text));
        }
        
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        self.parse_sse_response(&body)
    }
    
    /// Parse Server-Sent Events (SSE) response from ChatGPT
    fn parse_sse_response(&self, sse_data: &str) -> Result<(String, Option<String>), String> {
        let mut conversation_id: Option<String> = None;
        let mut response_text: Option<String> = None;
        
        // Parse SSE format: data: {...}\n\n
        for line in sse_data.lines() {
            if let Some(json_str) = line.strip_prefix("data: ") {
                if json_str == "[DONE]" {
                    continue;
                }
                
                if let Ok(parsed) = serde_json::from_str::<ConversationResponse>(json_str) {
                    // Extract conversation ID
                    if let Some(cid) = parsed.conversation_id {
                        conversation_id = Some(cid);
                    }
                    
                    // Extract message content
                    if let Some(message) = parsed.message {
                        if let Some(content) = message.content {
                            if let Some(parts) = content.parts {
                                if let Some(text) = parts.first() {
                                    response_text = Some(text.clone());
                                }
                            }
                        }
                    }
                    
                    // Check for errors
                    if let Some(error) = parsed.error {
                        return Err(format!("ChatGPT error: {}", error));
                    }
                }
            }
        }
        
        response_text
            .map(|text| (text, conversation_id))
            .ok_or_else(|| "No response found in SSE stream".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_response() {
        let client = ChatGPTClient {
            client: Client::new(),
            access_token: "test".to_string(),
        };
        
        let sse_data = r#"data: {"message":{"content":{"parts":["Hello, world!"]}}, "conversation_id":"test-123"}
data: [DONE]"#;
        
        let result = client.parse_sse_response(sse_data);
        assert!(result.is_ok());
        
        let (text, conv_id) = result.unwrap();
        assert_eq!(text, "Hello, world!");
        assert_eq!(conv_id, Some("test-123".to_string()));
    }
}
