mod config;

use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Router,
};
use log::info;
use serde::Deserialize;
use std::future::Future;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    url, AuthUrl, ClientId, ClientSecret, CsrfToken, HttpRequest, HttpResponse, PkceCodeChallenge,
    RedirectUrl, RequestTokenError, Scope, StandardErrorResponse, TokenUrl,
};
use oauth2::{AuthorizationCode, PkceCodeVerifier, TokenResponse};

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg = config::load();

    let state = AppState {
        config: cfg.clone(),
        state: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(|| async { "Ok" }))
        .route("/readiness", get(|| async { "Ok" }))
        .route("/login", get(login))
        .route("/oauth2/handler", get(oauth2_handler))
        .with_state(state);

    info!(config:? = &cfg; "starting server...");
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
    let m = state.state.lock().await;

    let verifier = m.get(&query.state.clone()).unwrap();
    let cfg = state.config.auth;

    get_client(cfg)
        .generate_token(query.code.clone(), verifier.clone())
        .await
        .ok()
        .unwrap()
}

fn get_client(cfg: config::AuthConfig) -> OAuthProvider {
    OAuthProvider::new(
        cfg.auth_url,
        cfg.token_url,
        cfg.client_id,
        cfg.client_secret.into(),
        cfg.redirect_url,
    )
}

#[derive(Clone)]
pub struct OAuthProvider {
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

impl OAuthProvider {
    pub fn new(
        auth_url: String,
        token_url: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        OAuthProvider {
            auth_url,
            token_url,
            client_id,
            client_secret,
            redirect_url,
            state: None,
        }
    }

    fn get_client(&self) -> Result<BasicClient, GetClientError> {
        let client_id = ClientId::new(self.client_id.clone());
        let client_secret = ClientSecret::new(self.client_secret.clone());
        let auth_url = AuthUrl::new(self.auth_url.clone()).map_err(GetClientError::ParseAuthURL)?;
        let token_url =
            TokenUrl::new(self.token_url.clone()).map_err(GetClientError::ParseTokenURL)?;

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));

        let client = client.set_redirect_uri(
            RedirectUrl::new(self.redirect_url.clone())
                .map_err(GetClientError::ParseRedirectURL)?,
        );

        Ok(client)
    }

    async fn generate_url<F, Fut>(
        mut self,
        scopes: Vec<String>,
        save: F,
    ) -> Result<Box<Self>, GetClientError>
    where
        F: FnOnce(StateAuth) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let binding = self.get_client()?;
        let (auth_url, csrf_token) = binding
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

    async fn generate_token(
        &self,
        code: String,
        verifier: String,
    ) -> Result<String, GenerateTokenError> {
        let token = self
            .get_client()?
            .exchange_code(AuthorizationCode::new(code.clone()))
            .set_pkce_verifier(PkceCodeVerifier::new(verifier.clone()))
            .request_async(async_http_client)
            .await?;
        Ok(token.access_token().secret().to_string())
    }
}

#[derive(Error, Debug)]
pub enum GetClientError {
    #[error("parse auth url: {0}")]
    ParseAuthURL(url::ParseError),

    #[error("parse token url: {0}")]
    ParseTokenURL(url::ParseError),

    #[error("parse redirect url: {0}")]
    ParseRedirectURL(url::ParseError),
}

#[derive(Error, Debug)]
pub enum GenerateTokenError {
    #[error("get client")]
    GetClient(#[from] GetClientError),

    #[error("request generate token endpoint")]
    Request(
        #[from] RequestTokenError<reqwest::Error, StandardErrorResponse<BasicErrorResponseType>>,
    ),
}

pub async fn async_http_client(request: HttpRequest) -> Result<HttpResponse, reqwest::Error> {
    let client = {
        let builder = reqwest::Client::builder();

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
