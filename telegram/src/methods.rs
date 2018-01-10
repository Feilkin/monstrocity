// Commands for API methods, so we can do cool stuff

use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use bot::Bot;
use dispatcher::Dispatcher;
use objects::ReplyMarkup;

pub struct Method<P: Serialize> {
    pub method: String,
    pub params: P,
}

impl<P: Serialize> Method<P> {
    pub fn execute<D: Dispatcher>(&self, bot: &Bot<D>) -> ()
    where
        P: Debug,
    {
        bot.make_request_json(&self.method, &self.params);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}
