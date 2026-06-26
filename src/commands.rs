use crate::{
  locales::t,
  types::{Context, Data, Error},
  forums::send_ai_space_prompt,
};

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  ctx
    .send(
      poise::CreateReply::default()
        .content(t("pong"))
        .ephemeral(true),
    )
    .await?;

  Ok(())
}

#[poise::command(slash_command)]
async fn lenix(ctx: Context<'_>) -> Result<(), Error> {
  ctx
    .send(
      poise::CreateReply::default()
        .content(t("greet_lenix"))
        .ephemeral(true),
    )
    .await?;

  Ok(())
}

#[poise::command(slash_command)]
async fn ai_space(ctx: Context<'_>) -> Result<(), Error> {
  send_ai_space_prompt(ctx).await
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
  let mut ping = ping();
  ping.description = Some(t("reply_pong").to_string());

  let mut lenix = lenix();
  lenix.description = Some(t("reply_lenix").to_string());

  let mut ai_space = ai_space();
  ai_space.name = "ai-space".to_string();
  ai_space.description = Some(t("reply_ai_space").to_string());

  vec![ping, lenix, ai_space]
}
