// v0.0.3 tg bot与openai api的交互，有一些bug，还不能流式输出
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::ChatAction;
use tokio::time::sleep;

async fn fetch_chat_gpt_output(user_message: &str) -> Result<String, reqwest::Error> {
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
                                output.push_str(content_str);
                                println!("output: {}", output);
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }

    Ok(output)
}

async fn handle_message(bot: Bot, msg: Message) {
    let chat_id = msg.chat.id;
    let user_message = msg.text().unwrap_or_default();

    let api_output = match fetch_chat_gpt_output(&user_message).await {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error fetching GPT output: {:?}", e);
            return;
        }
    };

    let text_chunks = api_output.lines().collect::<Vec<&str>>();

    // Send the initial message
    bot
        .send_chat_action(chat_id, ChatAction::Typing)
        .await
        .unwrap();
    println!("!: {:?}", &*text_chunks[0]);
    let sent_message = bot.send_message(chat_id, &*text_chunks[0]).await.unwrap();

    // Edit the message with the new text chunks
    for chunk in text_chunks.iter().skip(1) {
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
