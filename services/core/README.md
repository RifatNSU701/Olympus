# Olympus Core

The Rust/Axum core service is the high-performance application layer for Olympus.

## Responsibilities

- Public and authenticated marketplace APIs
- Authentication and authorization
- Product, inventory, cart, order and payment workflows
- Strongly typed domain and request validation
- Structured logging and HTTP tracing

## Local development

```bash
cargo run --manifest-path services/core/Cargo.toml
```

The initial health endpoint is available at `GET /health` on port `8080`.

Database credentials must be supplied through environment variables; secrets are never committed to the repository.
