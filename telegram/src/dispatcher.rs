//! A thing that consumes Updates, or something
//! The plan is to have a Dispatcher, that gets Updates from somewhere, maybe
//! does stuff to them, and then calls Command callbacks however it wants

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use reqwest;
use serde::Serialize;
use serde_json;
use serde_json::value::Value;

use config::Config;
use objects::UpdateKind;
use methods::Method;

pub trait Dispatcher {
    fn new(config: &Config) -> Self;
    fn dispatch_update(&mut self, update: UpdateKind) -> Result<(), String>;
}

enum Message {
    Update(UpdateKind),
    Terminate,
}

pub struct ASyncDispatcher {
    workers: Vec<Worker>,
    message_sender: mpsc::Sender<Message>,
    method_consumer: thread::JoinHandle<()>,
}

impl Dispatcher for ASyncDispatcher {
    fn new(config: &Config) -> ASyncDispatcher {

        // For sending updates to threads
        let (message_sender, message_receiver) = mpsc::channel();
        let message_receiver = Arc::new(Mutex::new(message_receiver));

        // For sending messages to telegram
        let (method_sender, method_receiver) = mpsc::channel();

        let worker_count = 10;
        let mut workers = Vec::with_capacity(worker_count);

        for id in 0..worker_count {
            workers.push(Worker::new(
                id,
                Arc::clone(&message_receiver),
                method_sender.clone(),
            ))
        }

        let auth_token = config.auth_token.clone();
        let method_consumer = thread::spawn(move || {
            let client = reqwest::Client::new();
            loop {
                let method: Method = method_receiver.recv().unwrap();

                let addr = format!(
                    "https://api.telegram.org/bot{}/{}",
                    &auth_token,
                    method.method
                );
                let mut req = client.post(&addr);
                req.json(&method.params);

                debug!(
                    ">>> sending to {:?}\n{:?}\n{:?}\n",
                    addr,
                    req,
                    serde_json::to_string(&method.params).unwrap()
                );

                let resp = req.send().unwrap();

                debug!("<<< response:\n{:?}\n", resp);

            }
        });

        ASyncDispatcher {
            message_sender: message_sender,
            workers: workers,
            method_consumer: method_consumer,
        }
    }

    fn dispatch_update(&mut self, update: UpdateKind) -> Result<(), String> {
        self.message_sender.send(Message::Update(update)).unwrap();
        Ok(())
    }
}

impl Drop for ASyncDispatcher {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.message_sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        message_receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        method_sender: mpsc::Sender<Method>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = message_receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Update(update) => {
                    match update {
                        UpdateKind::Message { message, .. } => {
                            if let Some(ref cmd) = message.text {
                                match cmd as &str {
                                    "/start" => {
                                        // send a greeting or something
                                        let reply = message.reply(
                                            "Please do not use this bot.\n_Thanks._"
                                                .to_owned(),
                                        );
                                        method_sender.send(reply).unwrap();
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }

                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
