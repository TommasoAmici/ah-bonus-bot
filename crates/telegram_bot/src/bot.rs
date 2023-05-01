use ah_api::search::search_products;

use clap::Parser;
use sqlx::SqlitePool;
use telegram_bot::db::{insert_product, insert_product_tracking};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile},
    utils::command::BotCommands,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "db-url", default_value = "sqlite:ah_bonus.db")]
    pub db_url: String,
    #[arg(short = 't', long = "token", default_value = "TELEGRAM_BOT_TOKEN")]
    pub token: String,
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

    let bot = Bot::new(args.token);

    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(commands_handler);
    let callback_query_handler = Update::filter_callback_query()
        .endpoint(move |bot, q| callback_query_handler(bot, q, pool.clone()));

    let handler = dptree::entry()
        .branch(command_handler)
        .branch(callback_query_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn callback_query_handler(
    bot: Bot,
    q: CallbackQuery,
    pool: SqlitePool,
) -> ResponseResult<()> {
    let product_id = q.data;
    let chat_id = q.message.unwrap().chat.id;
    match product_id {
        Some(id) => {
            let product_response = ah_api::product::get_product(id.as_str()).await?;
            let product = product_response
                .card
                .products
                .first()
                .expect("No product found");
            let insert_product = insert_product(&pool, product).await;
            if insert_product.is_err() {
                log::error!("Failed to insert product {}", product.id);
                bot.send_message(chat_id, format!("Failed to track {}", product.title))
                    .await?;
                return Ok(());
            }
            let insert_tracking = insert_product_tracking(&pool, product.id, chat_id.0).await;
            if insert_tracking.is_err() {
                log::error!("Failed to insert product {}", product.id);
                bot.send_message(chat_id, format!("Failed to track {}", product.title))
                    .await?;
                return Ok(());
            }
            bot.send_message(
                chat_id,
                format!("Started tracking prices for {}", product.title),
            )
            .await?;
            Ok(())
        }
        None => Ok(()),
    }
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help | Command::Start => help_endpoint(bot, msg).await,
        Command::Search(query) => search_endpoint(bot, msg, &query).await,
    }
}

async fn help_endpoint(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn search_endpoint(bot: Bot, msg: Message, query: &String) -> ResponseResult<()> {
    let search_results = search_products(query, 3).await?;

    for card in search_results.cards {
        let product = card.products.first().unwrap();

        let track_button = InlineKeyboardButton::new(
            "Track",
            teloxide::types::InlineKeyboardButtonKind::CallbackData(product.id.to_string()),
        );
        let inline_keyboard = InlineKeyboardMarkup::default().append_row(vec![track_button]);

        let image_url = product.images.last().unwrap().url.clone();
        bot.send_photo(msg.chat.id, InputFile::url(image_url))
            .caption(format!(
                "{} - €{} {}",
                product.title, product.price.now, product.price.unit_size
            ))
            .reply_markup(inline_keyboard)
            .disable_notification(true)
            .await?;
    }

    Ok(())
}
