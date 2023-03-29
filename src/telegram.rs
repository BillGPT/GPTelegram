use crate::gpt;
use teloxide::prelude::*;
use teloxide::types::{ChatId, Message};
use teloxide::types::ChatAction;

pub async fn init_telegram_bot(bot: Bot) {
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

async fn handle_message(bot: Bot, msg: Message) {
    let chat_id = msg.chat.id;
    let user_message = msg.text().unwrap_or_default();

    // Send the initial message
    bot
        .send_chat_action(chat_id, ChatAction::Typing)
        .await
        .unwrap();
    let sent_message = bot.send_message(chat_id, "...✍️").await.unwrap();

    // Fetch and update the message with the GPT output
    if let Err(e) = gpt::fetch_chat_gpt_output(&bot, chat_id, &sent_message, &user_message).await {
        eprintln!("Error fetching GPT output: {:?}", e);
    }
}
