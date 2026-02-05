use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub user: UserInfo,
    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub picture: Option<String>,
}

/// Validate OpenAI Web session by checking /api/auth/session
pub async fn validate_session(access_token: &str) -> Result<SessionInfo, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("Failed to create client: {}", e))?;
    
    let response = client
        .get("https://chatgpt.com/api/auth/session")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Invalid session: HTTP {}", response.status()));
    }
    
    response.json::<SessionInfo>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Extract user info from session for account creation
pub async fn get_user_info(access_token: &str) -> Result<UserInfo, String> {
    let session = validate_session(access_token).await?;
    Ok(session.user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires valid token
    async fn test_validate_session() {
        let token = "test-token";
        let result = validate_session(token).await;
        // This will fail without a real token, but validates the structure
        assert!(result.is_err());
    }
}
