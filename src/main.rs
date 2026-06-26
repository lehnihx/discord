mod constants;
mod locales;
mod types;
mod wrappers;

use poise::serenity_prelude as serenity;

use constants::CUSTOMERS;
use locales::t;
use types::{Context, Data, Error};
use wrappers::{reply_to_command};

async fn reject_if_not_customer(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx
        .guild_id()
        .is_some_and(|guild_id| CUSTOMERS.contains(&guild_id.get()))
    {
        return Ok(false);
    }

    reply_to_command(ctx, t("not_eligible"), true).await?;

    Ok(true)
}

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    if reject_if_not_customer(ctx).await? {
        return Ok(());
    }

    reply_to_command(ctx, t("pong"), true).await
}

#[poise::command(slash_command)]
async fn lenix(ctx: Context<'_>) -> Result<(), Error> {
    if reject_if_not_customer(ctx).await? {
        return Ok(());
    }

    reply_to_command(ctx, t("greet_lenix"), true).await
}

fn commands() -> Vec<poise::Command<Data, Error>> {
    let mut ping = ping();
    ping.description = Some(t("reply_pong").to_string());

    let mut lenix = lenix();
    lenix.description = Some(t("reply_lenix").to_string());

    vec![ping, lenix]
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let token = std::env::var("TOKEN").expect("TOKEN is missing from .env");
    let intents = serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
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
