// v0.0.4 streaming telegram bot, 每次streaming数据都刷新一次，速度很慢，有一点bug
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tokio::time::sleep;

async fn fetch_chat_gpt_output(
    bot: &Bot,
    chat_id: ChatId,
    sent_message: &Message,
    user_message: &str,
) -> Result<(), reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let api_url = "https://api.openai.com/v1/chat/completions";

    println!("user_message: {}", user_message);

    let payload = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": user_message}],
        "temperature": 1.0,
        "top_p": 1.0,
        "n": 1,
        "stream": true,
        "presence_penalty": 0,
        "frequency_penalty": 0
    });

    let client = Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut output = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        println!("chunk: {:?}", chunk);
        let mut utf8_str = String::from_utf8_lossy(&chunk).to_string();

        let trimmed_str = utf8_str.trim_start_matches("data: ");
        let json_result: Result<Value, _> = serde_json::from_str(trimmed_str);
        match json_result {
            Ok(json) => {
                if let Some(choices) = json.get("choices") {
                    if let Some(choice) = choices.get(0) {
                        if let Some(content) =
                            choice.get("delta").and_then(|delta| delta.get("content"))
                        {
                            if let Some(content_str) = content.as_str() {
                                println!("output: {}", content_str);
                                let content_str = content_str.trim_start_matches('\n');
                                if content_str.trim().is_empty() {
                                    // Skip this iteration if the content_str only contains whitespace characters
                                    continue;
                                }
                                output.push_str(content_str);
                                let tmp = format!("{}...✍️", output);
                                bot.send_chat_action(chat_id, ChatAction::Typing)
                                    .await
                                    .unwrap();
                                bot.edit_message_text(chat_id, sent_message.id, &tmp)
                                    .await
                                    .unwrap();
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(())
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
    if let Err(e) = fetch_chat_gpt_output(&bot, chat_id, &sent_message, &user_message).await {
        eprintln!("Error fetching GPT output: {:?}", e);
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