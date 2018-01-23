//! Dialog trees. Because why not.
//!
//! Commands are fine for simple interaction between a bot and a user (command -> response),
//! but aren't enough to handle complex interactions which includes multiple messages and replies,
//! inline keyboards, and other cool Telegram features.
//!

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::value::Value;

use objects::{Message, CallbackQuery, InlineKeyboardMarkup, InlineKeyboardButton};

pub struct Dialog {
    pub id: String,
    cards: HashMap<String, Arc<Card>>,
    root: String,
}

impl Dialog {
    pub fn new(id: String, root: String) -> Dialog {
        Dialog {
            id: id,
            cards: HashMap::new(),
            root: root,
        }
    }

    pub fn add_card(mut self, card: Card) -> Dialog {
        self.cards.insert(card.id.clone(), Arc::new(card));
        self
    }

    pub fn get_root(&self) -> Arc<Card> {
        Arc::clone(self.cards.get(&self.root).unwrap())
    }

    pub fn get_card(&self, id: &str) -> Arc<Card> {
        Arc::clone(self.cards.get(id).unwrap())
    }
}

enum CardText {
    Raw(String),
    Builder(Box<Fn(&Message) -> String + Sync + Send>),
    None,
}

pub struct Card {
    pub id: String,
    text: CardText,
    reply_callback: Option<Box<Fn(&Message) -> Result<Reply, Reply> + Sync + Send>>,
    keyboard: Option<Keyboard>,
    pub ends_dialog: bool,
}

impl Card {
    pub fn new(id: String) -> Card {
        Card {
            id: id,
            text: CardText::None,
            reply_callback: None,
            ends_dialog: false,
            keyboard: None,
        }
    }

    pub fn text(mut self, text: String) -> Card {
        self.text = CardText::Raw(text);
        self
    }

    pub fn wants_reply<F: 'static + Fn(&Message) -> Result<Reply, Reply> + Sync + Send>(
        mut self,
        f: F,
    ) -> Card {
        self.reply_callback = Some(Box::new(f));
        self
    }

    pub fn build_text<F: 'static + Fn(&Message) -> String + Sync + Send>(mut self, f: F) -> Card {
        self.text = CardText::Builder(Box::new(f));
        self
    }

    pub fn with_keyboard(mut self, keyboard: Keyboard) -> Card {
        self.keyboard = Some(keyboard);
        self
    }

    pub fn ends_dialog(mut self) -> Card {
        self.ends_dialog = true;
        self
    }

    pub fn get_text(&self, msg: &Message) -> String {
        match self.text {
            CardText::Raw(ref text) => text.to_owned(),
            CardText::Builder(ref builder) => builder(msg),
            CardText::None => "<None>".to_owned(),
        }
    }

    pub fn get_keyboard(&self) -> Option<InlineKeyboardMarkup> {
        match self.keyboard {
            Some(ref keyboard) => {
                let mut keyboard_markup = InlineKeyboardMarkup { inline_keyboard: Vec::new() };

                for row in &keyboard.rows {
                    let mut markup_row = Vec::new();

                    for button in row {
                        let mut markup_button = InlineKeyboardButton {
                            text: button.label.clone(),
                            url: None,
                            callback_data: None,
                            switch_inline_query: None,
                            switch_inline_query_current_chat: None,
                            callback_game: None,
                            pay: None,
                        };

                        match button.data {
                            ButtonData::CallbackData(ref data) => {
                                markup_button.callback_data = Some(data.clone())
                            }
                            _ => unimplemented!(),
                        };

                        markup_row.push(markup_button);
                    }
                    keyboard_markup.inline_keyboard.push(markup_row);
                }

                Some(keyboard_markup)
            }
            None => None,
        }
    }

    pub fn check_reply(&self, msg: &Message) -> Option<Result<Reply, Reply>> {
        match self.reply_callback {
            Some(ref cb) => Some(cb(msg)),
            None => None,
        }
    }
}

pub enum Reply {
    Text(String),
    Message(Value),
    ShowCard(String),
    None,
}

pub struct Keyboard {
    id: String,
    rows: Vec<Vec<Button>>,
    callback: Box<Fn(&CallbackQuery) -> KeyboardReply + Sync + Send>,
}

impl Keyboard {
    pub fn new<F: 'static + Fn(&CallbackQuery) -> KeyboardReply + Sync + Send>(
        id: String,
        callback: F,
    ) -> Keyboard {
        Keyboard {
            id: id,
            rows: Vec::new(),
            callback: Box::new(callback),
        }
    }

    pub fn add_row(mut self, row: &[Button]) -> Keyboard {
        let mut new_row = Vec::new();
        for button in row {
            new_row.push(button.clone());
        }
        self.rows.push(new_row);
        self
    }
}

pub enum KeyboardReply {
    HideKeyboardAndShowCard(String),
    ShowCard(String),
    CancelDialog,
}


#[derive(Clone)]
pub struct Button {
    label: String,
    data: ButtonData,
}

impl Button {
    pub fn new(label: String) -> Button {
        Button {
            label: label,
            data: ButtonData::None,
        }
    }

    pub fn with_callback_data(mut self, data: String) -> Button {
        self.data = ButtonData::CallbackData(data);
        self
    }
}

#[derive(Clone)]
pub enum ButtonData {
    Url(String),
    CallbackData(String),
    SwitchInlineQuery(String),
    SwitchInlineQueryCurrentChat(String),
    Pay,
    None,
}
