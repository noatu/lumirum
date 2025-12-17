use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

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

    let pool = database::setup_pool(&database_url)
        .await
        .expect("Failed to setup the database pool");
    tracing::info!("database pool is up");

    let state = AppState { pool };

    let router = endpoints::router().with_state(state);

    // Support for `systemfd --no-pid -s http::3000 -- cargo watch -x run`
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => {
            let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
            TcpListener::bind(&addr).await.unwrap()
        }
    };
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
