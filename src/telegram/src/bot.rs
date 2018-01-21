use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::PathBuf;
use std::{thread, time};

use ctrlc;
use serde;
use serde_json;
use uuid::Uuid;
use toml;

use config::Config;
use worker::Worker;

pub struct Bot {
    name: String,
    config: Arc<Config>, // Auth token, worker count, etc
    workers: Vec<Worker>,
}

impl Bot {
    /// runs the bot, spawning workers and waits for them to exit
    pub fn run(&mut self) -> () {
        let running = Arc::new(AtomicBool::new(true));
        let r = Arc::clone(&running);

        ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
            .expect("Error setting Ctrl-C handler");

        // Spawn Workers
        println!("Spawning {} workers", self.config.workers);

        for i in 1..self.config.workers {
            self.workers.push(Worker::new(i, Arc::clone(&self.config)));
        }

        // wait until we receive a SIGINT

        let sleep_dur = time::Duration::from_millis(10);
        while running.load(Ordering::SeqCst) {
            thread::sleep(sleep_dur);
        } // should I sleep here?

        // Send shutdown notice to all Workers
        println!("Shutting down workers");

        // Wait until Workers are shut down

        // profit
        println!("Shutting down bot");
    }
}

enum BotConfig {
    Config(Config),
    Path(PathBuf),
    None,
}

pub struct BotBuilder {
    name: String,
    config: BotConfig,
}

impl BotBuilder {
    pub fn new(name: String) -> BotBuilder {
        BotBuilder {
            name: name,
            config: BotConfig::None,
        }
    }

    // Move self, because one builder should only build one bot?
    pub fn with_config(mut self, conf: Config) -> BotBuilder {
        self.config = BotConfig::Config(conf);
        self
    }

    pub fn with_config_file<T: Into<PathBuf>>(mut self, path: T) -> BotBuilder {
        self.config = BotConfig::Path(path.into());
        self
    }

    pub fn build(self) -> Bot {
        let conf = match self.config {
            BotConfig::Config(conf) => conf,
            BotConfig::Path(path) => {
                let mut f = File::open(path).expect("file not found");

                let mut contents = String::new();
                f.read_to_string(&mut contents).expect(
                    "something went wrong reading the file",
                );

                toml::from_str::<Config>(&contents).expect("failed to parse config")
            }
            BotConfig::None => panic!("BotBuilder needs to have either Config or Path!"),
        };

        Bot {
            name: self.name,
            config: Arc::new(conf),
            workers: Vec::new(),
        }
    }
}
