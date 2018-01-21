//! The bot implementation, which will do the game logic and stuff.

extern crate telegram;

use telegram::bot::{Bot, BotBuilder};
use telegram::dialog::{Dialog, Card};

fn main() {
    println!("Hello, world!");

    let mut bot = BotBuilder::new("monstrocity".to_owned())
        .with_config_file("bot.toml")
        .build();


    // this does not compile, of course.
    bot.registerDialog("/start", Dialog::new("start_dialog")
    	.add_card(Card::new("greeting")
    		.build_text(|msg| {
    			format!("Hi, {}!\nWhat is your favorite animal?", msg.from.unwrap().username.unwrap())
    		})
    		.wants_reply(|msg| {
    			if let Some(text) = msg.text {
    				if is_animal(text) {
    					Ok(Reply::ShowCard("reply_card"))
    				} else {
    					Err(Reply::Text("That is not an animal."))
    				}
    			} else {
    				Err(Reply::Text("Please type the species your favorite animal"))
    			}
    		}))
    	.add_card(Card::new("reply_card")
    		.build_text(|msg| {
    			format!("I also like {}s!", msg.text.unwrap()) 
    		}))
    	.root("greeting"));

   	// ---

   	fn registerCommand<F: Fn(Message) -> ()>(&mut self, cmd: &str, callback: F) -> () {
   		self.commands.insert(cmd, Arc::new(callback));
   	}

    bot.run();
}
