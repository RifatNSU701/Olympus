-- Scope checkout idempotency keys to the buyer who supplied them.
-- A client must not be able to replay another buyer's key and receive that order.

ALTER TABLE orders
    DROP CONSTRAINT IF EXISTS orders_idempotency_key_key;

CREATE UNIQUE INDEX IF NOT EXISTS orders_buyer_id_idempotency_key_uidx
    ON orders (buyer_id, idempotency_key)
    WHERE idempotency_key IS NOT NULL;
