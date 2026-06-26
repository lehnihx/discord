mod constants;
mod locales;
mod types;
mod wrappers;
mod config;

use poise::serenity_prelude as serenity;

use locales::t;
use types::{Data, Error};
use wrappers::{commands, customer_only};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let token = std::env::var("TOKEN").expect("TOKEN is missing from .env");
    let intents = serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            command_check: Some(|ctx| Box::pin(customer_only(ctx))),
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("{} {}", t("logged_in_as"), ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
