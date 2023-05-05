SELECT p.id,
  p.name,
  p.url,
  p.image_url
FROM products p
  JOIN tracked_products tp ON p.id = tp.product_id
WHERE chat_id = ?1
