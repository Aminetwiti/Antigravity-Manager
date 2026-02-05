
#[tauri::command]
pub async fn get_token_stats_account_trend_daily(days: i64) -> Result<Vec<crate::modules::token_stats::AccountTrendPoint>, String> {
    crate::modules::token_stats::get_account_trend_daily(days)
}

// --- OpenAI Account Management Commands ---

/// Add OpenAI Web account (manual token input)
#[tauri::command]
pub async fn add_openai_web_account(
    app: tauri::AppHandle,
    proxy_state: tauri::State<'_, crate::commands::proxy::ProxyServiceState>,
    email: String,
    access_token: String,
    session_token: String,
) -> Result<Account, String> {
    modules::logger::log_info(&format!("Adding OpenAI Web account: {}", email));
    
    // Validate session
    let user_info = crate::auth::openai_web::get_user_info(&access_token).await?;
    
    // Create account
    let account = Account::new_openai_web(
        uuid::Uuid::new_v4().to_string(),
        user_info.email.clone(),
        access_token,
        session_token,
    );
    
    // Save account
    modules::account::save_account(&account)?;
    
    // Update index
    let mut index = modules::account::load_account_index()?;
    index.accounts.push(crate::models::AccountSummary {
        id: account.id.clone(),
        email: account.email.clone(),
    });
    modules::account::save_account_index(&index)?;
    
    // Update tray
    crate::modules::tray::update_tray_menus(&app);
    
    // Reload token pool
    let _ = crate::commands::proxy::reload_proxy_accounts(proxy_state).await;
    
    modules::logger::log_info(&format!("OpenAI Web account added: {}", user_info.email));
    Ok(account)
}

/// Add OpenAI API account (API key input)
#[tauri::command]
pub async fn add_openai_api_account(
    app: tauri::AppHandle,
    proxy_state: tauri::State<'_, crate::commands::proxy::ProxyServiceState>,
    name: String,
    api_key: String,
) -> Result<Account, String> {
    modules::logger::log_info(&format!("Adding OpenAI API account: {}", name));
    
    // Create account
    let account = Account::new_openai_api(
        uuid::Uuid::new_v4().to_string(),
        name.clone(),
        api_key,
    );
    
    // Save account
    modules::account::save_account(&account)?;
    
    // Update index
    let mut index = modules::account::load_account_index()?;
    index.accounts.push(crate::models::AccountSummary {
        id: account.id.clone(),
        email: account.email.clone(),
    });
    modules::account::save_account_index(&index)?;
    
    // Update tray
    crate::modules::tray::update_tray_menus(&app);
    
    // Reload token pool
    let _ = crate::commands::proxy::reload_proxy_accounts(proxy_state).await;
    
    modules::logger::log_info(&format!("OpenAI API account added: {}", name));
    Ok(account)
}

/// Validate OpenAI Web session
#[tauri::command]
pub async fn validate_openai_session(access_token: String) -> Result<bool, String> {
    match crate::auth::openai_web::validate_session(&access_token).await {
        Ok(_) => Ok(true),
        Err(e) => {
            modules::logger::log_warn(&format!("Session validation failed: {}", e));
            Ok(false)
        }
    }
}

/// Start OpenAI OAuth Flow (Native)
#[tauri::command]
pub async fn start_openai_oauth_flow(app: tauri::AppHandle) -> Result<String, String> {
    crate::auth::openai_oauth::start_oauth_flow(app).await
}
