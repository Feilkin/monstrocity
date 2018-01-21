//! Worker thread, which do the actual work

use std::sync::{mpsc, Arc, Mutex};
use std::{time, thread};

use redis;
use redis::Commands;
use reqwest;
use serde::Serialize;
use serde_json;
use serde_json::value::Value;

use objects::UpdateKind;
use methods::{Method, SendMessage};
use config::Config;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, config: Arc<Config>) -> Worker {
        let thread = thread::spawn(move || {
            let redis_client = redis::Client::open(config.redis_address.as_str()).unwrap();
            let con = redis_client.get_connection().unwrap();

            loop {
                match con.lpop::<&str, String>("TestQueue") {
                    Ok(update_data) => {
                        let update = serde_json::from_str::<UpdateKind>(&update_data).unwrap();
                        match update {
                            UpdateKind::Message { update_id, message } => {
                                println!("Got a message: {:?}", message)
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
