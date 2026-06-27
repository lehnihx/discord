use poise::serenity_prelude as serenity;

use crate::{ai, constants::AI_SPACES_CATEGORY_ID, locales::t, types::Error};

pub async fn handle_event(
  ctx: &serenity::Context,
  event: &serenity::FullEvent,
) -> Result<(), Error> {
  match event {
    serenity::FullEvent::InteractionCreate { interaction } => {
      let Some(component) = interaction.as_message_component() else {
        return Ok(());
      };

      if component.data.custom_id != "create_ai_space" {
        return Ok(());
      }

      create_ai_space(ctx, component).await
    }
    serenity::FullEvent::Message { new_message } => handle_ai_space_message(ctx, new_message).await,
    _ => Ok(()),
  }
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

  let category_id = if AI_SPACES_CATEGORY_ID == 0 {
    None
  } else {
    let category_id = serenity::ChannelId::new(AI_SPACES_CATEGORY_ID);

    if !channels
      .values()
      .any(|channel| channel.kind == serenity::ChannelType::Category && channel.id == category_id)
    {
      component
        .edit_response(
          ctx,
          serenity::EditInteractionResponse::new().content(t("ai_space_missing_category")),
        )
        .await?;

      return Ok(());
    }

    Some(category_id)
  };

  let mut builder = serenity::CreateChannel::new(&channel_name)
    .kind(serenity::ChannelType::Forum)
    .topic(format!("Private AI space for {}", component.user.name));

  if let Some(category_id) = category_id {
    builder = builder.category(category_id);
  }

  let bot_user_id = ctx.cache.current_user().id;
  let forum = match guild_id
    .create_channel(
      &ctx.http,
      builder
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

async fn handle_ai_space_message(
  ctx: &serenity::Context,
  message: &serenity::Message,
) -> Result<(), Error> {
  if message.author.bot {
    return Ok(());
  }

  let Some(guild_id) = message.guild_id else {
    return Ok(());
  };

  let serenity::Channel::Guild(thread) = message.channel_id.to_channel(ctx).await? else {
    return Ok(());
  };

  if thread.thread_metadata.is_none() {
    return Ok(());
  }

  let Some(parent_id) = thread.parent_id else {
    return Ok(());
  };

  let channels = guild_id.channels(&ctx.http).await?;
  let Some(parent) = channels.get(&parent_id) else {
    return Ok(());
  };

  if parent.kind != serenity::ChannelType::Forum || !parent.name.starts_with("ai-") {
    return Ok(());
  }

  let reply = match ai::generate_reply(
    &thread.name,
    &message.content,
    message.author.id.get(),
    message.author.display_name(),
  )
  .await
  {
    Ok(reply) => reply,
    Err(error) => format!("{} {}", t("ai_failed"), error),
  };

  message.reply(ctx, reply).await?;

  Ok(())
}
