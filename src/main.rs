
extern crate env_logger;
extern crate telegram;

use std::env;

use telegram::bot::Bot;
use telegram::dispatcher::ASyncDispatcher;

fn main() {
    env_logger::init().unwrap();
    println!("Hello, world!");

    let config_file = match env::var("BOT_CONFIG_FILE") {
        Ok(val) => val,
        Err(_) => "bot.toml".to_owned(),
    };
    // sketching out what I want the bot to look like

    let bot = <Bot>::new(&config_file);

    bot.run();
}
