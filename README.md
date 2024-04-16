# THIS IS CURRENTLY A DRAFT
# Mess - a utility for sending a message when a long job finishes

Building projects can take a long time and I wanted to get a notification when the build finishes

## To Install

- Build
    - `cargo build`
    - `sudo cp target/debug/mess /usr/local/bin/mess`
- Config
    - edit `mess.ini` and copy it to `~/.mess.ini`
- Run
    - `mess <bash command>`
- For WSL you will need to setup `wsl-notify-send` for desktop notifications. You can follow this [blog](https://stuartleeks.com/posts/wsl-github-cli-windows-notifications-part-1/) or my steps below

    ```bash
    wget https://github.com/stuartleeks/wsl-notify-send/releases/download/v0.1.871612270/wsl-notify-send_windows_amd64.zip
    unzip wsl-notify-send_windows_amd64.zip
    sudo cp wsl-notify-send.exe /usr/local/bin/notify-send
    ```

## Integrations

### Current (In Progress)

- Desktop notifications
    - Will work in WSL with [wsl-notify-send](https://github.com/stuartleeks/wsl-notify-send)
- Discord

### Future

- Slack
- Push notification
- Text
    - requires twilio API key
