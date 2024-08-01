use super::responses::StoreApiResponse;
use crate::{
    auth::backend::{AuthSession, Credentials},
    database::{
        models::user::{NewUser, User, UserLimited},
        setup::ConnectionPool,
    },
};
use askama_axum::IntoResponse;
use axum::{extract::State, response, Json};
use http::StatusCode;

/// Example of a public endpoint.
pub async fn example_endpoint_handler() -> impl IntoResponse {
    response::Json("Hello, World!").into_response()
}

/// Handle a registration request.
pub async fn register_handler(
    State(db_pool): State<ConnectionPool>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    let user = UserLimited::new(new_user.username.clone());
    return match User::register(new_user, db_pool).await {
        Ok(_) => response::Json(StoreApiResponse::new(true, Some(user))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
}

/// Handle a login request.
pub async fn login_handler(
    mut auth: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    let res = match auth.authenticate(creds).await {
        Ok(Some(user)) => auth
            .login(&user)
            .await
            .map(|_| response::Json(StoreApiResponse::new(true, Some(user.into()))))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    println!("res: {:#?}", res);
    res
}

/// Handle a startup request.
pub async fn start_handler(auth: AuthSession) -> impl IntoResponse {
    response::Json(StoreApiResponse::new(
        auth.user.is_some(),
        auth.user.map(|user| user.into()),
    ))
    .into_response()
}
