FROM rust:1.88-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/olympus-core /usr/local/bin/olympus-core

ENV RUST_LOG=olympus_core=info,tower_http=info
EXPOSE 8080

HEALTHCHECK --interval=15s --timeout=5s --start-period=20s --retries=5 \
  CMD curl -fsS http://127.0.0.1:8080/health/live || exit 1

USER 65532:65532
ENTRYPOINT ["/usr/local/bin/olympus-core"]
