//! JSON ser/deable types for Telegram Bot API Types

use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i64,
    #[serde(default)]
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
    #[serde(default)]
    all_members_are_administrators: bool,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<String>,
    sticker_set_name: Option<String>,
    #[serde(default)]
    can_set_sticker_set: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    message_id: i64,
    from: Option<User>,
    date: i64,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<Chat>,
    forward_signature: Option<String>,
    forward_date: Option<i64>,
    reply_to_message: Option<Box<Message>>,
    edit_date: Option<i64>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Sticker>,
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
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
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
pub struct MessageEntity {
    #[serde(rename = "type")]
    _type: String, // TODO: consider using an Enum here
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize {
    file_id: String,
    width: i64,
    height: i64,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {
    file_id: String,
    duration: i64,
    performer: Option<String>,
    title: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    file_id: String,
    width: i64,
    height: i64,
    duration: i64,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    file_id: String,
    duration: i64,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoNote {
    file_id: String,
    length: i64,
    duration: i64,
    thumb: Option<PhotoSize>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: String,
    user_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfilePhoto {
    total_count: i64,
    photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    file_id: String,
    file_size: Option<i64>,
    file_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    resize_keyboard: bool,
    #[serde(default)]
    one_time_keyboard: bool,
    #[serde(default)]
    selective: bool,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Animation {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    file_id: String,
    width: i64,
    height: i64,
    thumb: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<MaskPosition>,
    file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StickerSet {
    name: String,
    title: String,
    #[serde(default)]
    contains_masks: bool,
    stickers: Vec<Sticker>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaskPosition {
    point: String,
    x_shift: f64,
    y_shift: f64,
    scale: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabeledPrice {
    label: String,
    amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingOption {
    id: String,
    title: String,
    prices: Vec<LabeledPrice>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreCheckoutQuery {
    id: String,
    from: User,
    currency: String,
    total_amount: i64,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
}
