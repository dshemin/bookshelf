use actix_web::{post, HttpResponse, Responder};
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::AppState;
use tracing::{debug, error};

#[post("")]
pub async fn sync(state: AppState, claim: StandardKeycloakClaims) -> impl Responder {
    // actix_web_middleware_keycloak_auth use old "uuid" create, so we have to
    // convert it.
    let user_id = match uuid::Uuid::parse_str(&claim.sub.to_string()) {
        Ok(v) => v,
        Err(e) => {
            error!(err=e.to_string(), "failed to convert uuid from old version to new");
            return HttpResponse::InternalServerError();
        },
    };

    debug!(user_id=user_id.to_string(), "sync user");

    let res = state.user_services.sync.sync(user_id).await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!(err=e.to_string(), "failed to sync user to our database");
            HttpResponse::InternalServerError()
        },
    }
}
