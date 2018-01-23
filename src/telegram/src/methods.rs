// Commands for API methods, so we can do cool stuff

use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use objects::{ReplyMarkup, InlineKeyboardMarkup};

#[derive(Debug)]
pub struct Method<T: Serialize> {
    pub method: String,
    pub params: T,
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

impl Method<SendMessage> {
    pub fn with_keyboard(mut self, keyboard: InlineKeyboardMarkup) -> Method<SendMessage> {
        self.params.reply_markup = Some(ReplyMarkup::InlineKeyboardMarkup(keyboard));
        self
    }
}
