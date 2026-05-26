dev:
    @echo "Starting API (8787) and App (8080)..."
    @trap 'kill 0' EXIT; \
    (cd api && npx wrangler dev) & \
    sleep 12 && \
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
