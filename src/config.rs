use crate::{locales::t, types::CommandConfig, wrappers::reply_to_command};

pub const COMMANDS: &[CommandConfig] = &[
  CommandConfig {
    name: "ping",
    description_key: "reply_pong",
    action: |ctx| Box::pin(async move { reply_to_command(ctx, t("pong"), true).await }),
  },
  CommandConfig {
    name: "lenix",
    description_key: "reply_lenix",
    action: |ctx| Box::pin(async move { reply_to_command(ctx, t("greet_lenix"), true).await }),
  },
];
