//! Dialog trees. Because why not.
//!
//! Commands are fine for simple interaction between a bot and a user (command -> response),
//! but aren't enough to handle complex interactions which includes multiple messages and replies,
//! inline keyboards, and other cool Telegram features.
//!

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::value::Value;

use objects::Message;

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
}

enum CardText {
    Raw(String),
    Builder(Box<Fn(&Message) -> String + Sync + Send>),
    None,
}

pub struct Card {
    id: String,
    text: CardText,
    wants_reply: Option<Box<Fn(&Message) -> Result<Reply, Reply> + Sync + Send>>,
    ends_dialog: bool,
}

impl Card {
    pub fn new(id: String) -> Card {
        Card {
            id: id,
            text: CardText::None,
            wants_reply: None,
            ends_dialog: false,
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
        self.wants_reply = Some(Box::new(f));
        self
    }

    pub fn build_text<F: 'static + Fn(&Message) -> String + Sync + Send>(mut self, f: F) -> Card {
        self.text = CardText::Builder(Box::new(f));
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
}

pub enum Reply {
    Text(String),
    Message(Value),
    ShowCard(String),
    None,
}
