ALTER TABLE products ADD COLUMN IF NOT EXISTS reserved_stock INTEGER NOT NULL DEFAULT 0;
ALTER TABLE products ADD CONSTRAINT products_reserved_stock_check CHECK (reserved_stock >= 0 AND reserved_stock <= stock);
CREATE INDEX IF NOT EXISTS idx_products_created_at ON products(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_products_active_category ON products(category_id, created_at DESC) WHERE status = 'ACTIVE';
ALTER TABLE product_images ADD CONSTRAINT product_images_sort_order_check CHECK (sort_order >= 0);
CREATE INDEX IF NOT EXISTS idx_product_images_product_sort ON product_images(product_id, sort_order);
