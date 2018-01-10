
extern crate env_logger;
extern crate telegram;

use std::env;

use telegram::{Bot, BotBuilder};

fn display_help(msg: telegram::objects::Message) -> () {
    msg.reply("Please, go away");
}

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let config_file = match env::var("BOT_CONFIG_FILE") {
        Ok(val) => val,
        Err(_) => "bot.toml".to_owned(),
    };
    // sketching out what I want the bot to look like

    let bot = Bot::new(&config);

    let sender = bot.get_sender();
    let start_sender = sender.clone();
    bot.register_command("start", move |msg| {
        let reply = msg.reply("Hello, this is dog.");
        start_sender.send(reply).unwrap();
    })

    bot.run();
}
