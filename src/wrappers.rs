use crate::{
  constants::CUSTOMERS,
  locales::t,
  types::{Context, Error},
};
use poise::serenity_prelude as serenity;

pub async fn reply_to_component(
  ctx: &serenity::Context,
  component: &serenity::ComponentInteraction,
  content: impl Into<String>,
  ephemeral: bool,
) -> Result<(), Error> {
  component
    .create_response(
      ctx,
      serenity::CreateInteractionResponse::Message(
        serenity::CreateInteractionResponseMessage::new()
          .content(content)
          .ephemeral(ephemeral),
      ),
    )
    .await?;

  Ok(())
}

pub async fn customer_only(ctx: Context<'_>) -> Result<bool, Error> {
  if ctx
    .guild_id()
    .is_some_and(|guild_id| CUSTOMERS.contains(&guild_id.get()))
  {
    return Ok(true);
  }

  ctx
    .send(
      poise::CreateReply::default()
        .content(t("not_eligible"))
        .ephemeral(true),
    )
    .await?;

  Ok(false)
}