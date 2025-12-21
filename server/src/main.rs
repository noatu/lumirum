use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use std::process::exit;

mod database;
mod errors;
mod extractors;
mod responses;
mod router;
mod features {
    pub mod auth;
    pub mod devices;
    pub mod profiles;
    pub mod system;
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool, // pool cloning is cheap
    jwt_secret: String,
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
        "failed to setup the database pool",
    );
    tracing::info!("database pool is up");

    let jwt_secret = exit_on_error(
        std::env::var("JWT_SECRET"),
        "could not get JWT_SECRET environment variable",
    );

    let state = AppState { pool, jwt_secret };
    let router = router::router().with_state(state);

    // Support for `systemfd --no-pid -s http::3000 -- cargo watch -x run`
    let listener = if let Some(listener) = exit_on_error(
        ListenFd::from_env().take_tcp_listener(0),
        "failed to read from listenfd",
    ) {
        exit_on_error(
            listener.set_nonblocking(true),
            "failed to set listener to non-blocking",
        );
        exit_on_error(
            TcpListener::from_std(listener),
            "failed to convert std listener to tokio listener",
        )
    } else {
        let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        exit_on_error(
            TcpListener::bind(&addr).await,
            &format!("failed to bind to address '{addr}'"),
        )
    };

    match listener.local_addr() {
        Ok(addr) => tracing::info!("listening on: {addr}"),
        Err(err) => tracing::warn!("failed to get local address from listener: {err}"),
    }

    exit_on_error(
        axum::serve(listener, router).await,
        "server exited with error",
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
