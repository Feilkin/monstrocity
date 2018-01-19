use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{mpsc, Arc};

use serde;
use serde_json;
use uuid::Uuid;
use tiny_http::{Server, Response};
use toml;
use reqwest;
use regex::Regex;

use config;
use dialog::Dialog;
use dispatcher::{Dispatcher, ASyncDispatcher};
use methods::Method;
use objects;

pub enum Command {
    Dialog(String),
}

pub struct Bot<D: Dispatcher = ASyncDispatcher> {
    config: config::Config,
    client: reqwest::Client,
    dispatcher: D,
    dialogs: HashMap<String, Dialog>,
    commands: HashMap<String, Command>,
}

impl<D: Dispatcher> Bot<D> {
    pub fn new(config_file: &str) -> Bot<D> {
        let mut f = File::open(config_file).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );

        let config = toml::from_str(&contents).expect("failed to parse config");
        let client = reqwest::Client::new();

        Bot {
            dispatcher: D::new(&config),
            config: config,
            client: client,
            dialogs: HashMap::new(),
            commands: HashMap::new(),
        }
    }

    pub fn make_request(&self, method: &str) -> () {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.config.auth_token,
            method
        );
        let mut req = self.client.get(&addr);

        debug!(">>> sending to {:?}\n{:?}\n", addr, req);

        let mut resp = req.send().unwrap();

        debug!("<<< response:\n{:?}\n", resp);
        self.handle_response(resp);
    }

    pub fn make_request_json<T: serde::ser::Serialize + Debug>(
        &self,
        method: &str,
        params: &T,
    ) -> () {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.config.auth_token,
            method
        );
        let mut req = self.client.post(&addr);
        req.json(params);

        debug!(
            ">>> sending to {:?}\n{:?}\n{:?}\n",
            addr,
            req,
            serde_json::to_string(params).unwrap()
        );

        let resp = req.send().unwrap();

        debug!("<<< response:\n{:?}\n", resp);
        self.handle_response(resp);
    }

    pub fn handle_response(&self, mut response: reqwest::Response) -> () {
        match response.text() {
            Ok(body) => {
                debug!("Body: {:?}\n", body);
            }
            Err(_) => {
                panic!("Got an empty response: {:?}", response);
            }
        };
    }

    pub fn register_command_dialog(&mut self, command: String, dialog: Dialog) -> () {
        let dialog_id = dialog.id.clone();
        self.dialogs.insert(dialog_id.clone(), dialog);
        self.commands.insert(command, Command::Dialog(dialog_id));
    }

    pub fn check_command(&self, text: &str) -> Option<&Command> {
        if !text.starts_with("/") {
            return None
        }

        lazy_static! {
            static ref COMMAND_PATTERN: Regex = Regex::new(r"^/([[:alpha:]]+)(@[[:alpha:]]+)?").unwrap();
        }

        if !COMMAND_PATTERN.is_match(text) {
            return None
        }

        let caps = COMMAND_PATTERN.captures(text).unwrap();

        if let Some(bot_identifier) = caps.get(2) {
            // TODO: check if this command was meant for us.
            // TODO: Get bot username with getMe?
            return None
        }

        self.commands.get(caps.get(1).unwrap().as_str())
    }

    pub fn run(mut self) {

    }
}

#[derive(Debug)]
enum WebhookEnum {
    None,
    Internal {
        bind_address: String,
        external_address: String,
    },
    External { /* TODO: implement this */ },
}
