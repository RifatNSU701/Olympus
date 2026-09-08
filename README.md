# HIVE v2

> A premium, production-oriented multi-vendor marketplace platform.

HIVE v2 is the modernization of the original HIVE e-commerce project. The legacy Java/Swing implementation remains historical; the new platform is built as a secure web application with a modular backend, PostgreSQL persistence, and a premium responsive frontend.

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
Object Storage / Redis / Payment Providers
```

## Repository

- `apps/web` — storefront and dashboards
- `services/api` — Spring Boot REST API
- `database` — PostgreSQL migrations
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
4. Order/payment/inventory changes are auditable and idempotent.
5. Tests, linting and security checks are part of CI.
6. The application starts as a modular monolith; services are extracted only when justified by scale.

## Local development

Requirements: Java 21+, Node.js 22+, Docker.

```bash
docker compose up -d db
cd services/api
./mvnw spring-boot:run
```

## Status

HIVE v2 foundation is under active development. Production readiness is a release gate, not a claim made by the presence of this scaffold.
