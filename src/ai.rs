use serde::{Deserialize, Serialize};

use crate::{
  constants::{AI_COUNTRY, AI_DOMAINS, AI_MODEL, AI_PROMPT, AI_TEMPERATURE},
  locales::t,
  types::Error,
};

#[derive(Serialize)]
struct ChatRequest {
  model: &'static str,
  messages: Vec<ChatMessage>,
  temperature: f32,
  search_settings: SearchSettings,
  user: String,
}

#[derive(Serialize)]
struct SearchSettings {
  country: &'static str,
  include_domains: &'static [&'static str],
  include_images: bool,
}

#[derive(Serialize)]
struct ChatMessage {
  role: &'static str,
  content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
  choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
  message: ChatChoiceMessage,
}

#[derive(Deserialize)]
struct ChatChoiceMessage {
  content: String,
}

pub async fn generate_reply(
  topic: &str,
  user_message: &str,
  user_id: u64,
  display_name: &str,
) -> Result<String, Error> {
  let api_key = std::env::var("API_KEY").expect(t("api_key_err"));
  let client = reqwest::Client::new();

  let request = ChatRequest {
    model: AI_MODEL,
    temperature: AI_TEMPERATURE,
    search_settings: SearchSettings {
      country: AI_COUNTRY,
      include_domains: AI_DOMAINS,
      include_images: true,
    },
    messages: vec![
      ChatMessage {
        role: "system",
        content: AI_PROMPT.to_string(),
      },
      ChatMessage {
        role: "user",
        content: format!(
          "User: {display_name}\n\nTopic: {topic}\n\nMessage: {user_message}"
        ),
      },
    ],
    user: user_id.to_string(),
  };

  let response = client
    .post("https://api.groq.com/openai/v1/chat/completions")
    .bearer_auth(api_key)
    .json(&request)
    .send()
    .await?;

  if !response.status().is_success() {
    return Err(format!("{} {}", t("api_err"), response.text().await?).into());
  }

  let response = response.json::<ChatResponse>().await?;
  let Some(choice) = response.choices.into_iter().next() else {
    return Err(t("api_nochoices").into());
  };

  Ok(choice.message.content)
}
