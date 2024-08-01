use anyhow::Result;
use api::{private, public};
use auth::backend::AuthBackend;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_login::AuthManagerLayerBuilder;
use database::setup::init_db;
use http::HeaderValue;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer};
use tower_sessions::{
    cookie::{Key, SameSite},
    Expiry, MemoryStore, SessionManagerLayer,
};
use utils::shutdown_signal;

mod api;
mod auth;
mod database;
mod errors;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server.");

    // PostgreSQL connection pool.
    let db_pool = init_db().await?;

    // Session layer as a request extension.
    let session_store = MemoryStore::default();

    // Generates a singing/encryption key for the cookies.
    let key = Key::generate();

    // Session layer as a request extension.
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(time::Duration::minutes(30)))
        .with_same_site(SameSite::Strict)
        .with_secure(true)
        .with_private(key);

    // Authentication backend.
    let backend = AuthBackend::new(&db_pool);

    // Authentication layer.
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // Add CORS header to responses.
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
        .max_age(Duration::from_secs(3600));

    // Compresses response bodies.
    let compression = CompressionLayer::new();

    // Limit the size of request bodies to 1 MB.
    let request_size = RequestBodyLimitLayer::new(1024 * 1024);

    // Rate limiter
    let rate_limiter = Arc::new(GovernorConfigBuilder::default().finish().unwrap());

    // Routes that require authentication.
    // The custom middleware runs before these handlers.
    let protected_routes = Router::new()
        .route("/protected", get(private::example_protected_handler))
        .route("/logout", post(private::logout_handler))
        .route_layer(middleware::from_fn(api::middleware::check_auth));

    // Routes that are public.
    let public_routes = Router::new()
        .route("/", get(public::example_endpoint_handler))
        .route("/start", get(public::start_handler))
        .route("/login", post(public::login_handler))
        .route("/register", post(public::register_handler));

    // Router application.
    let app = Router::new()
        .nest("/", public_routes)
        .nest("/", protected_routes)
        .layer(request_size)
        .layer(auth_layer)
        .layer(cors)
        .layer(compression)
        .layer(GovernorLayer {
            config: rate_limiter,
        })
        .with_state(db_pool);

    // Socket address.
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // TCP listener.
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();

    tracing::info!("Listening on: {}", socket_addr);

    // Serve the application with hyper.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    Ok(())
}
