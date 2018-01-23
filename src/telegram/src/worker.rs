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
use objects::{Message, UpdateKind};
use methods::{Method, SendMessage};
use config::Config;

// TODO: can I have these here?

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


struct MethodSender {
    client: reqwest::Client,
    auth_token: String,
}

impl MethodSender {
    fn new(config: &Config) -> MethodSender {
        let client = reqwest::Client::new();

        MethodSender {
            client: client,
            auth_token: config.auth_token.clone(),
        }
    }

    fn send<P: Serialize>(&self, method: Method<P>) -> reqwest::Result<reqwest::Response> {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.auth_token,
            method.method
        );
        let mut req = self.client.post(&addr);
        req.json(&method.params);
        req.send()
    }
}

fn start_dialog(
    message: &Message,
    dialog: &Dialog,
    con: &redis::Connection,
    sender: &MethodSender,
) -> () {
    // get the first card
    let root = dialog.get_root();

    // reply to the user with the outputs of the card
    let mut reply = message.reply(root.get_text(&message));
    if let Some(keyboard) = root.get_keyboard() {
        reply = reply.with_keyboard(keyboard);
    }
    println!("Sending {:?}", reply);

    let resp = sender.send(reply).unwrap();

    println!("Got Repsonse {:?}", resp);

    // store dialog state to redis
    if !root.ends_dialog {
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

fn update_dialog(
    message: &Message,
    dialog: &Dialog,
    con: &redis::Connection,
    sender: &MethodSender,
) -> () {
    // get the current card
    let card_id: String = con.hget(&format!("Chat:{:?}:Dialog", message.chat.id), "CardID")
        .unwrap();
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
                        let resp = sender.send(reply).unwrap();

                        // store dialog state to redis
                        if card.ends_dialog {
                            con.del::<_, ()>(&format!("Chat:{:?}:Dialog", message.chat.id))
                                .unwrap();
                        } else {
                            con.hset::<_, _, _, ()>(
                                &format!("Chat:{:?}:Dialog", message.chat.id),
                                "CardID",
                                &card.id,
                            ).unwrap();
                        }
                    }
                    _ => {}
                }
            }
            Err(reply) => {
                // Display error to user
                match reply {
                    Reply::Text(text) => {
                        let reply = message.reply(text);
                        let resp = sender.send(reply).unwrap();
                    }
                    _ => {}
                }
            }
        }
    } else {
        con.del::<_, ()>(&format!("Chat:{:?}:Dialog", message.chat.id))
            .unwrap();
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, config: Arc<Config>, dialogs: Arc<HashMap<String, Dialog>>) -> Worker {
        let thread = thread::Builder::new()
            .name(format!("worker-{}", id))
            .spawn(move || {
                let redis_client = redis::Client::open(config.redis_address.as_str()).unwrap();
                let con = redis_client.get_connection().unwrap();
                let sender = MethodSender::new(&config);

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
                                                update_dialog(&message, dialog, &con, &sender);
                                            } else {
                                                // Invalid DialogID in database, drop the dialog
                                                con.del::<_, ()>(
                                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                                ).unwrap();
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
                                                    start_dialog(&message, dialog, &con, &sender);
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
            })
            .unwrap();

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
