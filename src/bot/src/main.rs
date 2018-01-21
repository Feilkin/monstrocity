//! The bot implementation, which will do the game logic and stuff.

extern crate telegram;

use telegram::bot::{Bot, BotBuilder};
use telegram::dialog::{Dialog, Card, Reply};

fn is_animal(name: &str) -> bool {
    match name {
        "dog" | "cat" | "horse" => true,
        _ => false,
    }
}

fn main() {
    println!("Hello, world!");

    let mut bot = BotBuilder::new("monstrocity".to_owned())
        .with_config_file("bot.toml")
        .register_dialog(
            "start".to_owned(),
            Dialog::new("start_dialog".to_owned(), "greeting".to_owned())
                .add_card(
                    Card::new("greeting".to_owned())
                        .build_text(|msg| {
                            format!(
                                "Hi, {}!\nWhat is your favorite animal?",
                                msg.from.as_ref().unwrap().username.as_ref().unwrap()
                            )
                        })
                        .wants_reply(|msg| if let Some(ref text) = msg.text {
                            if is_animal(text) {
                                Ok(Reply::ShowCard("reply_card".to_owned()))
                            } else {
                                Err(Reply::Text("That is not an animal.".to_owned()))
                            }
                        } else {
                            Err(Reply::Text(
                                "Please type the species your favorite animal".to_owned(),
                            ))
                        }),
                )
                .add_card(
                    Card::new("reply_card".to_owned())
                        .build_text(|msg| {
                            format!("I also like {}s!", msg.text.as_ref().unwrap())
                        })
                        .ends_dialog(),
                ),
        )
        .build();

    bot.run();
}
