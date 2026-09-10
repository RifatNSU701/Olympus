ALTER TABLE payments
  ADD COLUMN IF NOT EXISTS provider_reference VARCHAR(160),
  ADD COLUMN IF NOT EXISTS paid_at TIMESTAMPTZ,
  ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

CREATE INDEX IF NOT EXISTS idx_payments_order_status
  ON payments(order_id, status);

CREATE INDEX IF NOT EXISTS idx_payments_provider_reference
  ON payments(provider_reference)
  WHERE provider_reference IS NOT NULL;
