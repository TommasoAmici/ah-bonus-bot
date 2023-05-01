use ah_api::product::get_product;
use clap::Parser;
use sqlx::SqlitePool;
use std::{thread, time};
use telegram_bot::db::{get_all_product_ids, get_discounted_products, insert_product_history};
use teloxide::{
    prelude::*,
    types::{InputFile, ParseMode},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "db-url", default_value = "sqlite:ah_bonus.db")]
    pub db_url: String,
    #[arg(short = 't', long = "token", default_value = "TELEGRAM_BOT_TOKEN")]
    pub token: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    pretty_env_logger::init();

    let pool = SqlitePool::connect(&args.db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    get_current_prices(&pool)
        .await
        .expect("Failed to get current prices");

    notify_users_of_discounts(&pool)
        .await
        .expect("Failed to notify users of discounts");
}

async fn get_current_prices(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Fetching current prices");

    let product_ids = get_all_product_ids(pool).await?;
    let one_sec = time::Duration::from_secs(1);
    for product_id in product_ids {
        log::info!("Getting product with ID: {}", product_id);

        // be respectful to the API
        thread::sleep(one_sec);

        let product = get_product(product_id.to_string().as_str()).await;
        match product {
            Ok(resp) => {
                insert_product_history(
                    pool,
                    &resp
                        .card
                        .products
                        .first()
                        .expect("There should be a product"),
                )
                .await?;
            }
            Err(_) => {
                log::error!("Failed to get product with id {}", product_id);
                continue;
            }
        }
    }
    Ok(())
}

async fn notify_users_of_discounts(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Notifying users of discounts");

    let args = Cli::parse();
    let bot = Bot::new(args.token);

    let to_notify = get_discounted_products(pool).await?;

    for notification in to_notify {
        let message = bot
            .send_photo(
                ChatId {
                    0: notification.chat_id,
                },
                InputFile::url(url::Url::parse(&notification.image_url).unwrap()),
            )
            .caption(notification.message())
            .parse_mode(ParseMode::MarkdownV2)
            .await;
        if message.is_err() {
            log::error!("Failed to send message to {}", notification.chat_id);
        }
    }

    Ok(())
}
