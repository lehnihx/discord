use poise::serenity_prelude as serenity;

use crate::{
  locales::t,
  types::{Context, Error},
};

pub async fn handle_event(
  ctx: &serenity::Context,
  event: &serenity::FullEvent,
) -> Result<(), Error> {
  let serenity::FullEvent::InteractionCreate { interaction } = event else {
    return Ok(());
  };

  let Some(component) = interaction.as_message_component() else {
    return Ok(());
  };

  if component.data.custom_id != "create_ai_space" {
    return Ok(());
  }

  create_ai_space(ctx, component).await
}

pub async fn send_ai_space_prompt(ctx: Context<'_>) -> Result<(), Error> {
  ctx
    .send(
      poise::CreateReply::default()
        .content(t("ai_space_prompt"))
        .ephemeral(false)
        .components(vec![serenity::CreateActionRow::Buttons(vec![
          serenity::CreateButton::new("create_ai_space")
            .label(t("ai_space_button"))
            .style(serenity::ButtonStyle::Primary),
        ])]),
    )
    .await?;

  Ok(())
}

async fn create_ai_space(
  ctx: &serenity::Context,
  component: &serenity::ComponentInteraction,
) -> Result<(), Error> {
  component.defer_ephemeral(ctx).await?;

  let Some(guild_id) = component.guild_id else {
    component
      .edit_response(
        ctx,
        serenity::EditInteractionResponse::new().content(t("ai_space_missing_guild")),
      )
      .await?;

    return Ok(());
  };

  let channel_name = format!("ai-{}", component.user.id.get());
  let channels = match guild_id.channels(&ctx.http).await {
    Ok(channels) => channels,
    Err(error) => {
      respond_with_ai_space_error(ctx, component, &error).await?;
      return Ok(());
    }
  };

  if let Some(channel) = channels
    .values()
    .find(|channel| channel.kind == serenity::ChannelType::Forum && channel.name == channel_name)
  {
    respond_with_ai_space(ctx, component, channel.id).await?;
    return Ok(());
  }

  let bot_user_id = ctx.cache.current_user().id;
  let forum = match guild_id
    .create_channel(
      &ctx.http,
      serenity::CreateChannel::new(&channel_name)
        .kind(serenity::ChannelType::Forum)
        .topic(format!("Private AI space for {}", component.user.name))
        .permissions(vec![
          serenity::PermissionOverwrite {
            allow: serenity::Permissions::empty(),
            deny: serenity::Permissions::VIEW_CHANNEL,
            kind: serenity::PermissionOverwriteType::Role(guild_id.everyone_role()),
          },
          serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL
              | serenity::Permissions::SEND_MESSAGES
              | serenity::Permissions::READ_MESSAGE_HISTORY
              | serenity::Permissions::CREATE_PUBLIC_THREADS
              | serenity::Permissions::SEND_MESSAGES_IN_THREADS,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Member(component.user.id),
          },
          serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL
              | serenity::Permissions::SEND_MESSAGES
              | serenity::Permissions::READ_MESSAGE_HISTORY
              | serenity::Permissions::MANAGE_THREADS
              | serenity::Permissions::CREATE_PUBLIC_THREADS
              | serenity::Permissions::SEND_MESSAGES_IN_THREADS,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Member(bot_user_id),
          },
        ])
        .audit_log_reason("Create private AI space"),
    )
    .await
  {
    Ok(forum) => forum,
    Err(error) => {
      respond_with_ai_space_error(ctx, component, &error).await?;
      return Ok(());
    }
  };

  respond_with_ai_space(ctx, component, forum.id).await
}

async fn respond_with_ai_space(
  ctx: &serenity::Context,
  component: &serenity::ComponentInteraction,
  channel_id: serenity::ChannelId,
) -> Result<(), Error> {
  component
    .edit_response(
      ctx,
      serenity::EditInteractionResponse::new().content(format!(
        "{} <#{}>",
        t("ai_space_created"),
        channel_id.get()
      )),
    )
    .await?;

  Ok(())
}

async fn respond_with_ai_space_error(
  ctx: &serenity::Context,
  component: &serenity::ComponentInteraction,
  error: &serenity::Error,
) -> Result<(), Error> {
  component
    .edit_response(
      ctx,
      serenity::EditInteractionResponse::new().content(format!(
        "{} {}",
        t("ai_space_failed"),
        error
      )),
    )
    .await?;

  Ok(())
}
