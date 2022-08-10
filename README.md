<h1 align="center">
  <br><img src="project-logo.svg" height="192px">
  <br>
  sync-twitch-events
  <br>
</h1>

<h4 align="center">A CLI tool to create Discord scheduled event from a Twitch schedule.</h4>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/damoun/sync-twitch-events.svg"></a>
  <a href="https://github.com/damoun/sync-twitch-events/actions/workflows/build.yml">
      <img src="https://github.com/damoun/sync-twitch-events/actions/workflows/build.yml/badge.svg">
  </a>
</p>

<p align="center">
  <a href="#getting-started">Getting Started</a> •
  <a href="#contributing">Contributing</a> •
  <a href="CHANGELOG.md">Changelog</a>
</p>

## Getting Started

This repository can be use by anyone as a Github Template.
Click on the green button `Use this template`.

A few variables have to be set for your usage and so, please configure the `.github/workflows/sync.yml` workflow file. The following variable have to be change:

```diff
TWITCH_BROADCASTER_ID: "29776980"
DISCORD_EVENT_LOCATION: https://twitch.tv/dam0un
DISCORD_GUILD_ID: "306182341888442368"
```
Do not remove the `"` !

Then, create a Twitch application on [dev.twitch.tv](https://dev.twitch.tv/console/apps/create) and set the following variable in your Github repository settings as Secrets for Actions:

- TWITCH_CLIENT_ID
- TWITCH_CLIENT_SECRET

You also have to create an application on [discord.com/developers](https://discord.com/developers/applications) and invite your bot to your Discord server with the permission to manage events. The following variable need to be set in your Github repository settings as Secrets for Actions:

- DISCORD_TOKEN

You can now run the sync workflow manualy in your Github repository.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

> or if you like it simple:

1. `Fork` this repository
2. Create a `branch`
3. `Commit` your changes
4. `Push` your `commits` to the `branch`
5. Submit a `pull request`

> You can find more information about Pull Requests [here](https://help.github.com/categories/collaborating-on-projects-using-pull-requests/)

Check also the [list of contributors](AUTHOR.md#contributors) who helped on this project.
