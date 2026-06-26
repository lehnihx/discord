mod constants;
mod locales;
mod types;
mod wrappers;

use poise::serenity_prelude as serenity;

use constants::{CUSTOMERS};
use locales::t;
use types::{Context, Data, Error};
use wrappers::{reply_to_command, commands};

async fn customer_only(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx
        .guild_id()
        .is_some_and(|guild_id| CUSTOMERS.contains(&guild_id.get()))
    {
        return Ok(true);
    }

    reply_to_command(ctx, t("not_eligible"), true).await?;

    Ok(false)
}

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
