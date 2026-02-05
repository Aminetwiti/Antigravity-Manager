use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use super::{token::TokenData, quota::QuotaData};

/// Provider type for account
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Google,
    #[serde(rename = "openai_web")]
    OpenAIWeb,
    #[serde(rename = "openai_api")]
    OpenAIAPI,
}

/// Provider-specific credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderCredentials {
    Google {
        token: TokenData,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        device_profile: Option\u003cDeviceProfile\u003e,
    },
    OpenAIWeb {
        access_token: String,
        session_token: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        puid: Option\u003cString\u003e,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cf_clearance: Option\u003cString\u003e,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expires_at: Option\u003ci64\u003e,
    },
    OpenAIAPI {
        api_key: String,
    },
}

impl ProviderCredentials {
    /// Get the provider type from credentials
    pub fn provider(&self) -> Provider {
        match self {
            ProviderCredentials::Google { .. } => Provider::Google,
            ProviderCredentials::OpenAIWeb { .. } => Provider::OpenAIWeb,
            ProviderCredentials::OpenAIAPI { .. } => Provider::OpenAIAPI,
        }
    }
}

/// Account metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option\u003cString\u003e,
    pub created_at: i64,
    pub last_used: i64,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AccountMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            name: None,
            created_at: now,
            last_used: now,
            is_active: true,
        }
    }
}

/// 账号数据结构 (Refactored for multi-provider support)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub provider: Provider,
    pub credentials: ProviderCredentials,
    pub metadata: AccountMetadata,
    
    /// 设备指纹历史（生成/采集时记录），不含基线 (Google only, for backward compatibility)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_history: Vec\u003cDeviceProfileVersion\u003e,
    
    pub quota: Option\u003cQuotaData\u003e,
    
    /// Disabled accounts are ignored by the proxy token pool (e.g. revoked refresh_token -> invalid_grant).
    #[serde(default)]
    pub disabled: bool,
    /// Optional human-readable reason for disabling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option\u003cString\u003e,
    /// Unix timestamp when the account was disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option\u003ci64\u003e,
    
    /// User manually disabled proxy feature (does not affect app usage).
    #[serde(default)]
    pub proxy_disabled: bool,
    /// Optional human-readable reason for proxy disabling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_disabled_reason: Option\u003cString\u003e,
    /// Unix timestamp when the proxy was disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_disabled_at: Option\u003ci64\u003e,
    
    /// 受配额保护禁用的模型列表
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub protected_models: HashSet\u003cString\u003e,
    
    /// 403 验证阻止状态 (VALIDATION_REQUIRED)
    #[serde(default)]
    pub validation_blocked: bool,
    /// 验证阻止截止时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_blocked_until: Option\u003ci64\u003e,
    /// 验证阻止原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_blocked_reason: Option\u003cString\u003e,
    
    /// 绑定的代理 ID (None = 使用全局代理池)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option\u003cString\u003e,
    /// 代理绑定时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_bound_at: Option\u003ci64\u003e,
}

impl Account {
    /// Create a new Google account (backward compatible constructor)
    pub fn new(id: String, email: String, token: TokenData) -> Self {
        Self::new_google(id, email, token, None)
    }
    
    /// Create a new Google account with optional device profile
    pub fn new_google(
        id: String,
        email: String,
        token: TokenData,
        device_profile: Option\u003cDeviceProfile\u003e,
    ) -> Self {
        Self {
            id,
            email,
            provider: Provider::Google,
            credentials: ProviderCredentials::Google {
                token,
                device_profile,
            },
            metadata: AccountMetadata::default(),
            device_history: Vec::new(),
            quota: None,
            disabled: false,
            disabled_reason: None,
            disabled_at: None,
            proxy_disabled: false,
            proxy_disabled_reason: None,
            proxy_disabled_at: None,
            protected_models: HashSet::new(),
            validation_blocked: false,
            validation_blocked_until: None,
            validation_blocked_reason: None,
            proxy_id: None,
            proxy_bound_at: None,
        }
    }
    
    /// Create a new OpenAI Web account
    pub fn new_openai_web(
        id: String,
        email: String,
        access_token: String,
        session_token: String,
    ) -> Self {
        Self {
            id,
            email,
            provider: Provider::OpenAIWeb,
            credentials: ProviderCredentials::OpenAIWeb {
                access_token,
                session_token,
                puid: None,
                cf_clearance: None,
                expires_at: None,
            },
            metadata: AccountMetadata::default(),
            device_history: Vec::new(),
            quota: None,
            disabled: false,
            disabled_reason: None,
            disabled_at: None,
            proxy_disabled: false,
            proxy_disabled_reason: None,
            proxy_disabled_at: None,
            protected_models: HashSet::new(),
            validation_blocked: false,
            validation_blocked_until: None,
            validation_blocked_reason: None,
            proxy_id: None,
            proxy_bound_at: None,
        }
    }
    
    /// Create a new OpenAI API account
    pub fn new_openai_api(id: String, email: String, api_key: String) -> Self {
        Self {
            id,
            email,
            provider: Provider::OpenAIAPI,
            credentials: ProviderCredentials::OpenAIAPI { api_key },
            metadata: AccountMetadata::default(),
            device_history: Vec::new(),
            quota: None,
            disabled: false,
            disabled_reason: None,
            disabled_at: None,
            proxy_disabled: false,
            proxy_disabled_reason: None,
            proxy_disabled_at: None,
            protected_models: HashSet::new(),
            validation_blocked: false,
            validation_blocked_until: None,
            validation_blocked_reason: None,
            proxy_id: None,
            proxy_bound_at: None,
        }
    }

    pub fn update_last_used(&mut self) {
        self.metadata.last_used = chrono::Utc::now().timestamp();
    }

    pub fn update_quota(&mut self, quota: QuotaData) {
        self.quota = Some(quota);
    }
    
    /// Get provider name as string
    pub fn provider_name(&self) -> &str {
        match self.provider {
            Provider::Google => "google",
            Provider::OpenAIWeb => "openai_web",
            Provider::OpenAIAPI => "openai_api",
        }
    }
    
    /// Check if account credentials are expired (for providers with expiration)
    pub fn is_expired(&self) -> bool {
        match &self.credentials {
            ProviderCredentials::OpenAIWeb { expires_at, .. } => {
                if let Some(exp) = expires_at {
                    chrono::Utc::now().timestamp() > *exp
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    // === Backward Compatibility Helpers ===
    
    /// Get the account name (from metadata)
    pub fn name(&self) -> Option<&String> {
        self.metadata.name.as_ref()
    }
    
    /// Get mutable reference to account name
    pub fn name_mut(&mut self) -> &mut Option<String> {
        &mut self.metadata.name
    }
    
    /// Get Google token data (for backward compatibility)
    /// Returns None for non-Google providers
    pub fn token(&self) -> Option<&TokenData> {
        match &self.credentials {
            ProviderCredentials::Google { token, .. } => Some(token),
            _ => None,
        }
    }
    
    /// Get mutable Google token data (for backward compatibility)
    /// Returns None for non-Google providers
    pub fn token_mut(&mut self) -> Option<&mut TokenData> {
        match &mut self.credentials {
            ProviderCredentials::Google { token, .. } => Some(token),
            _ => None,
        }
    }
    
    /// Set Google token (for backward compatibility)
    /// Only works for Google providers
    pub fn set_token(&mut self, new_token: TokenData) -> Result<(), String> {
        match &mut self.credentials {
            ProviderCredentials::Google { token, .. } => {
                *token = new_token;
                Ok(())
            }
            _ => Err("Cannot set token on non-Google provider".to_string()),
        }
    }
    
    /// Get device profile (Google only)
    /// Returns None for non-Google providers or if not set
    pub fn device_profile(&self) -> Option<&DeviceProfile> {
        match &self.credentials {
            ProviderCredentials::Google { device_profile, .. } => device_profile.as_ref(),
            _ => None,
        }
    }
    
    /// Get mutable device profile (Google only)
    pub fn device_profile_mut(&mut self) -> Option<&mut Option<DeviceProfile>> {
        match &mut self.credentials {
            ProviderCredentials::Google { device_profile, .. } => Some(device_profile),
            _ => None,
        }
    }
    
    /// Set device profile (Google only)
    pub fn set_device_profile(&mut self, profile: Option<DeviceProfile>) -> Result<(), String> {
        match &mut self.credentials {
            ProviderCredentials::Google { device_profile, .. } => {
                *device_profile = profile;
                Ok(())
            }
            _ => Err("Cannot set device_profile on non-Google provider".to_string()),
        }
    }
    
    /// Get created_at timestamp (from metadata)
    pub fn created_at(&self) -> i64 {
        self.metadata.created_at
    }
    
    /// Get last_used timestamp (from metadata) 
    pub fn last_used(&self) -> i64 {
        self.metadata.last_used
    }
}


/// 账号索引数据（accounts.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIndex {
    pub version: String,
    pub accounts: Vec\u003cAccountSummary\u003e,
    pub current_account_id: Option\u003cString\u003e,
}

/// 账号摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub id: String,
    pub email: String,
    pub name: Option\u003cString\u003e,
    #[serde(default)]
    pub provider: Option\u003cProvider\u003e,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub proxy_disabled: bool,
    pub created_at: i64,
    pub last_used: i64,
}

impl AccountIndex {
    pub fn new() -> Self {
        Self {
            version: "3.0".to_string(),
            accounts: Vec::new(),
            current_account_id: None,
        }
    }
}

impl Default for AccountIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// 设备指纹（storage.json 中 telemetry 相关字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub machine_id: String,
    pub mac_machine_id: String,
    pub dev_device_id: String,
    pub sqm_id: String,
}

/// 指纹历史版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfileVersion {
    pub id: String,
    pub created_at: i64,
    pub label: String,
    pub profile: DeviceProfile,
    #[serde(default)]
    pub is_current: bool,
}

/// 导出账号项（用于备份/迁移）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportItem {
    pub email: String,
    pub refresh_token: String,
}

/// 导出账号响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountExportResponse {
    pub accounts: Vec\u003cAccountExportItem\u003e,
}
