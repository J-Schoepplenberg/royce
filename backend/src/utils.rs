/// Waits for a shutdown signal from the OS.
pub async fn shutdown_signal() {
    // Wait on Ctrl+C signal asynchrounously.
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // Wait on terminate signal asynchrounously.
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    // Wait on terminate signal asynchrounously.
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for either signal future to complete.
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, commencing shutdown.");
}
