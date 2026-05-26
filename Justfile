dev:
    @echo "Starting API (port 8000) and App (port 3000)..."
    @trap 'kill 0' EXIT; \
    (just dev-api) & \
    echo "Waiting for API to be ready..." && \
    until curl -s -o /dev/null http://localhost:8000/api/media 2>/dev/null; do sleep 1; done && \
    echo "API ready, starting frontend..." && \
    just dev-app

dev-api:
    cd api && npx wrangler dev --port 8000

dev-app:
    cd app && trunk serve --port 3000

build:
    cd api && cargo build --target wasm32-unknown-unknown --release
    cd app && trunk build --release

deploy:
    cd app && trunk build --release
    cd api && npx wrangler deploy

d1-query query='SELECT name FROM sqlite_master WHERE type="table"':
    cd api && npx wrangler d1 execute photon-db --local --command "{{query}}"
