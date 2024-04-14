# THIS IS CURRENTLY A DRAFT
# Mess - a utility for sending a message when a long job finishes

Building projects can take a long time and I wanted to get a notification when the build finishes

## To Install

- Build
    - `cargo build`
- Run
    - `MESS_DURATION=10 MESS_MESSENGER=desktop mess/target/debug/mess <your bash command with args>`
- For WSL you will need to setup `wsl-notify-send` for desktop notifications. You can follow this [blog](https://stuartleeks.com/posts/wsl-github-cli-windows-notifications-part-1/) or my steps below
    - `wget https://github.com/stuartleeks/wsl-notify-send/releases/download/v0.1.871612270/wsl-notify-send_windows_amd64.zip`
    - `unzip wsl-notify-send_windows_amd64.zip`
    - Add the path of `wsl-notify-send.exe` to `src/main.rs:73` and rebuild

## Integrations

### Current (In Progress)

- Desktop notifications
    - Will work in WSL with [wsl-notify-send](https://github.com/stuartleeks/wsl-notify-send)

### Future

- Discord
- Push notification
- Slack
- Text
    - requires twilio API key
