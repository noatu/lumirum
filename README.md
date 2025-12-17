# LumiRum

## Running
```bash
# Start in the background
docker compose up -d

# Stop the application
docker compose down

# Stop and clear all data (including database)
docker compose down -v
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
