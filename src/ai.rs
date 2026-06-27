use serde::{Deserialize, Serialize};

use crate::types::Error;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_MODEL: &str = "llama-3.1-8b-instant";

#[derive(Serialize)]
struct ChatRequest {
  model: &'static str,
  messages: Vec<ChatMessage>,
  temperature: f32,
  max_tokens: u16,
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

pub async fn generate_reply(topic: &str, user_message: &str) -> Result<String, Error> {
  let api_key = std::env::var("API_KEY").expect("API_KEY key is missing");
	println!("{}", api_key);
  let client = reqwest::Client::new();

  let request = ChatRequest {
    model: GROQ_MODEL,
    temperature: 0.7,
    max_tokens: 700,
    messages: vec![
      ChatMessage {
        role: "system",
        content: "You are a helpful AI assistant inside a Discord forum thread. Reply clearly and concisely.".to_string(),
      },
      ChatMessage {
        role: "user",
        content: format!("Topic: {topic}\n\nMessage: {user_message}"),
      },
    ],
  };

  let response = client
    .post(GROQ_API_URL)
    .bearer_auth(api_key)
    .json(&request)
    .send()
    .await?;

  if !response.status().is_success() {
    return Err(format!("Groq API error: {}", response.text().await?).into());
  }

  let response = response.json::<ChatResponse>().await?;
  let Some(choice) = response.choices.into_iter().next() else {
    return Err("Groq API returned no choices".into());
  };

  Ok(choice.message.content)
}
