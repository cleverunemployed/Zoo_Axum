FROM rust:1.75-slim-bullseye as builder

WORKDIR /app


RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*


COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src


COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app


RUN apt-get update && \
    apt-get install -y ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/Zoo_Axum /usr/local/bin/app


COPY --from=builder /app/migrations ./migrations

COPY .env .env || true

EXPOSE 3000

CMD ["/usr/local/bin/app"]