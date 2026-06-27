use std::collections::HashMap;

#[derive(Clone, serde::Serialize)]
pub struct ConversationMessage {
  pub role: &'static str,
  pub content: String,
}

pub struct Data {
  pub conversations: tokio::sync::Mutex<HashMap<u64, Vec<ConversationMessage>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
