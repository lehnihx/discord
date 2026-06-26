use poise::serenity_prelude as serenity;

const CUSTOMERS: &[u64] = &[1244750233582440488];

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn is_customer(ctx: Context<'_>) -> bool {
    ctx.guild_id()
        .is_some_and(|guild_id| CUSTOMERS.contains(&guild_id.get()))
}

async fn reject_if_not_customer(ctx: Context<'_>) -> Result<bool, Error> {
    if is_customer(ctx) {
        return Ok(false);
    }

    ctx.send(
        poise::CreateReply::default()
            .content("Server not eligible")
            .ephemeral(true),
    )
    .await?;

    Ok(true)
}

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    if reject_if_not_customer(ctx).await? {
        return Ok(());
    }

    ctx.send(
        poise::CreateReply::default()
            .content("Pong!")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn lenix(ctx: Context<'_>) -> Result<(), Error> {
    if reject_if_not_customer(ctx).await? {
        return Ok(());
    }

    ctx.send(
        poise::CreateReply::default()
            .content("Hi Lenix!")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let token = std::env::var("TOKEN").expect("TOKEN is missing from .env");
    let intents = serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), lenix()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
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
