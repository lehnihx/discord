pub fn t(key: &str) -> &str {
  match key {
    "logged_in_as" => "Logged in as",
    "not_eligible" => "Server not eligible",
    "pong" => "🏓 Pong!",
    "greet_lenix" => "Hi Lenix!",
    "reply_pong" => "Replies with Pong!",
    "reply_lenix" => "Replies with Hi Lenix!",
    "reply_ai_space" => "Create or open your private AI space",
    "ai_space_prompt" => "Create your private AI forum space.",
    "ai_space_button" => "Create AI Space",
    "ai_space_created" => "Your AI space is ready:",
    "ai_space_missing_guild" => "AI spaces can only be created inside a server.",
    "ai_space_failed" => "Could not create your AI space:",
    _ => key,
  }
}
