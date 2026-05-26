dev:
    @echo "Starting API (port 8787) and App (port 8080)..."
    @trap 'kill 0' EXIT; \
    (cd api && npx wrangler dev) & \
    echo "Waiting for API to be ready..." && \
    until curl -s -o /dev/null http://localhost:8787/api/media 2>/dev/null; do sleep 1; done && \
    echo "API ready, starting frontend..." && \
    (cd app && trunk serve)

dev-api:
    cd api && npx wrangler dev

dev-app:
    cd app && trunk serve

build:
    cd api && cargo build --target wasm32-unknown-unknown --release
    cd app && trunk build --release

deploy:
    cd app && trunk build --release
    cd api && npx wrangler deploy

d1-query:
    cd api && npx wrangler d1 execute photon-db --local --command "{{query}}"
