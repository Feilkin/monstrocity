//! Worker thread, which do the actual work

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::{time, thread};

use redis;
use redis::Commands;
use regex::Regex;
use reqwest;
use serde::Serialize;
use serde_json;
use serde_json::value::Value;

use dialog::{Dialog, Reply};
use objects::UpdateKind;
use methods::{Method, SendMessage};
use config::Config;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

fn parse_command(text: &str) -> Option<String> {
    lazy_static! {
        static ref CMD_PATTERN: Regex = Regex::new(r"^/([[:alpha:]]+)(@[[:alpha:]]+)?").unwrap();
    }

    if CMD_PATTERN.is_match(text) {
        let captures = CMD_PATTERN.captures(text).unwrap();
        Some(captures[1].to_owned())
    } else {
        None
    }
}

impl Worker {
    pub fn new(id: usize, config: Arc<Config>, dialogs: Arc<HashMap<String, Dialog>>) -> Worker {
        let thread = thread::spawn(move || {
            let redis_client = redis::Client::open(config.redis_address.as_str()).unwrap();
            let con = redis_client.get_connection().unwrap();
            let client = reqwest::Client::new();

            loop {
                match con.lpop::<&str, String>("TestQueue") {
                    Ok(update_data) => {
                        let update = serde_json::from_str::<UpdateKind>(&update_data).unwrap();
                        match update {
                            UpdateKind::Message { update_id, message } => {
                                // First, check if user has ongoing Dialog
                                match con.hget::<&str, &str, String>(
                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                    "DialogID",
                                ) {
                                    Ok(dialog_id) => {
                                        // TODO, we have a ongoing dialog
                                        println!("User had dialog: {}", dialog_id);
                                        if let Some(dialog) = dialogs.get(&dialog_id) {
                                            // get the current card
                                            let card_id: String =
                                                con.hget(
                                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                                    "CardID",
                                                ).unwrap();
                                            let card = dialog.get_card(&card_id);

                                            println!("Current Card is: {}", card_id);

                                            if let Some(reply) = card.check_reply(&message) {
                                                match reply {
                                                    Ok(reply) => {
                                                        // Advance the dialog
                                                        match reply {
                                                            Reply::ShowCard(card_id) => {
                                                                let card = dialog.get_card(&card_id);

                                                // reply to the user with the outputs of the card
                                                let reply = message.reply(card.get_text(&message));

                                                let addr = format!(
                                                    "https://api.telegram.org/bot{}/{}",
                                                    &config.auth_token,
                                                    reply.method
                                                );
                                                let mut req = client.post(&addr);
                                                req.json(&reply.params);
                                                let resp = req.send().unwrap();

                                                // store dialog state to redis
                                                if card.ends_dialog {
                                                    con.del::<_, ()>(&format!("Chat:{:?}:Dialog", message.chat.id)).unwrap();
                                                } else {
                                                    con.hset::<_, _, _, ()>(
                                                        &format!("Chat:{:?}:Dialog", message.chat.id),
                                                        "CardID",
                                                        &card.id,
                                                    ).unwrap();
                                                }
                                                            },
                                                            _ => {},
                                                        }
                                                    }
                                                    Err(reply) => {
                                                        // Display error to user
                                                        match reply {
                                                            Reply::Text(text) => {
                                                                let reply = message.reply(text);

                                                                let addr = format!(
                                                                    "https://api.telegram.org/bot{}/{}",
                                                                    &config.auth_token,
                                                                    reply.method
                                                                );
                                                                let mut req = client.post(&addr);
                                                                req.json(&reply.params);
                                                                let resp = req.send().unwrap();
                                                            },
                                                            _ => {}
                                                        }
                                                    }
                                                }
                                            } else {
                                                con.del::<_, ()>(&format!("Chat:{:?}:Dialog", message.chat.id)).unwrap();
                                            }
                                        } else {
                                            // Invalid DialogID in database, drop the dialog
                                            con.del::<_, ()>(&format!("Chat:{:?}:Dialog", message.chat.id)).unwrap();
                                        }
                                    }
                                    Err(_) => {
                                        // Parse the text for command
                                        let text = match message.text {
                                            Some(ref text) => text,
                                            None => "",
                                        };
                                        if let Some(cmd) = parse_command(text) {
                                            println!("!! Got command: {}", cmd);
                                            if let Some(dialog) = dialogs.get(&cmd) {
                                                // User requested a dialog, start it
                                                // get the first card
                                                let root = dialog.get_root();

                                                // reply to the user with the outputs of the card
                                                let reply = message.reply(root.get_text(&message));

                                                let addr = format!(
                                                    "https://api.telegram.org/bot{}/{}",
                                                    &config.auth_token,
                                                    reply.method
                                                );
                                                let mut req = client.post(&addr);
                                                req.json(&reply.params);
                                                let resp = req.send().unwrap();

                                                // store dialog state to redis
                                                con.hset::<_, _, _, ()>(
                                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                                    "DialogID",
                                                    &dialog.id,
                                                ).unwrap();
                                                con.hset::<_, _, _, ()>(
                                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                                    "CardID",
                                                    &root.id,
                                                ).unwrap();
                                            }
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    Err(_) => thread::sleep(time::Duration::from_millis(10)),
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
