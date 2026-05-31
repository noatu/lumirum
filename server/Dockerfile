FROM lukemathwalker/cargo-chef:latest-rust-1.90.0-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ARG SEED_DATA=true
RUN if [ "$SEED_DATA" != "true" ]; then rm migrations/1_test-data.sql; fi
RUN cargo build --release --bin lumirum-server

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/lumirum-server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/lumirum-server"]
