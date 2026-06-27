import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const dist = "dist";
const html = await readFile(join(dist, "index.html"), "utf8");
const nestedHtml = html.split("./assets/").join("../assets/");

for (const route of ["terms", "privacy"]) {
  await mkdir(join(dist, route), { recursive: true });
  await writeFile(join(dist, route, "index.html"), nestedHtml);
}

await writeFile(join(dist, ".nojekyll"), "");
