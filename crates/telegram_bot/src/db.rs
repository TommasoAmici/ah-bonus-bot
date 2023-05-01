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

pub async fn insert_product_tracking(
    pool: &SqlitePool,
    product_id: i64,
    chat_id: i64,
) -> Result<SqliteQueryResult, Error> {
    sqlx::query_file!("src/queries/insert_tracking.sql", product_id, chat_id)
        .execute(pool)
        .await
}
