
extern crate env_logger;
extern crate telegram;

use std::env;

use telegram::bot::Bot;
use telegram::dialog::{Dialog, Card, Reply};
use telegram::dispatcher::ASyncDispatcher;

fn check_password(text: &str) -> bool {
    match text {
        "abba" => true,
        _ => false,
    }
}

fn main() {
    env_logger::init().unwrap();
    println!("Hello, world!");

    let config_file = match env::var("BOT_CONFIG_FILE") {
        Ok(val) => val,
        Err(_) => "bot.toml".to_owned(),
    };

    let mut bot = <Bot>::new(&config_file);

    bot.register_command_dialog(
        "start".to_owned(),
        Dialog::new("start_dialog".to_owned())
            .add_card(
                Card::new("get_name".to_owned())
                    .text("Hello please give password.".to_owned())
                    .wants_reply(|msg| if let Some(ref text) = msg.text {
                        if check_password(text) {
                            Ok(Reply::ShowCard("say_hello".to_owned()))
                        } else {
                            Err(Reply::Text("Wrong password!".to_owned()))
                        }
                    } else {
                        Err(Reply::Text("Please give password.".to_owned()))
                    }),
            )
            .add_card(Card::new("say_hello".to_owned()).build_text(|msg| {
                format!("Password {} accepted!\n\nWelcome in.", msg.text.unwrap())
            })),
    );

    bot.run();
}
