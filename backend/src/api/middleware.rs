use crate::auth::backend::AuthSession;
use askama_axum::IntoResponse;
use axum::{extract::Request, middleware::Next};
use http::StatusCode;

/// Example of custom middleware to check if the user is authenticated.
/// 
/// Runs before the request handler.
/// 
/// If the request is not authenticated, return a 401 Unauthorized status code.
/// Otherwise, continue to the request handler.
pub async fn check_auth(
    auth: AuthSession,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    println!("request: {:#?}", request);
    println!("auth: {:#?}", auth);

    auth.user.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(next.run(request).await)
}
