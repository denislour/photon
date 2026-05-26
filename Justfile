dev:
    @echo "=== Photon Dev Server ==="
    @echo "Pre-building BE (WASM debug)..."
    @cd api && cargo build --target wasm32-unknown-unknown 2>&1 | tail -1
    @echo "Starting API (port 8000) and App (port 3000)..."
    @trap 'kill 0' EXIT; \
    (cd api && wrangler dev --port 8000) & \
    echo "Waiting for API to be ready..." && \
    until curl -s -o /dev/null http://localhost:8000/api/media 2>/dev/null; do sleep 1; done && \
    echo "API ready, starting frontend..." && \
    cd app && trunk serve --port 3000

dev-api:
    cd api && wrangler dev --port 8000

dev-app:
    cd app && trunk serve --port 3000

build:
    cd api && cargo build --target wasm32-unknown-unknown --release
    cd app && trunk build --release

deploy:
    cd api && wrangler deploy
    cd app && trunk build --release

d1-query query='SELECT name FROM sqlite_master WHERE type="table"':
    cd api && wrangler d1 execute photon-db --local --command "{{query}}"
