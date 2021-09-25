# discord-everything-bot
Discord bot intended to be a self-hosted replication of MEE6. WIP

The bot is currently named "Bingus Bot". Don't ask...

# Feature Checklist

* [ ] Welcome
 - * [x] Message when user joins
 - * [ ] DM when user joins
 - * [ ] Assign roles when user joins
 - * [ ] DM when user leaves
* [ ] Custom Commands
* [ ] **(WIP)** Reaction Roles
* [ ] Moderator
 - * [ ] Moderator Roles
 - * [ ] Audit Logging
 - * [ ] AutoMod
 - * [ ] AutoMod Actions
 - * [ ] Restricted Channels
 - * [ ] Commands
* [ ] Music

# Setup
Until containers are setup, it's best to run the two services separately.

In fact, I'm confident that it currently **only works in a dev environment**.

## Environment Config
You need a `.env` file for each project.

The `server` project needs the following environment variables:
- `DISCORD_TOKEN` - Your Discord Application token
- `DISCORD_APP_ID` - Your Discord Application ID
- `DATABASE_URL` - The path to your sqlite db file. Format: "sqlite:<absolute path to file>"
- `DEFAULT_GUILD_ID` - Currently the bot only works with one guild, the id for that guild should be entered here
- `IMGUR_CLIENT_ID` - To avoid having to manage image hosting ourselves, you can enter an imgur client id here to host there.
> There is currently no other way to send images in embeds without offloading hosting to imgur.

The `ui` project needs the following environment variables:
- `VITE_DEFAULT_GUILD_ID` - Currently the bot only works with one guild, the id for that guild should be entered here

## API
Currently the database is sqlite. Migrations aren't implemented yet, so an empty db file is included. To run the server, just start with cargo.

```bash
$ /server> cargo run
```

## UI
The UI is built with Sveltekit. Run the dev server to get started.

```bash
$ /ui> npm i

$ /ui> npm run dev
```

From this point you should be able to navigate to localhost:3000 and start playing around with the UI.
