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

mod config; // I don't think this needs to be public?
pub mod dispatcher;
pub mod objects;
pub mod methods;
pub mod bot;
