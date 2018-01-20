//! The bot implementation, which will do the game logic and stuff.

extern crate telegram;

use telegram::bot::{Bot, BotBuilder};

fn main() {
    println!("Hello, world!");

    let mut bot = BotBuilder::new("monstrocity".to_owned())
        .with_config_file("bot.toml")
        .build();

    bot.run();
}
