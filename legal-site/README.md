# Legal Site

Static Terms of Service and Privacy Policy site for Discord app verification.

## Edit the legal details

Update `src/legal.ts` before publishing:

- `app.name`
- `app.owner`
- `app.contact`
- any policy text that does not match how the bot actually stores or handles data

## Local development

```bash
bun install
bun run dev
```

## Build

```bash
bun run build
```

The build creates direct static routes at:

- `/terms/`
- `/privacy/`

## GitHub Pages

The included workflow deploys `legal-site/dist` to GitHub Pages. In the GitHub repository settings, set Pages source to **GitHub Actions**.

After deployment, use these Discord Developer Portal URLs:

- `https://YOUR_GITHUB_USERNAME.github.io/YOUR_REPOSITORY_NAME/terms/`
- `https://YOUR_GITHUB_USERNAME.github.io/YOUR_REPOSITORY_NAME/privacy/`
