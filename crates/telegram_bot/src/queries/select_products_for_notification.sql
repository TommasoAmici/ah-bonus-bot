SELECT DISTINCT ph.product_id,
  ph.price,
  ph.discount AS "discount!: String",
  ph.discount_start_date > DATE('now') AS "future_discount",
  ph.discount_start_date AS "discount_start_date!: Date",
  ph.discount_end_date AS "discount_end_date!: Date",
  tp.chat_id,
  p.name,
  p.url,
  p.image_url
FROM products_history ph
  JOIN tracked_products tp ON ph.product_id = tp.product_id
  JOIN products p ON ph.product_id = p.id
WHERE ph.discount_end_date >= DATE('now')
