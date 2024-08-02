use super::responses::StoreApiResponse;
use crate::auth::backend::AuthSession;
use axum::response::{self, IntoResponse};
use http::StatusCode;

/// Example endpoint: can only be accessed by authenticated users.
pub async fn example_protected_handler() -> impl IntoResponse {
    response::Json("Protected route").into_response()
}

/// Handle a logout request.
pub async fn logout_handler(mut auth: AuthSession) -> impl IntoResponse {
    match auth.logout().await {
        Ok(_) => response::Json(StoreApiResponse::new(false, None)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
