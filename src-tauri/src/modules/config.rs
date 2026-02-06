use std::fs;
use serde_json;

use crate::models::AppConfig;
use super::account::get_data_dir;

const CONFIG_FILE: &str = "gui_config.json";

/// Load application configuration
pub fn load_app_config() -> Result<AppConfig, String> {
    let data_dir = get_data_dir()?;
    let config_path = data_dir.join(CONFIG_FILE);
    
    if !config_path.exists() {
        let config = AppConfig::new();
        // [FIX #1460] Persist initial config to prevent new API Key on every refresh
        let _ = save_app_config(&config);
        return Ok(config);
    }
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("failed_to_read_config_file: {}", e))?;
    
    let mut v: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("failed_to_parse_config_file: {}", e))?;
    
    let mut modified = false;

    // Migration logic
    if let Some(proxy) = v.get_mut("proxy") {
        let mut custom_mapping = proxy.get("custom_mapping")
            .and_then(|m| m.as_object())
            .map(|m| m.clone())
            .unwrap_or_default();

        // Migrate Anthropic mapping
        if let Some(anthropic) = proxy.get_mut("anthropic_mapping").and_then(|m| m.as_object_mut()) {
            for (k, v) in anthropic.iter() {
                // Only move non-series fields, as series fields are now handled by Preset logic or builtin tables
                if !k.ends_with("-series") {
                    if !custom_mapping.contains_key(k) {
                        custom_mapping.insert(k.clone(), v.clone());
                    }
                }
            }
            // Remove old field
            proxy.as_object_mut().unwrap().remove("anthropic_mapping");
            modified = true;
        }

        // Migrate OpenAI mapping
        if let Some(openai) = proxy.get_mut("openai_mapping").and_then(|m| m.as_object_mut()) {
            for (k, v) in openai.iter() {
                if !k.ends_with("-series") {
                    if !custom_mapping.contains_key(k) {
                        custom_mapping.insert(k.clone(), v.clone());
                    }
                }
            }
            // Remove old field
            proxy.as_object_mut().unwrap().remove("openai_mapping");
            modified = true;
        }

        if modified {
            proxy.as_object_mut().unwrap().insert("custom_mapping".to_string(), serde_json::Value::Object(custom_mapping));
        }
    }

    let mut modified = false;
    let mut config: AppConfig = serde_json::from_value(v)
        .map_err(|e| format!("failed_to_convert_config_after_migration: {}", e))?;

    // Environment variable overrides (highest priority)
    
    // API Key
    if let Ok(key) = std::env::var("ABV_API_KEY").or_else(|_| std::env::var("API_KEY")) {
        if !key.trim().is_empty() && config.proxy.api_key != key {
            config.proxy.api_key = key;
            modified = true;
        }
    }

    // Port
    if let Ok(port_str) = std::env::var("ABV_PORT").or_else(|_| std::env::var("PORT")) {
        if let Ok(port) = port_str.parse::<u16>() {
            if config.proxy.port != port {
                config.proxy.port = port;
                modified = true;
            }
        }
    }

    // Web UI Password
    if let Ok(pwd) = std::env::var("ABV_WEB_PASSWORD").or_else(|_| std::env::var("WEB_PASSWORD")) {
        if !pwd.trim().is_empty() && config.proxy.admin_password != Some(pwd.clone()) {
            config.proxy.admin_password = Some(pwd);
            modified = true;
        }
    }

    // Auth Mode
    if let Ok(mode_str) = std::env::var("ABV_AUTH_MODE").or_else(|_| std::env::var("AUTH_MODE")) {
        let mode = match mode_str.to_lowercase().as_str() {
            "off" => Some(crate::proxy::ProxyAuthMode::Off),
            "strict" => Some(crate::proxy::ProxyAuthMode::Strict),
            "all_except_health" => Some(crate::proxy::ProxyAuthMode::AllExceptHealth),
            "auto" => Some(crate::proxy::ProxyAuthMode::Auto),
            _ => None,
        };
        if let Some(m) = mode {
            if config.proxy.auth_mode != m {
                config.proxy.auth_mode = m;
                modified = true;
            }
        }
    }

    // If migration or env overrides occurred, auto-save once to clean up/persist
    if modified {
        let _ = save_app_config(&config);
    }

    Ok(config)
}

/// Save application configuration
pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    let data_dir = get_data_dir()?;
    let config_path = data_dir.join(CONFIG_FILE);
    
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("failed_to_serialize_config: {}", e))?;
    
    fs::write(&config_path, content)
        .map_err(|e| format!("failed_to_save_config: {}", e))
}
