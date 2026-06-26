use crate::types::{Context, Error};

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
