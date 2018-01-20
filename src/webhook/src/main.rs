//! Botti webhookki juttu

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ctrlc;
extern crate redis;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;
extern crate tiny_http;

extern crate telegram;

use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use redis::Commands;
use tiny_http::{Response, Server};

use telegram::objects::UpdateKind;

#[derive(Debug, Deserialize)]
struct Config {
    bind_address: String,
    redis_address: String,
    // TODO
    //spam_limit: 10,
}

fn main() {
    env_logger::init();

    println!("Hello, world!");

    let mut f = File::open("webhook.toml").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    let config = toml::from_str::<Config>(&contents).expect("failed to parse config");

    // setup the webhook
    let webhook_server = Server::http(&config.bind_address).unwrap();

    let redis_client = redis::Client::open(config.redis_address.as_str()).unwrap();
    let con = redis_client.get_connection().unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);
    ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    let sleep_dur = time::Duration::from_millis(10);

    while running.load(Ordering::SeqCst) {
        // get updates
        while let Ok(Some(mut request)) = webhook_server.try_recv() {
            // TODO: check token so we know it is form Telegram

            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();

            // try parsing the received Update
            let update: UpdateKind = match serde_json::from_str(&body) {
                Ok(update) => update,
                Err(err) => panic!("!!! Failed to parse:\n{:?}\n{:}\n", request, err),
            };

            // TODO: spam filtering

            // for now, we can use the client body because it is already JSON
            let length: i64 = con.rpush("TestQueue", body).unwrap();

            let response = Response::from_string("").with_status_code(200);
            request.respond(response).unwrap();

            debug!(
                "Saved update {} to TestQueue!",
                match update {
                    UpdateKind::Message { update_id, .. } |
                    UpdateKind::EditedMessage { update_id, .. } |
                    UpdateKind::ChannelPost { update_id, .. } |
                    UpdateKind::EditedChannelPost { update_id, .. } |
                    UpdateKind::InlineQuery { update_id, .. } |
                    UpdateKind::ChosenInlineQuery { update_id, .. } |
                    UpdateKind::CallbackQuery { update_id, .. } |
                    UpdateKind::ShippingQuery { update_id, .. } |
                    UpdateKind::PreCheckoutQuery { update_id, .. } => update_id,
                }
            );
        }

        thread::sleep(sleep_dur);
    }

    println!("Shutting down the webhook...");
}
