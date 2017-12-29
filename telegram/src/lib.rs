//! I am a dwarf and I'm making a bot
//! Telegram Bot, Telegram Bot

extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate reqwest;
extern crate uuid;
extern crate tiny_http;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use uuid::Uuid;
use tiny_http::{Server, Response};

mod config;
pub mod types;

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

        Bot { config: config }
    }

    pub fn make_request<T: serde::ser::Serialize>(
        &self,
        client: &mut reqwest::Client,
        method: &str,
        params: &T,
    ) -> () {
        client
            .post(&format!(
                "https://api.telegram.org/bot{}/{}",
                &self.config.auth_token,
                method
            ))
            .json(params)
            .send()
            .unwrap();
    }

    pub fn run(self) {
        // for sending stuff
        let mut client = reqwest::Client::new();

        // setup the webhook
        let webhook_server = Server::http(&self.config.webhook.bind_address).unwrap();

        // register the webhook
        {
            let params = [("url", &self.config.webhook.external_address)];
            self.make_request(&mut client, "setWebhook", &params);
        }

        loop {
            // get updates
            for request in webhook_server.incoming_requests() {
                println!(
                    "received request! method: {:?}, url: {:?}, headers: {:?}",
                    request.method(),
                    request.url(),
                    request.headers()
                );
                let response = Response::from_string("hello world");
                request.respond(response);
            }

            // update world
            // send stuff
        }

        // unregister the webhook
        self.make_request(&mut client, "deleteWebhook", &());
    }
}
