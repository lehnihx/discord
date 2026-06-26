import { REST, Routes, SlashCommandBuilder } from "discord.js";
import dotenv from "dotenv";
import { COMMANDS, APP_ID, VERSION } from "./constants";
import locales from "./locales.json"

dotenv.config();

const commands = COMMANDS.map(({ name, description }) => (
	new SlashCommandBuilder()
    .setName(name)
    .setDescription(description)
    .toJSON()
))

const rest = new REST({ version: VERSION }).setToken(process.env.TOKEN!);

await rest.put(
  Routes.applicationCommands(APP_ID),
  { body: commands }
);

console.log(locales.register_success);