//! I am a dwarf and I'm making a bot
//! Telegram Bot, Telegram Bot

extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
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

        println!("Config: {:?}", &config);

        Bot { config: config }
    }

    pub fn make_request<T: serde::ser::Serialize + Debug>(
        &self,
        client: &mut reqwest::Client,
        method: &str,
        params: Option<&T>,
    ) -> () {
        let addr = format!(
            "https://api.telegram.org/bot{}/{}",
            &self.config.auth_token,
            method
        );
        let mut req;
        match params {
            Some(params) => {
                req = client.post(&addr);
                req.form(params);
            }
            None => {
                req = client.get(&addr);
            }
        };
        println!(">> sending to {:?}, {:?}\n", addr, req);
        let mut resp = req.send().unwrap();
        println!("<< resp: {:?}\n", resp);
        match resp.text() {
            Ok(body) => {
                println!("Body: {:?}\n", body);
            }
            Err(_) => (),
        };
    }

    pub fn run(self) {
        // for sending stuff
        let mut client = reqwest::Client::new();

        // setup the webhook
        let webhook_server = Server::http(&self.config.webhook.bind_address).unwrap();

        // register the webhook
        {
            self.make_request::<&()>(&mut client, "deleteWebhook", None);
            self.make_request(
                &mut client,
                "setWebhook",
                Some(&[("url", &self.config.webhook.external_address)]),
            );
        }

        loop {
            // get updates
            for mut request in webhook_server.incoming_requests() {
                println!(
                    "received request! method: {:?}, url: {:?}, headers: {:?}",
                    request.method(),
                    request.url(),
                    request.headers()
                );
                let update: types::Update = serde_json::from_reader(request.as_reader()).unwrap();

                println!(">> Got Update: \n{:?}", update);

                let response = Response::from_string("").with_status_code(200);
                request.respond(response).unwrap();
            }

            // update world
            // send stuff
        }

        // unregister the webhook
        self.make_request::<&()>(&mut client, "deleteWebhook", None);
    }
}
