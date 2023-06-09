# GPTelegram
Introducing a Rust-based ChatGPT-Telegram bot integration that delivers seamless, fluid conversation output. Engage in natural language chat with this bot, powered by state-of-the-art AI and designed for effortless communication

# Demo
![image](https://github.com/RevAtN/GPTelegram/blob/main/demo.gif)

# Run

## Mac/Linux
```
export OPENAI_API_KEY=sk-...
export TELOXIDE_TOKEN=123...:abc...
export ALLOWED_USER_ID=123...

cargo run
```
## Win(space is needed)
```
$Env:OPENAI_API_KEY = "sk-..."
$Env:TELOXIDE_TOKEN = "123...:abc..."
$Env:ALLOWED_USER_ID = "123..."

cargo run
```


# TODO

- [ ] Add payment function
- [ ] Multiple Users
- [ ] Long Term Chat
- [x] More Error Handling
- [x] User management
- [x] Streaming Output on TG Bot
