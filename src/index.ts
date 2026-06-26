import {
	Client,
	GatewayIntentBits,
	Events,
} from "discord.js";
import dotenv from "dotenv";
import locales from "./locales.json"
import { COMMANDS, CUSTOMERS } from "./constants";
import { replyToCommand } from "./methods";

dotenv.config();

const client = new Client({
	intents: [GatewayIntentBits.Guilds],
});

client.once(Events.ClientReady, (client) => {
	console.log(`${locales.logged_in_as} ${client.user.tag}`);
});

client.on(Events.InteractionCreate, async (interaction) => {
	if (!interaction.isChatInputCommand()) return;
	if (!CUSTOMERS.includes(String(interaction.guildId))) {
		replyToCommand(interaction, locales.not_eligible)
		return
	}

	const command = COMMANDS.find(command => command.name === interaction.commandName);

	if (command?.action) await command.action(interaction);
});

client.login(process.env.TOKEN);