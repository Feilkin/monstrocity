//! The bot implementation, which will do the game logic and stuff.

extern crate telegram;

use telegram::bot::{Bot, BotBuilder};
use telegram::dialog::{Dialog, Card, Reply, Keyboard, Button, KeyboardReply};

fn main() {
    println!("Hello, world!");

    let mut bot = BotBuilder::new("monstrocity".to_owned())
        .with_config_file("bot.toml")
        .register_dialog(
            "start".to_owned(),
            Dialog::new("start".to_owned(), "greeting".to_owned())
                .add_card(
                    Card::new("greeting".to_owned())
                        .build_text(|msg| {
                            format!(
                                "Hi, {}!\nWhat is your favorite animal?\n\n_Hint: try _`dog`_._",
                                msg.from.as_ref().unwrap().username.as_ref().unwrap()
                            )
                        })
                        .with_keyboard(
                            Keyboard::new("fav_animal".to_owned(), |query| {
                                KeyboardReply::HideKeyboardAndShowCard("reply_card".to_owned())
                            }).add_row(&[Button::new("Dog".to_owned()).with_callback_data("dog".to_owned())])
                                .add_row(&[Button::new("Cat".to_owned()).with_callback_data("cat".to_owned())])
                                .add_row(&[Button::new("Ziltoid".to_owned()).with_callback_data("puppet".to_owned())]),
                        ),
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
