#!/bin/bash

cleanup() {
    echo "Cleaning up Docker containers..."
    docker compose -f docker-compose.yml down
}

trap cleanup EXIT

docker compose -f docker-compose.yml up -d

sleep 5

echo "Starting Todo app..."

# cargo run -- list # -> json

# cargo run --features "postgres" --no-default-features # -> postgres

cargo run --features "mongodb" --no-default-features # -> mongodb

echo "Todo app finished."