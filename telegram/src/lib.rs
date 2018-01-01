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

pub struct Bot {
    config: config::Config,
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

        Bot { config: config }
    }


    pub fn make_request(&self, client: &mut reqwest::Client, method: &str) -> () {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.config.auth_token,
            method
        );
        let mut req = client.get(&addr);

        debug!(">>> sending to {:?}\n{:?}\n", addr, req);

        let mut resp = req.send().unwrap();

        debug!("<<< response:\n{:?}\n", resp);
        self.handle_response(resp);
    }

    pub fn make_request_json<T: serde::ser::Serialize + Debug>(
        &self,
        client: &mut reqwest::Client,
        method: &str,
        params: &T,
    ) -> () {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.config.auth_token,
            method
        );
        let mut req = client.post(&addr);
        req.json(params);

        debug!(">>> sending to {:?}\n{:?}\n", addr, req);

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

    pub fn run(self) {
        // for sending stuff
        let mut client = reqwest::Client::new();

        // setup the webhook
        let webhook_server = Server::http(&self.config.webhook.bind_address).unwrap();

        // register the webhook
        {
            self.make_request(&mut client, "deleteWebhook");
            self.make_request_json(
                &mut client,
                "setWebhook",
                &json!({
                    "url": &self.config.webhook.external_address
                }),
            );
        }

        'main: loop {
            // get updates
            for mut request in webhook_server.incoming_requests() {
                // TODO: check token so we know it is form Telegram

                let mut body = String::new();
                request.as_reader().read_to_string(&mut body).unwrap();

                debug!("<<< got update:\n{:}\n", body);

                let update: objects::Update = match serde_json::from_str(&body) {
                    Ok(update) => update,
                    Err(err) => panic!("!!! Failed to parse:\n{:?}\n{:}\n", request, err),
                };
                debug!("### got Update: \n{:?}\n", update);

                // TODO: handle responses here

                match update.message {
                    objects::MessageType::Message(message) => {
                        if let Some(ref cmd) = message.text {
                            match cmd as &str {
                                "/start" => {
                                    // send a greeting or something
                                }
                                "/stop" => {
                                    // stop the bot for now
                                    break 'main;
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

            // update world
            // send stuff
        }

        // unregister the webhook
        self.make_request(&mut client, "deleteWebhook");
    }
}
