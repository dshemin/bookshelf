mod config;

<<<<<<< Updated upstream
use axum::{Router, routing::get};
||||||| Stash base
use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
=======
use axum::{
    Router,
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
>>>>>>> Stashed changes
use log::info;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

<<<<<<< Updated upstream
||||||| Stash base
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    url, AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, HttpResponse, PkceCodeChallenge,
    RedirectUrl, RequestTokenError, Scope, StandardErrorResponse, TokenUrl,
};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

=======
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, HttpResponse, PkceCodeChallenge,
    RedirectUrl, RequestTokenError, Scope, StandardErrorResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType},
    url,
};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

>>>>>>> Stashed changes
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cfg = config::load()?;

    let state = AppState {
        config: cfg.clone(),
        state: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .with_state(state);

    info!(config:? = &cfg; "starting server...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
    state: Arc<Mutex<HashMap<String, String>>>,
}
