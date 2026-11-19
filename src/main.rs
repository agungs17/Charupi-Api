use axum::extract::DefaultBodyLimit;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use charupi_api::{
    config::AppConfig,
    middleware::{
        catch_panic_layer, cors_layer, propagate_request_id_layer, set_request_id_layer,
        trace_layer,
    },
    routes::create_router,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Inisialisasi Structured Logging & Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "charupi_api=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Charupi-Api Server...");

    // 2. Muat Konfigurasi dari Environment (.env)
    let config = AppConfig::from_env()?;
    tracing::info!(
        host = %config.host,
        port = config.port,
        env = %config.environment,
        "Configuration loaded successfully"
    );

    // 3. Inisialisasi Shared Application State
    let state = AppState::new(config.clone());

    // 4. Rakit Router dengan Layer Middleware Fondasi Produksi
    let app = create_router(state)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 1. Proteksi Payload Request Maksimal 10 MB
        .layer(catch_panic_layer())                     // 2. Proteksi Panic Recovery (Server Tidak Pernah Crash)
        .layer(cors_layer())                            // 3. CORS Policy & Header Exposure
        .layer(trace_layer())                           // 4. Structured Logging dengan Request ID
        .layer(propagate_request_id_layer())            // 5. Propagasi Header X-Request-ID ke Klien
        .layer(set_request_id_layer());                 // 6. Generate / Parse X-Request-ID Header Unik

    // 5. Binding TCP Listener
    let addr = config.socket_addr()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on http://{}", addr);

    // 6. Menjalankan Server dengan Graceful Shutdown Listener
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server gracefully stopped");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal. Initiating graceful shutdown...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal. Initiating graceful shutdown...");
        },
    }
}
