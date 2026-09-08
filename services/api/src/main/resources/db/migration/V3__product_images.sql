CREATE INDEX IF NOT EXISTS idx_product_images_primary ON product_images(product_id, is_primary) WHERE is_primary = TRUE;
CREATE INDEX IF NOT EXISTS idx_products_seller_status ON products(seller_id, status);
