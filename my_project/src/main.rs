use teloxide::{prelude::*, utils::command::BotCommands, types::ParseMode};
use std::env;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Старт.")]
    Start,
    #[command(description = "Помощь.")]
    Help,
    #[command(description = "конвертация валюты.")]
    Convert { command_body: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {

    match cmd {
        Command::Start => bot.send_message(msg.chat.id, "Это бот для конвертации валют!\n\nДля получения большей информации - /help").await?,
        Command::Help => {
            let help = "Помощь по использованию бота:\n
/help - вывод этого сообщения.\n
<b><u>Запрос:</u></b>
/convert СУММА ИЗ_ВАЛЮТЫ В_ВАЛЮТУ
<b><u>Пример:</u></b>
/convert 128 RUB USD";
            
            bot.send_message(msg.chat.id, help).parse_mode(ParseMode::Html).await?
        },
        Command::Convert { command_body } => {
            let command_body: Vec<&str> = command_body.split(' ').collect();
            if command_body.len() != 3 {
                bot.send_message(msg.chat.id, "Неверное количество параметров команды!").await?
            } else {
                let api_key = env::var("EXCHANGE_RATES_API_KEY").expect("EXCHANGE_RATES_API_KEY not set");
                let from_currency = command_body[1].to_uppercase();
                let to_currency = command_body[2].to_uppercase();
                let amount_str = command_body[0].replace(",", ".");

                let amount = match amount_str.parse::<f64>() {
                    Ok(parsed_amount) => parsed_amount,
                    Err(_) => {
                        bot.send_message(
                            msg.chat.id,
                            "Ошибка ввода суммы"
                        ).await?;
                        return Ok(());
                    }
                };
                let url = format!(
                    "https://v6.exchangerate-api.com/v6/{}/latest/{}",
                    api_key, from_currency
                );
                let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
                let conversion_rates = response["conversion_rates"].as_object();

                //Проверка корректности валюты
                if let Some(rates) = conversion_rates {
                    if !rates.contains_key(&to_currency) {
                        bot.send_message(
                            msg.chat.id,
                            format!("Некорректная валюта: {}", to_currency)
                        ).await?;
                        return Ok(());
                    }
                } else {
                    bot.send_message(
                        msg.chat.id,
                        format!("Некорректная валюта: {}", from_currency)
                    ).await?;
                    return Ok(());
                }

                let rate = response["conversion_rates"][&to_currency].as_f64().unwrap();

                bot.send_message(
                    msg.chat.id,
                    format!(
                        "1 {} равняется {:.2} {}\n{} {} равняется {:.2} {}",
                        from_currency, rate, to_currency,
                        amount, from_currency, amount * rate, to_currency
                    ),
                )
                .await?
            }
        }
    };

    Ok(())
}