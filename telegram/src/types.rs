//! JSON ser/deable types for Telegram Bot API Types

use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    id: i64,
    #[serde(rename = "type")]
    _type: String, // TODO: use a enum here?
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    all_members_are_administrators: bool,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<String>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    message_id: i64,
    from: Option<User>,
    date: DateTime<Utc>,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<Chat>,
    forward_signature: Option<String>,
    forward_date: Option<DateTime<Utc>>,
    reply_to_message: Option<Box<Message>>,
    edit_date: Option<DateTime<Utc>>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    //game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    //sticker: Option<Sticker>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    new_chat_members: Option<Vec<User>>,
    left_chat_members: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    #[serde(default)]
    delete_chat_photo: bool,
    #[serde(default)]
    group_chat_created: bool,
    #[serde(default)]
    supergroup_chat_created: bool,
    #[serde(default)]
    channel_chat_created: bool,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<Message>>,
    //invoice: Option<Invoice>,
    //successful_payment: Option<SuccessfulPayment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    #[serde(rename = "channel_post")]
    ChannelPost(Message),
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost {},
    #[serde(rename = "inline_query")]
    InlineQuery {},
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineQuery {},
    #[serde(rename = "callback_query")]
    CallbackQuery {},
    #[serde(rename = "shipping_query")]
    ShippingQuery {},
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery {},
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Update {
    update_id: i64,
    message: Message,
}


// TODO: Why do I do this to myself
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageEntity {}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoNote {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfilePhoto {}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardButton {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardRemove {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineKeyboardButton {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForceReply {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatPhoto {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMember {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseParameters {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputMedia {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputMediaPhoto {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputMediaVideo {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputFile {}
