# Mess

This is a CLI tool to send you a [notification](#integrations) after your command is finished.

I built this because I often have long running compilations, docker builds, ansible jobs, etc. and wanted to get notified when they are done instead of baby sitting them. It's especially horrible when you expect something to run for a few hours and it errors 20 minutes in.

By default it will send desktop notifications and not have any delay.

## Install

`curl -fsSL https://github.com/walbers/mess/releases/download/latest/install.sh | bash`

## Setup

Edit `~/.mess/config` to set how long a program must run for it to trigger a message when it ends and for the type of integration you want. Mess currently supports desktop notifications and text through Twilio. I am working on adding a discord and slack bot.
- DURATION is the amount of seconds a program can take until mess sends you a notification when it finishes. If you set it to 10 and your command fails after 5 seconds you will not get a notification. 0 means you will always get notified.
- MESSENGERS is the list of ways you want to get notifications. MESSENGER="text" will send you a text message through twilio so you must fill out the TWILIO_ fields in the config. Twilio provides gives you $15 free which is about 2000 texts. You can also provide a list of messengers so you can get both desktop and text notifications `MESSENGERS="desktop,text"`

## Usage

Use it just like you would the `time` command

- `mess <some bash command>`
- `mess find / -name ".log"`
- `mess docker build .`


## Integrations

### Current

- Desktop notifications
    - Uses linux's `notify-send`
    - Will work in WSL with [wsl-notify-send](https://github.com/stuartleeks/wsl-notify-send)
- Text
    - requires twilio API key

### Future

- Slack bot
- Discord bot


## Building

- [Install rust]((https://www.rust-lang.org/))
- Build
    - `git clone https://github.com/walbers/mess.git`
    - `cd mess`
    - `cargo build --release`
    - `sudo cp target/release/mess /usr/local/bin/mess`
- Config
    - Edit `config`
    - `mkdir ~/.mess`
    - `cp config ~/.mess/config`
- Run
    - `mess <bash command>`

- For WSL you will need to setup `wsl-notify-send` for desktop notifications. You can follow this [blog](https://stuartleeks.com/posts/wsl-github-cli-windows-notifications-part-1/) or my steps below
    ```bash
    wget https://github.com/stuartleeks/wsl-notify-send/releases/download/v0.1.871612270/wsl-notify-send_windows_amd64.zip
    unzip wsl-notify-send_windows_amd64.zip
    sudo cp wsl-notify-send.exe /usr/local/bin/notify-send
    ```

