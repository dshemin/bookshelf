mod config;

use axum::{Router, extract::State, routing::get};
use oauth_axum::{CustomProvider, OAuthClient};

#[tokio::main]
async fn main() {
    let cfg = config::load();

    let state = AppState { config: cfg };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .route("/login", get(login))
        .with_state(state);

    println!("Starting server...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
}

async fn login(State(state): State<AppState>) -> String {
    let state_oauth = get_client(state.config.auth)
        .generate_url(vec!["*".to_string()], |state_e| async move {
            println!("{:?}", state_e);
        })
        .await
        .ok()
        .unwrap()
        .state
        .unwrap();

    state_oauth.url_generated.unwrap()
}

fn get_client(cfg: config::AuthConfig) -> CustomProvider {
    CustomProvider::new(
        cfg.auth_url,
        cfg.token_url,
        cfg.client_id,
        cfg.client_secret,
        cfg.redirect_url,
    )
}
