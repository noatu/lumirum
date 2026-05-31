# LumiRum Server

## Running

Run from the **repo root**:

```bash
# Start in the background (includes test data)
docker compose up -d

# Stop the application
docker compose down

# Stop and clear all data (including database)
docker compose down -v
```

To run without test data:
```bash
docker compose build --build-arg SEED_DATA=false
docker compose up -d
```

Then open <http://localhost:3000>

## Development

```bash
# Start only the database
docker compose up database -d

# From server/
source .envrc
cargo run
```
