FROM rust:1.88-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/olympus-core /usr/local/bin/olympus-core

ENV RUST_LOG=olympus_core=info,tower_http=info
EXPOSE 8080

USER 65532:65532
ENTRYPOINT ["/usr/local/bin/olympus-core"]
