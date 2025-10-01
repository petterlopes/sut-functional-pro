FROM rust:1.84 AS build
WORKDIR /app

# Copy manifest separately to leverage layer caching between builds
COPY api/Cargo.toml api/Cargo.lock ./
COPY api/migrations ./migrations
COPY openapi.yaml /openapi.yaml
COPY api/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/sut-api /app/sut-api
COPY --from=build /app/migrations /app/migrations
ENV RUST_LOG=info
ENV MIGRATIONS_DIR=/app/migrations
CMD ["/app/sut-api"]