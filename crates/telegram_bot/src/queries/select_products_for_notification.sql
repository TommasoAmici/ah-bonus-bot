SELECT DISTINCT ph.product_id,
  ph.price,
  ph.discount AS "discount!: String",
  tp.chat_id,
  p.name,
  p.url,
  p.image_url
FROM products_history ph
  JOIN tracked_products tp ON ph.product_id = tp.product_id
  JOIN products p ON ph.product_id = p.id
WHERE ph.created_at > DATETIME('now', '-1 day')
  AND ph.discount IS NOT NULL
