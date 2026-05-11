# Multi-stage build for roadmap-cli
FROM rust:1.83-slim AS builder

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build actual application
COPY . .
RUN touch src/main.rs && cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/roadmap-cli /usr/local/bin/roadmap

EXPOSE 7878

ENV DATABASE_URL=postgres://admin:secret@postgres:5432/roadmap
ENV JWT_SECRET=change-me-in-production

CMD ["roadmap", "serve", "--port", "7878"]
