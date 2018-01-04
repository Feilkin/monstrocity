//! I am a dwarf and I'm making a bot
//! Telegram Bot, Telegram Bot

#[macro_use]
extern crate log;
extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;
extern crate reqwest;
extern crate uuid;
extern crate tiny_http;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt::Debug;

use uuid::Uuid;
use tiny_http::{Server, Response};

mod config;
pub mod objects;
pub mod command;

use command::Command;

pub struct Bot {
    config: config::Config,
    client: reqwest::Client,
}

impl Bot {
    pub fn new(config_file: &str) -> Bot {
        let mut f = File::open(config_file).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );

        let config = toml::from_str(&contents).expect("failed to parse config");

        println!("Config: {:?}", &config);

        // for sending stuff
        let client = reqwest::Client::new();

        Bot {
            config: config,
            client: client,
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

    pub fn run(mut self) {

        // setup the webhook
        let webhook_server = Server::http(&self.config.webhook.bind_address).unwrap();

        // register the webhook
        {
            self.make_request("deleteWebhook");
            self.make_request_json(
                "setWebhook",
                &json!({
                    "url": &self.config.webhook.external_address
                }),
            );
        }

        let mut quit = false;
        'main: loop {
            // get updates
            while let Ok(Some(mut request)) = webhook_server.try_recv() {
                // TODO: check token so we know it is form Telegram

                let mut body = String::new();
                request.as_reader().read_to_string(&mut body).unwrap();

                debug!("<<< got update:\n{:}\n", body);

                let update: objects::UpdateKind = match serde_json::from_str(&body) {
                    Ok(update) => update,
                    Err(err) => panic!("!!! Failed to parse:\n{:?}\n{:}\n", request, err),
                };
                debug!("### got Update: \n{:?}\n", update);

                // TODO: handle responses here

                match update {
                    objects::UpdateKind::Message { message, .. } => {
                        if let Some(ref cmd) = message.text {
                            match cmd as &str {
                                "/start" => {
                                    // send a greeting or something
                                    let reply = message.reply(
                                        "Please do not use this bot.\n_Thanks._"
                                            .to_owned(),
                                    );
                                    reply.execute(&self);
                                }
                                "/stop" => {
                                    // stop the bot for now
                                    let reply = message.reply("I don't blame you.".to_owned());
                                    reply.execute(&self);
                                    quit = true;
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }

                let response = Response::from_string("").with_status_code(200);
                request.respond(response).unwrap();
            }

            if quit == true {
                break;
            }

            // update world
            // send stuff
        }

        // unregister the webhook
        self.make_request("deleteWebhook");
    }
}
