import type { ChatInputCommandInteraction, CacheType } from "discord.js"
import locales from "./locales.json"
import { replyToCommand } from "./methods"

export const APP_ID = "1212430652134129664"

export const CUSTOMERS = [
	"1244750233582440488"
]

export const COMMANDS = [
	{
		name: "ping",
		description: locales.reply_pong,
		action: async (interaction: ChatInputCommandInteraction<CacheType>) => await replyToCommand(interaction, locales.pong)
	},
	{
		name: "lenix",
		description: locales.reply_lenix,
		action: async (interaction: ChatInputCommandInteraction<CacheType>) => await replyToCommand(interaction, locales.greet_lenix)
	}
]

export const VERSION = "10"