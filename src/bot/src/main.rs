//! The bot implementation, which will do the game logic and stuff.

extern crate fluent;
extern crate telegram;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::sync::Arc;

use fluent::MessageContext;
use fluent::types::FluentValue;

use telegram::bot::{Bot, BotBuilder};
use telegram::dialog::{Dialog, Card, Reply, Keyboard, Button, KeyboardReply};

// helper function until fluent-rs fixes its shit
fn find_attribute<'i, 'a>(msg: &'a fluent::syntax::ast::Message, id: &'i str) -> Option<&'a fluent::syntax::ast::Attribute> {
    if let Some(ref attributes) = msg.attributes {
        for attribute in attributes {
            if &attribute.id.name == id {
                return Some(attribute);
            }
        }
    }
    None
}

fn main() {
    println!("Hello, world!");

    let mut ctx = MessageContext::new(&["en-US"]);

    {
        let mut f = File::open("localization/monstocity_en-US.ftl").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );
        ctx.add_messages(&contents);
    }

    let ctx_rc = Arc::new(ctx);

    let ctx1 = Arc::clone(&ctx_rc);
    let ctx2 = Arc::clone(&ctx_rc);
    let mut bot = BotBuilder::new("monstrocity".to_owned())
        .with_config_file("bot.toml")
        .register_dialog(
            "start".to_owned(),
            Dialog::new("start", "greeting").add_card(
                Card::new("greeting".to_owned())
                    .build_text(move |msg| {
                        let reply = ctx1.get_message("start-dialog").unwrap();
                        let username = msg.from.as_ref().unwrap().username.as_ref().unwrap();
                        let mut args = HashMap::new();
                        args.insert("username", FluentValue::from(username.clone()));
                        ctx1.format(reply, Some(&args)).unwrap()
                    })
                    .ends_dialog(),
            ),
        )
        .register_dialog(
            "character".to_owned(),
            Dialog::new("character", "character_start").add_card(
                Card::new("character_start".to_owned())
                    .build_text(move |_msg| {
                        // No one has characters lol
                        ctx2.get_message("character-creation-class-selection-dialog")
                            .and_then(|reply| ctx2.format(reply, None))
                            .unwrap()
                    })
                    .with_keyboard({
                        let msg = ctx_rc.get_message("character-creation-class-selection-dialog").unwrap();

                        Keyboard::new("class_selection".to_owned(), |_query| {
                            KeyboardReply::HideKeyboardAndShowCard("character_done".to_owned())
                        })
                            .add_row(&[Button::new(&ctx_rc.format(find_attribute(msg, "button-decker").unwrap(), None).unwrap()).with_callback_data("decker")])
                            .add_row(&[Button::new(&ctx_rc.format(find_attribute(msg, "button-samurai").unwrap(), None).unwrap()).with_callback_data("samurai")])
                            .add_row(&[Button::new(&ctx_rc.format(find_attribute(msg, "button-ninja").unwrap(), None).unwrap()).with_callback_data("ninja")])
                            .add_row(&[Button::new(&ctx_rc.format(find_attribute(msg, "button-engineer").unwrap(), None).unwrap()).with_callback_data("engineer")])
                    })
                    .ends_dialog(),
            )
                .add_card(Card::new("character_done".to_owned())
                    .ends_dialog()),
        )
        .build();

    bot.run();
}
