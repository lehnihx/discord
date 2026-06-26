use crate::{
  constants::CUSTOMERS,
  locales::t,
  types::{Context, Error},
};
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
