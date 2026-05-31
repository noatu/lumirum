default:
    @just --list

# Start all services in Docker
up replicas="1":
    docker compose up -d --scale server={{replicas}}

# Build Docker images
build:
    docker compose build

# Rebuild images and start all services
rebuild replicas="1":
    docker compose up --build -d --scale server={{replicas}}

# Stop all services
down:
    docker compose down

# Stop all services and remove volumes
down-volumes:
    docker compose down -v

# Start only the database (for local dev)
up-dev:
    docker compose up database -d

# Run the server locally against the Docker database
server: up-dev
    cd server && source .envrc && cargo run

# Run the website dev server. Needs just server
web:
    cd website && npm run dev

# Run Locust load tests against the API gateway
locust:
    locust -f locust/locustfile.py --host http://localhost:3000

# Remove build artifacts and caches for all components
clean:
    rm -rf server/target
    rm -rf website/node_modules website/dist
    rm -rf mobile/.gradle mobile/build mobile/app/build
    rm -rf iot/.pio
