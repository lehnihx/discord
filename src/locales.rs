pub fn t(key: &str) -> &str {
  match key {
    "logged_in_as" => "Logged in as",
    "not_eligible" => "Server not eligible",
    "pong" => "🏓 Pong!",
    "greet_lenix" => "Hi Lenix!",
    "reply_pong" => "Replies with Pong!",
    "reply_lenix" => "Replies with Hi Lenix!",
    _ => key,
  }
}
