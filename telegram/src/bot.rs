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

use config;
use dispatcher::{Dispatcher, SimpleDispatcher};
use methods::Method;
use objects;

pub struct Bot<D: Dispatcher> {
    config: config::Config,
    client: reqwest::Client,
    dispatcher: D,
    channel: (mpsc::Sender<Arc<Method>>, mpsc::Receiver<Arc<Method>>),
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
            config: config,
            dispatcher: D::new(),
            client: client,
            channel: mpsc::channel(),
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

    pub fn get_sender(&self) -> mpsc::Sender {
        let (ref sender, _) = self.channel;
        return *sender.clone();
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

#[derive(Debug)]
enum WebhookEnum {
    None,
    Internal {
        bind_address: String,
        external_address: String,
    },
    External { /* TODO: implement this */ },
}

/// Builder for Bot, I'm not sure if this is the best way to do it.
pub struct BotBuilder<D: Dispatcher = SimpleDispatcher> {
    config: config::Config,
    dispatcher: Option<D>,
    webhook: WebhookEnum,
}

impl<D: Dispatcher> BotBuilder<D> {
    pub fn new(config_file: &str) -> BotBuilder<D> {
        let mut f = File::open(config_file).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );

        let config = toml::from_str(&contents).expect("failed to parse config");

        println!("Config: {:?}", &config);

        BotBuilder {
            config: config,
            dispatcher: None,
            webhook: WebhookEnum::None,
        }
    }

    pub fn with_dispatcher(mut self, dispatcher: D) -> BotBuilder<D> {
        self.dispatcher = Some(dispatcher);
        self
    }

    pub fn with_internal_webhook(
        mut self,
        bind_address: String,
        external_address: String,
    ) -> BotBuilder<D> {
        self.webhook = WebhookEnum::Internal {
            bind_address,
            external_address,
        };
        self
    }

    pub fn with_commands(mut self, commands: &[(&str, &Fn(&objects::Message) -> ())]) {
        unimplemented!();
    }

    pub fn with_external_webhook(mut self, address: String) -> BotBuilder<D> {
        unimplemented!();
    }

    pub fn build(self) -> Bot<D> {
        unimplemented!();
    }
}
