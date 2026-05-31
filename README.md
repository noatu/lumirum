# LumiRum

Smart lighting management system. The server exposes a REST API consumed by the Android app, the web app, and the IoT firmware running on ESP32 devices.

- [`server/`](server/) — Rust/Axum REST API with PostgreSQL
- [`website/`](website/) — React/TypeScript web app (Mantine UI, TanStack Query)
- [`mobile/`](mobile/) — Android app (Kotlin, Jetpack Compose)
- [`iot/`](iot/) — ESP32 firmware (PlatformIO/C++)

## Running

Requires [Docker](https://docs.docker.com/get-docker/) and optionally [`just`](https://github.com/casey/just).

```bash
just rebuild      # build images and start everything
just up           # start with existing images
just down         # stop
just down-volumes # stop and wipe the database
```

Without `just`, run the underlying commands directly — see the [`Justfile`](Justfile) for the full list.

The web app is available at <http://localhost:80> and the API at <http://localhost:3000>.

## Development

Run the server and web app locally against a Docker database:

```bash
just server    # starts the database, then runs the server with cargo run
just web       # runs the Vite dev server (proxies API to localhost:3000)
```

## Cleanup

```bash
just clean     # remove build artifacts (target/, node_modules/, .gradle/, .pio/)
```
