SELECT DISTINCT tp.chat_id
FROM tracked_products tp
WHERE tp.chat_id NOT IN (
    SELECT tp.chat_id
    FROM products_history ph
      JOIN tracked_products tp ON ph.product_id = tp.product_id
    WHERE ph.discount_end_date >= DATE('now')
  );
