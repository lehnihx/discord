use crate::{constants::{CUSTOMERS}, locales::t, types::{ActionFn, Context, Data, Error}, config::{COMMANDS}};

pub async fn reply_to_command(
    ctx: Context<'_>,
    content: &str,
    ephemeral: bool,
) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .content(content)
            .ephemeral(ephemeral),
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

    reply_to_command(ctx, t("not_eligible"), true).await?;

    Ok(false)
}

fn run_command_action<'a>(
    ctx: poise::ApplicationContext<'a, Data, Error>,
) -> poise::BoxFuture<'a, Result<(), poise::FrameworkError<'a, Data, Error>>> {
    Box::pin(async move {
        let Some(action) = ctx.command.custom_data.downcast_ref::<ActionFn>() else {
            let error =
                std::io::Error::new(std::io::ErrorKind::Other, "missing command action metadata");

            return Err(poise::FrameworkError::new_command(
                poise::Context::Application(ctx),
                error.into(),
            ));
        };

        action(poise::Context::Application(ctx))
            .await
            .map_err(|error| {
                poise::FrameworkError::new_command(poise::Context::Application(ctx), error)
            })
    })
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    COMMANDS
        .iter()
        .map(|config| poise::Command {
            name: config.name.to_string(),
            qualified_name: config.name.to_string(),
            identifying_name: config.name.to_string(),
            source_code_name: config.name.to_string(),
            description: Some(t(config.description_key).to_string()),
            slash_action: Some(run_command_action),
            custom_data: Box::new(config.action),
            ..Default::default()
        })
        .collect()
}
