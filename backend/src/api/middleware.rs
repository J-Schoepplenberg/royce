use crate::auth::backend::AuthSession;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use http::StatusCode;

/// Example: custom middleware that runs before request handlers.
///
/// If the request is not authenticated, return a 401 Unauthorized status code.
/// Otherwise, continue to the request handler.
pub async fn check_auth(
    auth: AuthSession,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    auth.user.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(next.run(request).await)
}
