import type { ChatInputCommandInteraction, CacheType } from "discord.js";

export const replyToCommand = async (interaction: ChatInputCommandInteraction<CacheType>, content: string, ephemeral = true) => interaction.reply({ content, ephemeral })