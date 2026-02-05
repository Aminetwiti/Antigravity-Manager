use crate::models::account::{Account, Provider, ProviderCredentials};
use crate::proxy::server::AppState;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann"; // VS Code Extension Client ID
const REDIRECT_URI: &str = "http://localhost:1455/auth/callback";
const AUTH_URL: &str = "https://auth.openai.com/oauth/authorize";
const TOKEN_URL: &str = "https://auth0.openai.com/oauth/token";

#[derive(Clone)]
struct OAuthState {
    app_handle: AppHandle,
    verifier: String,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    id_token: Option<String>,
    scope: Option<String>,
}

#[derive(Serialize, Clone)]
struct OAuthSuccessPayload {
    email: String,
    access_token: String,
    refresh_token: String,
}

pub async fn start_oauth_flow(app_handle: AppHandle) -> Result<String, String> {
    // 1. Generate PKCE Verifier and Challenge
    let verifier = generate_verifier();
    let challenge = generate_challenge(&verifier);
    let state_param = generate_verifier(); // Reusing random string gen for state

    // 2. Setup Shutdown Channel
    let (tx, rx) = oneshot::channel();
    let oauth_state = OAuthState {
        app_handle: app_handle.clone(),
        verifier: verifier.clone(),
        shutdown_tx: Arc::new(Mutex::new(Some(tx))),
    };

    // 3. Start Local Server
    let app = Router::new()
        .route("/auth/callback", get(handle_callback))
        .with_state(oauth_state);

    let listener = TcpListener::bind("127.0.0.1:1455")
        .await
        .map_err(|e| format!("Failed to bind port 1455: {}", e))?;

    tauri::async_runtime::spawn(async move {
        if let Err(e) = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                rx.await.ok();
            })
            .await
        {
            eprintln!("OAuth server error: {}", e);
        }
    });

    // 4. Construct Auth URL
    let url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope=openid%20profile%20email%20offline_access&code_challenge={}&code_challenge_method=S256&state={}&id_token_add_organizations=true&codex_cli_simplified_flow=true&originator=codex_vscode",
        AUTH_URL, CLIENT_ID, REDIRECT_URI, challenge, state_param
    );

    Ok(url)
}

async fn handle_callback(
    Query(params): Query<CallbackParams>,
    State(state): State<OAuthState>,
) -> impl IntoResponse {
    // 1. Exchange Code for Token
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", CLIENT_ID),
        ("code", &params.code),
        ("redirect_uri", REDIRECT_URI),
        ("code_verifier", &state.verifier),
    ];

    let token_res = client
        .post(TOKEN_URL)
        .form(&params)
        .send()
        .await;

    match token_res {
        Ok(res) => {
            if res.status().is_success() {
                if let Ok(token_data) = res.json::<TokenResponse>().await {
                    // Success!
                    
                    // TODO: Extract email from id_token or fetch user profile
                    // For now, use a placeholder or decode id_token if available
                    let email = extract_email_from_id_token(token_data.id_token.as_deref())
                        .unwrap_or_else(|| "openai_user".to_string());

                    let payload = OAuthSuccessPayload {
                        email: email.clone(),
                        access_token: token_data.access_token.clone(),
                        refresh_token: token_data.refresh_token.clone().unwrap_or_default(),
                    };

                    // Emit event to frontend
                    let _ = state.app_handle.emit("openai-oauth-success", payload);
                    
                    // Shutdown server
                    if let Ok(mut tx) = state.shutdown_tx.lock() {
                        if let Some(tx) = tx.take() {
                            let _ = tx.send(());
                        }
                    }

                    return Html("<h1>Login Successful!</h1><p>You can close this window and return to the app.</p><script>window.close()</script>");
                }
            }
            Html("<h1>Login Failed</h1><p>Failed to retrieve tokens.</p>")
        }
        Err(_) => Html("<h1>Login Failed</h1><p>Network error.</p>"),
    }
}

fn generate_verifier() -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
}

fn generate_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier);
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(result)
}

fn extract_email_from_id_token(id_token: Option<&str>) -> Option<String> {
    // Simple unverified decode to get email (security is handled by TLS and backend exchange)
    let token = id_token?;
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    
    // Decode middle part
    let payload_part = parts[1];
    let decoded = URL_SAFE_NO_PAD.decode(payload_part).ok()?;
    let json_str = String::from_utf8(decoded).ok()?;
    let json: serde_json::Value = serde_json::from_str(&json_str).ok()?;
    
    json.get("email").and_then(|v| v.as_str()).map(String::from)
}
