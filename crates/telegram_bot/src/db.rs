use ah_api::product::Product;

use sqlx::{sqlite::SqliteQueryResult, Error, SqlitePool};

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
    let discount = product.get_discount();
    sqlx::query_file!(
        "src/queries/insert_product_history.sql",
        product.id,
        price,
        discount
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

pub struct NotificationDiscount {
    pub name: String,
    pub url: String,
    pub image_url: String,
    pub product_id: i64,
    pub discount: String,
    pub price: i64,
    pub chat_id: i64,
}

impl NotificationDiscount {
    /// Returns a markdown formatted message for the Telegram bot.
    pub fn message(&self) -> String {
        format!(
            "[{}](https://www.ah.nl{}) is now on discount: {}",
            self.name, self.url, self.discount
        )
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
