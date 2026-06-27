mod ai;
mod commands;
mod constants;
mod forums;
mod locales;
mod types;
mod wrappers;

use poise::serenity_prelude as serenity;

use commands::commands;
use forums::handle_event;
use locales::t;
use types::{Data, Error};
use wrappers::customer_only;

#[tokio::main]
async fn main() -> Result<(), Error> {
  dotenvy::dotenv().ok();

  let token = std::env::var("TOKEN").expect(t("token_missing"));
  let intents = serenity::GatewayIntents::GUILDS
    | serenity::GatewayIntents::GUILD_MESSAGES
    | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: commands(),
      command_check: Some(|ctx| Box::pin(customer_only(ctx))),
      event_handler: |ctx, event, _framework, data| {
        Box::pin(async move { handle_event(ctx, event, data).await })
      },
      ..Default::default()
    })
    .setup(|ctx, ready, framework| {
      Box::pin(async move {
        println!("{} {}", t("logged_in_as"), ready.user.name);
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(Data {
          conversations: Default::default(),
        })
      })
    })
    .build();

  let mut client = serenity::ClientBuilder::new(token, intents)
    .framework(framework)
    .await?;

  client.start().await?;

  Ok(())
}
