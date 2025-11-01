mod config;

use async_trait::async_trait;
use axum::{
    Router,
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use serde::Deserialize;
use std::future::Future;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, HttpResponse, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl, basic::BasicClient,
};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

#[tokio::main]
async fn main() {
    let cfg = config::load();

    println!("config: {:?}", cfg);

    let state = AppState {
        config: cfg,
        state: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .route("/login", get(login))
        .route("/oauth2/handler", get(oauth2_handler))
        .with_state(state);

    println!("Starting server...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    config: config::Config,
    state: Arc<Mutex<HashMap<String, String>>>,
}

async fn login(State(state): State<AppState>) -> Redirect {
    println!("login");
    let state_oauth = get_client(state.config.auth)
        .generate_url(
            vec![
                "email".to_string(),
                "profile".to_string(),
                "openid".to_string(),
            ],
            |state_e| async move {
                let mut st = state.state.lock().await;
                st.insert(state_e.state, state_e.verifier);
            },
        )
        .await
        .ok()
        .unwrap()
        .state
        .unwrap();

    let auth_url = state_oauth.url_generated.unwrap();

    Redirect::temporary(&auth_url)
}

#[derive(Clone, Deserialize)]
pub struct OAuth2Callback {
    pub code: String,
    pub state: String,
}

async fn oauth2_handler(State(state): State<AppState>, query: Query<OAuth2Callback>) -> String {
    println!("oauth2_handler");
    let m = state.state.lock().await;

    let verifier = m.get(&query.state.clone()).unwrap();
    let cfg = state.config.auth;

    get_client(cfg)
        .generate_token(query.code.clone(), verifier.clone())
        .await
        .ok()
        .unwrap()
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

#[derive(Clone)]
pub struct CustomProvider {
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub state: Option<StateAuth>,
}

#[derive(Clone)]
pub enum MethodExecute {
    DB,
    MEMORY,
}

#[derive(Clone, Debug)]
pub struct StateAuth {
    pub url_generated: Option<String>,
    pub state: String,
    pub verifier: String,
}

impl CustomProvider {
    pub fn new(
        auth_url: String,
        token_url: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        CustomProvider {
            auth_url,
            token_url,
            client_id,
            client_secret,
            redirect_url,
            state: None,
        }
    }
}

/// OAuthClient is the main struct of the lib, it will handle all the connection with the provider
#[async_trait]
pub trait OAuthClient {
    fn get_client(&self) -> Result<BasicClient, ()>;

    /// Get fields data from generated URL
    /// # Return
    /// StateAuth - The state, verifier and url_generated
    fn get_state(&self) -> Option<StateAuth>;

    /// Generate the URL to redirect the user to the provider
    /// # Arguments
    /// * `scopes` - Vec<String> - The scopes that you want to access in the provider
    /// * `save` - F - The function that will use to save your state in the db/memory
    async fn generate_url<F, Fut>(mut self, scopes: Vec<String>, save: F) -> Result<Box<Self>, ()>
    where
        F: FnOnce(StateAuth) -> Fut + Send,
        Fut: Future<Output = ()> + Send;

    /// Generate the token from the code and verifier
    /// # Arguments
    /// * `code` - String - The code that the provider will return after the user accept the auth
    /// * `verifier` - String - The verifier that was generated in the first step
    /// # Return
    /// The token generated
    async fn generate_token(&self, code: String, verifier: String) -> Result<String, ()>;
}

#[async_trait]
impl OAuthClient for CustomProvider {
    fn get_client(&self) -> Result<BasicClient, ()> {
        Ok(BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            AuthUrl::new(self.auth_url.clone()).unwrap(),
            Some(TokenUrl::new(self.token_url.clone()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(self.redirect_url.clone()).unwrap()))
    }

    fn get_state(&self) -> Option<StateAuth> {
        self.state.clone()
    }

    async fn generate_url<F, Fut>(mut self, scopes: Vec<String>, save: F) -> Result<Box<Self>, ()>
    where
        F: FnOnce(StateAuth) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let binding = self.get_client();
        let (auth_url, csrf_token) = binding?
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter().map(Scope::new).collect::<Vec<Scope>>())
            .set_pkce_challenge(pkce_challenge)
            .url();

        let state = StateAuth {
            url_generated: Some(auth_url.to_string()),
            state: csrf_token.secret().to_string(),
            verifier: pkce_verifier.secret().to_string(),
        };

        self.state = Some(state.clone());
        save(state).await;

        Ok(Box::new(self.clone()))
    }

    async fn generate_token(&self, code: String, verifier: String) -> Result<String, ()> {
        let token = self
            .get_client()?
            .exchange_code(AuthorizationCode::new(code.clone()))
            .set_pkce_verifier(PkceCodeVerifier::new(verifier.clone()))
            .request_async(async_http_client)
            .await
            .unwrap();
        Ok(token.access_token().secret().to_string())
    }
}

pub async fn async_http_client(request: HttpRequest) -> Result<HttpResponse, reqwest::Error> {
    let client = {
        let builder = reqwest::Client::builder();

        // Following redirects opens the client up to SSRF vulnerabilities.
        // but this is not possible to prevent on wasm targets
        #[cfg(not(target_arch = "wasm32"))]
        let builder = builder.redirect(reqwest::redirect::Policy::none());
        let builder = builder.danger_accept_invalid_certs(true);

        builder.build()?
    };

    let mut request_builder = client
        .request(request.method, request.url.as_str())
        .body(request.body);
    for (name, value) in &request.headers {
        request_builder = request_builder.header(name.as_str(), value.as_bytes());
    }
    let request = request_builder.build()?;

    let response = client.execute(request).await?;

    let status_code = response.status();
    let headers = response.headers().to_owned();
    let chunks = response.bytes().await?;
    Ok(HttpResponse {
        status_code,
        headers,
        body: chunks.to_vec(),
    })
}
