use std::{collections::HashSet, str::FromStr};

use ah_api::search::search_products;

use clap::Parser;
use sqlx::SqlitePool;
use telegram_bot::db;
use teloxide::{
    adaptors::{throttle::Limits, Throttle},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile},
    utils::command::BotCommands,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "db-url", default_value = "sqlite:ah_bonus.db")]
    pub db_url: String,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Display help message.")]
    Help,
    #[command(description = "Search for a product.")]
    Search(String),
    #[command(
        description = "Get a list of all products you're tracking. The command also allows you to stop tracking a product."
    )]
    List,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    pretty_env_logger::init();
    log::info!("Starting AH Bonus bot...");

    let pool = SqlitePool::connect(&args.db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let bot = Bot::from_env().throttle(Limits::default());
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set commands");

    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(commands_handler);
    let callback_query_handler = Update::filter_callback_query().endpoint(callback_query_handler);

    let handler = dptree::entry()
        .branch(command_handler)
        .branch(callback_query_handler);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

enum Action {
    TrackProduct = 0,
    StopTrackingProduct = 1,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Action::TrackProduct),
            "1" => Ok(Action::StopTrackingProduct),
            _ => Err(()),
        }
    }
}

async fn callback_query_handler(
    bot: Throttle<Bot>,
    q: CallbackQuery,
    pool: SqlitePool,
) -> ResponseResult<()> {
    let text = q.data;
    if text.is_none() {
        return Ok(());
    }
    let text = text.unwrap();

    let message = q.message;
    if message.is_none() {
        return Ok(());
    }
    let message = message.unwrap();

    let (action, product_id) = text.split_once(":").expect("Invalid callback query");
    let parsed_action: Action = action.parse::<Action>().expect("Invalid action");

    match parsed_action {
        Action::TrackProduct => track_product(&bot, &message, &pool, product_id).await,
        Action::StopTrackingProduct => {
            stop_tracking_product(&bot, &message, &pool, product_id).await
        }
    }
}

fn create_track_keyboard(product_id: i64) -> InlineKeyboardMarkup {
    let button = InlineKeyboardButton::new(
        "Track",
        teloxide::types::InlineKeyboardButtonKind::CallbackData(format!(
            "{}:{}",
            Action::TrackProduct as u8,
            product_id
        )),
    );
    InlineKeyboardMarkup::default().append_row(vec![button])
}

fn create_stop_track_keyboard(product_id: i64) -> InlineKeyboardMarkup {
    let button = InlineKeyboardButton::new(
        "Stop tracking",
        teloxide::types::InlineKeyboardButtonKind::CallbackData(format!(
            "{}:{}",
            Action::StopTrackingProduct as u8,
            product_id
        )),
    );
    InlineKeyboardMarkup::default().append_row(vec![button])
}

async fn track_product(
    bot: &Throttle<Bot>,
    msg: &Message,
    pool: &SqlitePool,
    product_id: &str,
) -> ResponseResult<()> {
    let chat_id = &msg.chat.id;
    log::info!(
        "start tracking: product_id={} chat_id={}",
        product_id,
        chat_id.0
    );

    let product_response = ah_api::product::get_product(product_id).await?;
    let product = product_response
        .card
        .products
        .first()
        .expect("No product found");

    let insert = db::insert_product(&pool, product).await;
    if insert.is_err() {
        match insert.err().unwrap() {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    log::info!("Product {} already exists in database", product.id);
                } else {
                    log::error!("Failed to insert product {}. Error: {}", product.id, e);
                    bot.send_message(*chat_id, format!("Failed to track {}", product.title))
                        .await?;
                    return Ok(());
                }
            }
            e => {
                log::error!("Failed to insert product {}. Error: {}", product.id, e);
                bot.send_message(*chat_id, format!("Failed to track {}", product.title))
                    .await?;
                return Ok(());
            }
        }
    }

    let insert_tracking = db::insert_product_tracking(&pool, product.id, chat_id.0).await;
    if insert_tracking.is_err() {
        match insert_tracking.err().unwrap() {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    bot.send_message(*chat_id, "Already tracking this product")
                        .await?;
                }
            }
            _ => {
                log::error!("Failed to insert product {}", product.id);
                bot.send_message(*chat_id, format!("Failed to track {}", product.title))
                    .await?;
            }
        }
        return Ok(());
    }

    let keyboard = create_stop_track_keyboard(product.id);
    bot.edit_message_reply_markup(*chat_id, msg.id)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn stop_tracking_product(
    bot: &Throttle<Bot>,
    msg: &Message,
    pool: &SqlitePool,
    product_id: &str,
) -> ResponseResult<()> {
    let chat_id = &msg.chat.id;
    let parsed_product_id = product_id.parse::<i64>().expect("Invalid product id");
    log::info!(
        "stop tracking: product_id={} chat_id={}",
        product_id,
        chat_id.0
    );

    let delete = db::delete_product_tracking(&pool, parsed_product_id, chat_id.0).await;
    match delete {
        Ok(_) => {
            let keyboard = create_track_keyboard(parsed_product_id);
            bot.edit_message_reply_markup(*chat_id, msg.id)
                .reply_markup(keyboard)
                .await?;
        }
        Err(e) => {
            log::error!("Failed to delete product tracking {}", e);
            bot.send_message(*chat_id, "Failed to stop tracking product")
                .await?;
        }
    }

    Ok(())
}

async fn commands_handler(
    bot: Throttle<Bot>,
    msg: Message,
    cmd: Command,
    pool: SqlitePool,
) -> ResponseResult<()> {
    match cmd {
        Command::Help | Command::Start => help_endpoint(bot, msg).await,
        Command::Search(query) => search_endpoint(bot, msg, &pool, &query).await,
        Command::List => list_endpoint(bot, msg, &pool).await,
    }
}

async fn help_endpoint(bot: Throttle<Bot>, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn list_endpoint(bot: Throttle<Bot>, msg: Message, pool: &SqlitePool) -> ResponseResult<()> {
    let tracked_products = db::get_all_tracked_products(pool, msg.chat.id.0).await;
    match tracked_products {
        Ok(products) => {
            if products.len() == 0 {
                bot.send_message(msg.chat.id, "No tracked products").await?;
                return Ok(());
            }

            for product in products {
                let keyboard = create_stop_track_keyboard(product.id);

                let image_url = url::Url::parse(&product.image_url).unwrap();
                bot.send_photo(msg.chat.id, InputFile::url(image_url))
                    .caption(product.name)
                    .reply_markup(keyboard)
                    .disable_notification(true)
                    .await?;
            }
        }
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                "Failed to retrieve list of tracked products, try again later",
            )
            .await?;
        }
    }
    Ok(())
}

async fn search_endpoint(
    bot: Throttle<Bot>,
    msg: Message,
    pool: &SqlitePool,
    query: &String,
) -> ResponseResult<()> {
    log::info!("search: query={}", query);
    let search_results = search_products(query, 3).await?;

    let tracked_products = db::get_all_tracked_products_ids(pool, msg.chat.id.0)
        .await
        .unwrap_or_default();
    let tracked_products_set = tracked_products.into_iter().collect::<HashSet<_>>();

    for card in search_results.cards {
        let product = card.products.first().unwrap();

        let keyboard = if tracked_products_set.contains(&product.id) {
            create_stop_track_keyboard(product.id)
        } else {
            create_track_keyboard(product.id)
        };

        let image_url = product.images.last().unwrap().url.clone();
        bot.send_photo(msg.chat.id, InputFile::url(image_url))
            .caption(format!(
                "{} - €{} {}",
                product.title, product.price.now, product.price.unit_size
            ))
            .reply_markup(keyboard)
            .disable_notification(true)
            .await?;
    }

    Ok(())
}
