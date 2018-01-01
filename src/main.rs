
extern crate env_logger;
extern crate telegram;

use std::env;

use telegram::Bot;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let config_file = match env::var("BOT_CONFIG_FILE") {
        Ok(val) => val,
        Err(_) => "bot.toml".to_owned(),
    };

    let bot = Bot::new(&config_file);

    bot.run();
}
