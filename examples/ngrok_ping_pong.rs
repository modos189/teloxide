// The version of ngrok ping-pong-bot, which uses a webhook to receive updates
// from Telegram, instead of long polling.

use teloxide::{dispatching::update_listeners::webhooks, prelude::*};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting ngrok ping-pong bot...");

    let bot = Bot::from_env().auto_send();

    let addr = ([127, 0, 0, 1], 8443).into();
    let url = "Your HTTPS ngrok URL here. Get it by `ngrok http 8443`".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |msg: Message, bot: AutoSend<Bot>| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            respond(())
        },
        listener,
    )
    .await;
}
