# LumiRum

## Running
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

Alternatively for development:
```bash
# Run only the database
docker compose up database -d

# Source the development environment variables
source .envrc

# Run the app locally
cargo run
```
