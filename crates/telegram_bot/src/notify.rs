use ah_api::product::get_product;
use clap::Parser;
use sqlx::SqlitePool;
use std::{thread, time};
use telegram_bot::db;
use teloxide::{
    adaptors::throttle::Limits,
    prelude::*,
    types::{InputFile, ParseMode},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "db-url", default_value = "sqlite:ah_bonus.db")]
    pub db_url: String,
    #[arg(long = "dry-run")]
    pub dry_run: bool,
    #[arg(long = "no-fetch")]
    pub no_fetch: bool,
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

    if !args.no_fetch {
        get_current_prices(&pool)
            .await
            .expect("Failed to get current prices");
    }

    notify_users_of_discounts(&pool, args.dry_run)
        .await
        .expect("Failed to notify users of discounts");
}

async fn get_current_prices(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Fetching current prices");

    let product_ids = db::get_all_product_ids(pool).await?;
    let one_sec = time::Duration::from_secs(1);
    for product_id in product_ids {
        log::info!("Getting product with ID: {}", product_id);

        // be respectful to the API
        thread::sleep(one_sec);

        let product = get_product(product_id.to_string().as_str()).await;
        match product {
            Ok(resp) => {
                db::insert_product_history(
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

async fn notify_users_of_discounts(pool: &SqlitePool, dry_run: bool) -> Result<(), sqlx::Error> {
    log::info!("Notifying users of discounts");

    let bot = Bot::from_env().throttle(Limits::default());

    let to_notify = db::get_discounted_products(pool).await?;

    for notification in to_notify {
        if dry_run {
            log::info!(
                "Would have sent message to {}. Message: {}",
                notification.chat_id,
                notification.message()
            );
            continue;
        }

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
            log::error!(
                "Failed to send message to {}. Error: {}",
                notification.chat_id,
                message.err().unwrap().to_string()
            );
        }
    }

    let users_not_notified = db::get_users_not_notified(pool).await?;
    for user in users_not_notified {
        let msg = "None of the products you are tracking are on sale this week";
        if dry_run {
            log::info!("Would have sent message to {}. Message: {}", user, msg);
            continue;
        }
        let message = bot
            .send_message(ChatId { 0: user }, msg)
            .parse_mode(ParseMode::MarkdownV2)
            .await;
        if message.is_err() {
            log::error!(
                "Failed to send message to {}. Error: {}",
                user,
                message.err().unwrap().to_string()
            );
        }
    }

    Ok(())
}
