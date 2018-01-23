//! JSON ser/deable types for Telegram Bot API Types

use methods::{Method, SendMessage};

// TODO: Implement DateTime parsing for date fields
//use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    #[serde(default)]
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub _type: String, // TODO: use a enum here?
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default)]
    pub all_members_are_administrators: bool,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<String>,
    pub sticker_set_name: Option<String>,
    #[serde(default)]
    pub can_set_sticker_set: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Chat>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<i64>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i64>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_members: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    #[serde(default)]
    pub delete_chat_photo: bool,
    #[serde(default)]
    pub group_chat_created: bool,
    #[serde(default)]
    pub supergroup_chat_created: bool,
    #[serde(default)]
    pub channel_chat_created: bool,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
}

// Nice.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl Message {
    pub fn reply(&self, text: String) -> Method<SendMessage> {
        Method {
            method: "sendMessage".to_owned(),
            params: SendMessage {
                chat_id: self.chat.id,
                text: text,
                parse_mode: Some("Markdown".to_owned()),
                reply_to_message_id: Some(self.message_id),
                disable_web_page_preview: None,
                disable_notification: None,
                reply_markup: None,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UpdateKind {
    Message { update_id: i64, message: Message },
    EditedMessage {
        update_id: i64,
        edited_message: Message,
    },
    ChannelPost {
        update_id: i64,
        channel_post: Message,
    },
    EditedChannelPost {
        update_id: i64,
        edited_channel_post: Message,
    },
    InlineQuery {
        update_id: i64,
        inline_query: InlineQuery,
    },
    ChosenInlineQuery {
        update_id: i64,
        chosen_inline_result: ChosenInlineQuery,
    },
    CallbackQuery {
        update_id: i64,
        callback_query: CallbackQuery,
    },
    ShippingQuery {
        update_id: i64,
        shipping_query: ShippingQuery,
    },
    PreCheckoutQuery {
        update_id: i64,
        pre_checkout_query: PreCheckoutQuery,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChosenInlineQuery {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingQuery {}

// TODO: Why do I do this to myself
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub _type: String, // TODO: consider using an Enum here
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: i64,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Voice {
    pub file_id: String,
    pub duration: i64,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoNote {
    pub file_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: String,
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfilePhoto {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default)]
    pub resize_keyboard: bool,
    #[serde(default)]
    pub one_time_keyboard: bool,
    #[serde(default)]
    pub selective: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(default)]
    pub request_contact: bool,
    #[serde(default)]
    pub request_location: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    #[serde(default)]
    pub selective: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}


// TODO: Use a enum here, since we can only send one of the optionals
#[derive(Debug, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instanct: Option<String>,
    pub data: Option<String>, // TODO: consider using a enum here
    pub game_short_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String, // TODO: Consider using enum
    pub until_date: Option<i64>,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "photo")]
    InputMediaPhoto {
        media: String, // TODO: Use enum?
        caption: Option<String>,
    },
    #[serde(rename = "video")]
    InputMediaVideo {
        media: String, // TODO: Use enum?
        caption: Option<String>,
        width: Option<i64>,
        height: Option<i64>,
        duration: Option<i64>,
    },
}


#[derive(Debug, Deserialize, Serialize)]
pub struct InputFile {} // TODO: implement file uploads

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallbackGame {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Animation {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: i64,
    pub height: i64,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    #[serde(default)]
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}
