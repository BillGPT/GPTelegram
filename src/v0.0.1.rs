// v0.0.1 简单的tgbot流式输出实现
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tokio::time::sleep;

async fn handle_message(bot: Bot, msg: Message) {
    let chat_id = msg.chat.id;

    let text_chunks = vec![
        "Hello",
        "Hello!",
        "Hello! How",
        "Hello! How can",
        "Hello! How can I",
        "Hello! How can I assist",
        "Hello! How can I assist you",
        "Hello! How can I assist you today",
    ];

    // Send the initial message
    bot.send_chat_action(chat_id, ChatAction::Typing).await.unwrap();
    let sent_message = bot.send_message(chat_id, &*text_chunks[0]).await.unwrap();

    // Edit the message with the new text chunks
    for chunk in text_chunks.iter().skip(1) {
        //sleep(Duration::from_millis(100)).await;
        bot.send_chat_action(chat_id, ChatAction::Typing).await.unwrap();
        bot.edit_message_text(chat_id, sent_message.id, &**chunk)
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting chatgpt-tgbot-rust...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| {
        async move {
            if msg.text().is_some() {
                handle_message(bot, msg).await;
            }
            ResponseResult::<()>::Ok(())
        }
    })
    .await;
}
