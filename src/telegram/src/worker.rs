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

use dialog::Dialog;
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

            loop {
                match con.lpop::<&str, String>("TestQueue") {
                    Ok(update_data) => {
                        let update = serde_json::from_str::<UpdateKind>(&update_data).unwrap();
                        match update {
                            UpdateKind::Message { update_id, message } => {
                                // First, check if user has ongoing Dialog
                                match con.hget::<&str, &str, i64>(
                                    &format!("Chat:{:?}:Dialog", message.chat.id),
                                    "DialogID",
                                ) {
                                    Ok(dialog_id) => {
                                        // TODO, we have a ongoing dialog
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
                                                let root = dialog.get_root();
                                                // TODO: send here
                                                println!(
                                                    ">> Send this: {}",
                                                    root.get_text(&message)
                                                );
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
