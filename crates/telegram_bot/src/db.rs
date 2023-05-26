use ah_api::product::Product;

use sqlx::{sqlite::SqliteQueryResult, Error, SqlitePool};
use teloxide::utils::markdown::escape;
use time::Date;

pub async fn insert_product(
    pool: &SqlitePool,
    product: &Product,
) -> Result<SqliteQueryResult, Error> {
    let image_url = product
        .images
        .last()
        .expect("There should be an image")
        .url
        .to_string();

    sqlx::query_file!(
        "src/queries/insert_product.sql",
        product.id,
        product.title,
        product.brand,
        product.price.unit_size,
        product.link,
        image_url,
    )
    .execute(pool)
    .await
}

pub async fn insert_product_history(
    pool: &SqlitePool,
    product: &Product,
) -> Result<SqliteQueryResult, Error> {
    let price = product.get_price_for_db();
    let discount_text = product.get_discount_text();
    let (start, end) = match product.discount {
        None => (None, None),
        Some(ref discount) => (
            Some(discount.start_date.clone()),
            Some(discount.end_date.clone()),
        ),
    };
    sqlx::query_file!(
        "src/queries/insert_product_history.sql",
        product.id,
        price,
        discount_text,
        start,
        end,
    )
    .execute(pool)
    .await
}

pub async fn insert_product_tracking(
    pool: &SqlitePool,
    product_id: i64,
    chat_id: i64,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query_file!("src/queries/insert_tracking.sql", product_id, chat_id)
        .execute(pool)
        .await
}

pub async fn delete_product_tracking(
    pool: &SqlitePool,
    product_id: i64,
    chat_id: i64,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query_file!(
        "src/queries/delete_product_tracking.sql",
        product_id,
        chat_id
    )
    .execute(pool)
    .await
}

pub async fn get_all_product_ids(pool: &SqlitePool) -> Result<Vec<i64>, Error> {
    sqlx::query_scalar!("SELECT id FROM products")
        .fetch_all(pool)
        .await
}

/// Returns a list of product IDs that are tracked by the given chat ID.
pub async fn get_all_tracked_products_ids(
    pool: &SqlitePool,
    chat_id: i64,
) -> Result<Vec<i64>, Error> {
    sqlx::query_scalar!(
        "SELECT product_id FROM tracked_products WHERE chat_id = ?",
        chat_id
    )
    .fetch_all(pool)
    .await
}

pub struct TrackedProduct {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub image_url: String,
}

pub async fn get_all_tracked_products(
    pool: &SqlitePool,
    chat_id: i64,
) -> Result<Vec<TrackedProduct>, Error> {
    sqlx::query_file_as!(
        TrackedProduct,
        "src/queries/select_tracked_products.sql",
        chat_id
    )
    .fetch_all(pool)
    .await
}

pub struct NotificationDiscount {
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub product_id: i64,
    pub discount: String,
    pub future_discount: i32,
    pub discount_start_date: time::Date,
    pub discount_end_date: time::Date,
    pub price: i64,
    pub chat_id: i64,
}

impl NotificationDiscount {
    /// Returns a markdown formatted message for the Telegram bot.
    pub fn message(&self) -> String {
        if self.future_discount == 1 {
            format!(
                "[{}](https://www.ah.nl{}) will be on discount from {} to {}: {}",
                escape(self.name.as_str()),
                self.url,
                escape(self.discount_start_date.to_string().as_str()),
                escape(self.discount_end_date.to_string().as_str()),
                escape(self.discount.as_str())
            )
        } else {
            format!(
                "[{}](https://www.ah.nl{}) is now on discount: {}",
                escape(self.name.as_str()),
                self.url,
                escape(self.discount.as_str())
            )
        }
    }
}

pub async fn get_discounted_products(
    pool: &SqlitePool,
) -> Result<Vec<NotificationDiscount>, Error> {
    sqlx::query_file_as!(
        NotificationDiscount,
        "src/queries/select_products_for_notification.sql"
    )
    .fetch_all(pool)
    .await
}

/// Returns a list of chat IDs that track products that are not on discount.
/// This is used to send a message to the user that none of the products they
/// track are on discount.
pub async fn get_users_not_notified(pool: &SqlitePool) -> Result<Vec<i64>, Error> {
    sqlx::query_file_scalar!("src/queries/select_users_not_notified.sql")
        .fetch_all(pool)
        .await
}
