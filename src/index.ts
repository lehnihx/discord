import {
	Client,
	GatewayIntentBits,
	Events,
} from "discord.js";
import dotenv from "dotenv";
import locales from "./locales.json"
import { COMMANDS } from "./constants";

dotenv.config();

const client = new Client({
	intents: [GatewayIntentBits.Guilds],
});

client.once(Events.ClientReady, (client) => {
	console.log(`${locales.logged_in_as} ${client.user.tag}`);
});

client.on(Events.InteractionCreate, async (interaction) => {
	if (!interaction.isChatInputCommand()) return;

	const command = COMMANDS.find(command => command.name === interaction.commandName);

	if (command?.action) await command.action(interaction);
});

client.login(process.env.TOKEN);