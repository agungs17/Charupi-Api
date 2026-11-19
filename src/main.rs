use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use charupi_api::{
    config::AppConfig,
    db::{create_pool, run_migrations},
    middleware::{cors_layer, trace_layer},
    routes::create_router,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Inisialisasi Structured Logging & Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "charupi_api=debug,tower_http=info,sqlx=warn".into()),
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

    // 3. Inisialisasi Database Connection Pool & Jalankan Migrasi
    let db_pool = create_pool(&config).await?;
    run_migrations(&db_pool).await?;

    // 4. Inisialisasi Client HTTP Teroptimasi (Connection Pooling)
    let http_client = reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .tcp_nodelay(true)
        .timeout(Duration::from_secs(config.omniroute_timeout_secs))
        .build()?;

    // 5. Inisialisasi Shared Application State
    let state = AppState::new(db_pool.clone(), http_client, config.clone());

    // 6. Rakit Router dengan Layer Middleware Terstandarisasi
    let app = create_router(state)
        .layer(trace_layer())
        .layer(cors_layer())
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // Batas payload 10MB
        .layer(TimeoutLayer::new(Duration::from_secs(180)));

    // 7. Binding TCP Listener
    let addr = config.socket_addr()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on http://{}", addr);

    // 8. Menjalankan Server dengan Graceful Shutdown Listener
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
