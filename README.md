# Olympus

> A premium, production-oriented multi-vendor marketplace platform.

Olympus is the next-generation evolution of the original e-commerce project in this repository. The legacy Java/Swing implementation is retained as historical reference; Olympus is being rebuilt as a secure, scalable web platform with a modular backend, PostgreSQL persistence, and a premium responsive experience.

## Architecture

```text
Next.js + TypeScript
        |
      HTTPS
        |
Spring Boot API
        |
PostgreSQL
        |
Redis / Object Storage / Payment Providers
        |
AI Services
```

## Repository

- `apps/web` — premium storefront and dashboards
- `services/api` — Spring Boot REST API
- `database` — database and migration assets
- `infra` — infrastructure and deployment configuration
- `docs` — architecture, security, API and operations documentation

## Core product areas

- Buyer storefront, search, cart and checkout
- Seller portal, inventory and analytics
- Admin operations and audit logging
- Secure authentication and role-based authorization
- Orders, payments, wallets and transactions
- Reviews, wishlists, coupons and notifications
- AI-ready recommendation and fraud-detection boundaries

## Engineering principles

1. Server-side authorization is mandatory; UI controls are never security boundaries.
2. Passwords are hashed; secrets are never committed.
3. Money uses decimal database types and transactional service logic.
4. Order, payment and inventory changes are auditable and idempotent.
5. Tests, linting and security checks are part of CI.
6. The application starts as a modular monolith; services are extracted only when justified by scale.
7. Production readiness is earned through testing, security review, observability and deployment validation.

## Local development

Requirements: Java 21+, Node.js 22+, Docker.

```bash
docker compose up -d db
cd services/api
./mvnw spring-boot:run
```

API health check:

```text
GET /api/v1/health
```

## Status

Olympus is under active development. The current branch establishes the production foundation; production readiness is a release gate, not a claim made by the scaffold.
