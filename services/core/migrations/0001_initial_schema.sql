CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR(320) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  full_name VARCHAR(160) NOT NULL,
  phone VARCHAR(32),
  role VARCHAR(32) NOT NULL CHECK (role IN ('BUYER','SELLER','ADMIN','SUPER_ADMIN')),
  status VARCHAR(32) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE','SUSPENDED','PENDING')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS seller_profiles (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  store_name VARCHAR(160) NOT NULL,
  description TEXT,
  verified BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(120) NOT NULL UNIQUE,
  slug VARCHAR(140) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS products (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  seller_id UUID NOT NULL REFERENCES users(id),
  category_id UUID REFERENCES categories(id),
  name VARCHAR(240) NOT NULL,
  slug VARCHAR(260) NOT NULL UNIQUE,
  description TEXT,
  price NUMERIC(19,4) NOT NULL CHECK (price >= 0),
  stock INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0),
  status VARCHAR(32) NOT NULL DEFAULT 'DRAFT' CHECK (status IN ('DRAFT','ACTIVE','ARCHIVED')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_products_seller ON products(seller_id);
CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_status ON products(status);
CREATE INDEX IF NOT EXISTS idx_products_seller_status ON products(seller_id, status);

CREATE TABLE IF NOT EXISTS product_images (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  storage_key VARCHAR(500) NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS carts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), buyer_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS cart_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), cart_id UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
  product_id UUID NOT NULL REFERENCES products(id), quantity INTEGER NOT NULL CHECK (quantity > 0), UNIQUE(cart_id, product_id)
);
CREATE TABLE IF NOT EXISTS orders (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), buyer_id UUID NOT NULL REFERENCES users(id), status VARCHAR(32) NOT NULL DEFAULT 'PENDING_PAYMENT',
  currency CHAR(3) NOT NULL DEFAULT 'BDT', subtotal NUMERIC(19,4) NOT NULL CHECK (subtotal >= 0), shipping_amount NUMERIC(19,4) NOT NULL DEFAULT 0 CHECK (shipping_amount >= 0),
  tax_amount NUMERIC(19,4) NOT NULL DEFAULT 0 CHECK (tax_amount >= 0), total_amount NUMERIC(19,4) NOT NULL CHECK (total_amount >= 0),
  idempotency_key VARCHAR(120) UNIQUE, created_at TIMESTAMPTZ NOT NULL DEFAULT now(), updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS order_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  product_id UUID NOT NULL REFERENCES products(id), seller_id UUID NOT NULL REFERENCES users(id), quantity INTEGER NOT NULL CHECK (quantity > 0), unit_price NUMERIC(19,4) NOT NULL CHECK (unit_price >= 0)
);
CREATE TABLE IF NOT EXISTS payments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), order_id UUID NOT NULL UNIQUE REFERENCES orders(id), provider VARCHAR(64) NOT NULL,
  provider_transaction_id VARCHAR(160), amount NUMERIC(19,4) NOT NULL CHECK (amount >= 0), status VARCHAR(32) NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS wallets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  balance NUMERIC(19,4) NOT NULL DEFAULT 0 CHECK (balance >= 0), currency CHAR(3) NOT NULL DEFAULT 'BDT'
);
CREATE TABLE IF NOT EXISTS wallet_transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), wallet_id UUID NOT NULL REFERENCES wallets(id), amount NUMERIC(19,4) NOT NULL,
  transaction_type VARCHAR(32) NOT NULL, reference_id UUID, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS audit_logs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(), actor_id UUID REFERENCES users(id), action VARCHAR(120) NOT NULL,
  entity_type VARCHAR(80) NOT NULL, entity_id UUID, metadata JSONB NOT NULL DEFAULT '{}'::jsonb, created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_audit_actor_created ON audit_logs(actor_id, created_at DESC);
