-- Persist the delivery address captured during checkout.
-- Nullable for compatibility with orders created before this migration.

ALTER TABLE orders
    ADD COLUMN IF NOT EXISTS shipping_address VARCHAR(500);
