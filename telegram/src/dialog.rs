//! Dialog trees. Because why not.
//!
//! Commands are fine for simple interaction between a bot and a user (command -> response),
//! but aren't enough to handle complex interactions which includes multiple messages and replies,
//! inline keyboards, and other cool Telegram features.
//!

use std::collections::HashMap;

use serde_json::value::Value;

use objects::Message;

pub struct Dialog {
    pub id: String,
    cards: HashMap<String, Card>,
}

impl Dialog {
    pub fn new(id: String) -> Dialog {
        Dialog {
            id: id,
            cards: HashMap::new(),
        }
    }

    pub fn add_card(mut self, card: Card) -> Dialog {
        self.cards.insert(card.id.clone(), card);
        self
    }
}

enum CardText {
    Raw(String),
    Builder(Box<Fn(Message) -> String>),
    None,
}

pub struct Card {
    id: String,
    text: CardText,
    wants_reply: Option<Box<Fn(Message) -> Result<Reply, Reply>>>,
}

impl Card {
    pub fn new(id: String) -> Card {
        Card {
            id: id,
            text: CardText::None,
            wants_reply: None,
        }
    }

    pub fn text(mut self, text: String) -> Card {
        self.text = CardText::Raw(text);
        self
    }

    pub fn wants_reply<F: 'static + Fn(Message) -> Result<Reply, Reply>>(mut self, f: F) -> Card {
        self.wants_reply = Some(Box::new(f));
        self
    }

    pub fn build_text<F: 'static + Fn(Message) -> String>(mut self, f: F) -> Card {
        self.text = CardText::Builder(Box::new(f));
        self
    }
}

pub enum Reply {
    Text(String),
    Message(Value),
    ShowCard(String),
    None,
}
