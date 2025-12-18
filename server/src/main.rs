use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use std::process::exit;

mod database;
mod endpoints;

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool, // pool cloning is cheap
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| concat!(env!("CARGO_CRATE_NAME"), "=debug,info").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@database:5432/data".into());
    tracing::debug!(database_url);

    let pool = exit_on_error(
        database::setup_pool(&database_url).await,
        "Failed to setup the database pool",
    );
    tracing::info!("database pool is up");

    let state = AppState { pool };
    let router = endpoints::router().with_state(state);

    // Support for `systemfd --no-pid -s http::3000 -- cargo watch -x run`
    let listener = if let Some(listener) = exit_on_error(
        ListenFd::from_env().take_tcp_listener(0),
        "Failed to read from listenfd",
    ) {
        exit_on_error(
            listener.set_nonblocking(true),
            "Failed to set listener to non-blocking",
        );
        exit_on_error(
            TcpListener::from_std(listener),
            "Failed to convert std listener to tokio listener",
        )
    } else {
        let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        exit_on_error(
            TcpListener::bind(&addr).await,
            &format!("Failed to bind to address '{addr}'"),
        )
    };

    match listener.local_addr() {
        Ok(addr) => tracing::info!("listening on: {addr}"),
        Err(err) => tracing::warn!("Failed to get local address from listener: {err}"),
    }

    exit_on_error(
        axum::serve(listener, router).await,
        "Server exited with error",
    );
}

/// Helper to exit on error with logging
fn exit_on_error<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> T {
    match result {
        Ok(val) => val,
        Err(err) => {
            tracing::error!("{context}: {err}");
            exit(1);
        }
    }
}
