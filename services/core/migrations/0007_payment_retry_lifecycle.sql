-- Allow a buyer to retry a failed payment while preventing
-- multiple active payment attempts for the same order.
ALTER TABLE payments
    DROP CONSTRAINT IF EXISTS payments_order_id_key;

CREATE UNIQUE INDEX IF NOT EXISTS payments_one_active_attempt_per_order_uidx
    ON payments (order_id)
    WHERE status IN ('PENDING', 'PAID');

CREATE INDEX IF NOT EXISTS idx_payments_order_created
    ON payments (order_id, created_at DESC);
