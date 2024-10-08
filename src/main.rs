use std::process::{Command, Stdio, exit};
use std::time::Instant;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::env;
use ini::Ini;
use serenity::http::Http as SerenityHttp;
use serenity::model::id::ChannelId;
use nix::sys::utsname::uname;
use serde::{Deserialize, Serialize};

const AVAILABLE_MESSENGERS: [&str; 5] = ["BEEP", "DESKTOP", "DISCORD", "TEXT", "SLACK"];

#[derive(Serialize, Deserialize)]
struct SlackMessage {
    channel: String,
    text: String,
}

fn get_mess_env_settings() -> HashMap<String, String> {
    let mut mess_variables: HashMap<String, String> = HashMap::new();
    for (key, value) in env::vars() {
        if key.starts_with("MESS_") {
            mess_variables.insert(key[5..].to_string(), value);
        }
    }
    mess_variables
}


fn get_settings() -> HashMap<String, String> {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let config_path = format!("{}/.config/mess", home_dir);
    let conf = Ini::load_from_file(config_path).expect("Failed to load config file");

    // ~/.config/mess file settings
    let mut mess_settings: HashMap<String, String> = HashMap::new();
    for (_sec, prop) in &conf {
        for (key, value) in prop.iter() {
            mess_settings.insert(key.to_string().to_uppercase(), value.to_string());
        }
    }

    // Environment variables settings
    let env_settings = get_mess_env_settings();
    for (key, value) in env_settings {
        mess_settings.insert(key.to_uppercase(), value);
    }

    mess_settings
}

fn get_messengers(mess_variables: &HashMap<String, String>) -> Vec<String> {
    if !mess_variables.contains_key("MESSENGERS") {
        eprintln!("No messengers found!");
        exit(1);
    }

    let messengers_str =  mess_variables.get("MESSENGERS").expect("Failed to get messengers");
    let messengers: Vec<String> = messengers_str.split(',').map(|s| s.to_string().replace(" ", "").to_uppercase()).collect();
    for messenger in &messengers {
        if !AVAILABLE_MESSENGERS.contains(&messenger.as_str()) {
            eprintln!("Messenger {} not available!", messenger);
            exit(1);
        }
    }
    messengers
}

fn get_duration_allowed(mess_variables: &HashMap<String, String>) -> u64 {
    if !mess_variables.contains_key("DURATION") {
        eprintln!("No duration allowed found!");
        exit(1);
    }

    let duration_allowed_str = mess_variables.get("DURATION").expect("Failed to get duration allowed");
    let duration_allowed = duration_allowed_str.parse::<u64>().unwrap_or_else(|e| {
        eprint!("Failed to parse DURATION to integer: {}", e);
        exit(1);
    });
    duration_allowed
}

fn run_the_program(program_to_run: &[String]) {
    let mut child = Command::new(&program_to_run[0])
    .args(&program_to_run[1..])
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute command");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        match line {
            Ok(_) => {},
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let _status = child.wait().expect("Command wasn't running");
}

fn send_beep_message() {
    let uname = uname().expect("Failed to get uname");
    let kernel_name = uname.release().to_str().expect("Failed to get kernel name");
    if kernel_name.contains("WSL") {
        Command::new("powershell.exe")
        .arg("-c")
        .arg("[console]::beep(1000, 500)")
        .output()
        .expect("Failed to execute command");
    }
    else {
        print!("\x07");
    }
}

fn send_desktop_message(program_name: &String, duration: u64) {
    let message = format!("Your program {} has finished after {} minutes", program_name, duration);
    let mut child = Command::new("notify-send")
    .arg("--category")
    .arg("Mess")
    .arg(&message)
    .spawn()
    .expect("Failed to execute command");

    let _status = child.wait().expect("Command wasn't running");
}

async fn send_discord_message(program_name: &String, duration: u64, mess_settings: &HashMap<String, String>) {
    let token = mess_settings.get("DISCORD_TOKEN").expect("Failed to get Discord token");
    let channel_id = mess_settings.get("DISCORD_CHANNEL_ID").expect("Failed to get Discord channel id");
    let message = format!("Your program {} has finished after {} minutes", program_name, duration);
    let http = SerenityHttp::new(&token);
    let channel = ChannelId::new(channel_id.parse::<u64>().expect("Failed to parse channel id"));
    match channel.say(&http, &message).await {
        Ok(message) => println!("Message sent to Discord: {:?}", message.content),
        Err(why) => eprintln!("Error sending message to Discord: {:?}", why),
    }
}

async fn send_slack_message(program_name: &String, duration: u64, mess_settings: &HashMap<String, String>) {
    let message = SlackMessage {
        channel: mess_settings.get("SLACK_CHANNEL_ID").expect("Failed to get Slack channel").to_string(),
        text: format!("Your program {} has finished after {} minutes", program_name, duration),
    };
    let slack_token = mess_settings.get("SLACK_BOT_TOKEN").
        expect("Failed to get Slack token");
    let slack_client = reqwest::Client::new();
    let _res = slack_client.post("https://slack.com/api/chat.postMessage")
        .bearer_auth(slack_token)
        .json(&message)
        .send()
        .await;

    // println!("Result {:?}", _res);
}

async fn send_text_message(program_name: &String, duration: u64, mess_settings: &HashMap<String, String>) {
    let sender = mess_settings.get("TWILIO_SENDER").expect("Failed to get Twilio sender");
    let receiver = mess_settings.get("TWILIO_RECEIVER").expect("Failed to get Twilio receiver");
    let account = mess_settings.get("TWILIO_ACCOUNT").expect("Failed to get Twilio account");
    let api_key = mess_settings.get("TWILIO_API_KEY").expect("Failed to get Twilio api key");
    let message = format!("Your program {} has finished after {} minutes", program_name, duration);

    let form_data = [("To", receiver), ("From", sender), ("Body", &message)];

    let client = reqwest::Client::new();
    let _res = client.post(
        format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account))
        .form(&form_data)
        .basic_auth(account, Some(api_key))
        .send()
        .await;
    // println!("Result {:?}", res)
}

async fn send_message(program_name: &String, duration: u64, mess_settings: HashMap<String, String>) {
    let messengers = get_messengers(&mess_settings);
    for messenger in messengers {
        match &messenger as &str {
            "BEEP" => send_beep_message(),
            "DESKTOP" => send_desktop_message(&program_name, duration),
            "DISCORD" => send_discord_message(&program_name, duration, &mess_settings).await,
            "SLACK" => send_slack_message(&program_name, duration, &mess_settings).await,
            "TEXT" => send_text_message(&program_name, duration, &mess_settings).await,
            _ => eprint!("The messenger {} doesn't exist. Skipping...", messenger),
        }
    }
}

#[tokio::main]
async fn main() {
    // collect arguements and settings
    let mut debug = false;
    let mess_settings = get_settings();
    if let Some(v) = mess_settings.get("DEBUG") {
        if v.to_uppercase() == "TRUE" {
            debug = true;
        }
    }
    let messengers = get_messengers(&mess_settings); // to check if messengers are valid
    let duration_allowed = get_duration_allowed(&mess_settings);
    let args_passed: Vec<String> = env::args().collect();

    if args_passed.len() == 1 {
        let mess_version = env!("CARGO_PKG_VERSION");
        println!("Mess version: {}", mess_version);
        println!("Make sure to setup ~/.config/mess file check github.com/walbers/mess for more info");
        println!("Usage: mess <program> <program-args>");
        exit(0);
    }

    // run the program
    let start = Instant::now();
    run_the_program(&args_passed[1..]);
    let duration = start.elapsed();

    // send message
    if debug {
        println!("-------------------");
        println!("Duration: {:?}", duration);
        println!("Duration allowed: {:?}", duration_allowed);
        println!("Messengers: {:?}", messengers);
    }

    if duration.as_secs() >= duration_allowed {
        send_message(&args_passed[1], duration.as_secs(), mess_settings).await;
    } else if debug {
        println!("Execution was less than allowed duration. No message sent.");
    }
}
