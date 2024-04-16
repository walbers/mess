use std::process::{Command, Stdio, exit};
use std::time::Instant;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::env;
use ini::Ini;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

const AVAILABLE_MESSENGERS: [&str; 1] = ["desktop"];

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
    let home_dir = env::var("HOME").unwrap();
    let config_path = format!("{}/.mess.ini", home_dir);
    let conf = Ini::load_from_file(config_path).unwrap();

    let mut mess_settings: HashMap<String, String> = HashMap::new();
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
            mess_settings.insert(key.to_string(), value.to_string());
        }
    }

    let env_settings = get_mess_env_settings();
    for (key, value) in env_settings {
        mess_settings.insert(key, value);
    }

    mess_settings
}

fn get_messengers(mess_variables: &HashMap<String, String>) -> Vec<String> {
    if !mess_variables.contains_key("MESSENGERS") {
        eprintln!("No messengers found!");
        exit(1);
    }

    let messengers_str =  mess_variables.get("MESSENGERS").unwrap();
    let messengers: Vec<String> = messengers_str.split(',').map(|s| s.to_string()).collect();
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

    let duration_allowed_str = mess_variables.get("DURATION").unwrap();
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
            Ok(line) => println!("Output: {}", line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    let _status = child.wait().expect("Command wasn't running");
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

async fn send_discord_message(program_name: &String, duration: u64, mess_settings: HashMap<String, String>) {

}

async fn send_message(program_name: &String, duration: u64, mess_settings: HashMap<String, String>) {
    let messengers = get_messengers(&mess_settings);
    for messenger in messengers {
        match &messenger as &str {
            "desktop" => send_desktop_message(&program_name, duration),
            "discord" => send_discord_message(&program_name, duration, mess_settings),
            _ => eprint!("The messenger {} doesn't exist. Skipping...", messenger),
        }
    }
}
#[tokio::main]
async fn main() {
    // HANDLE case just `mess` or `mess --help --version and --test to make sure message works`

    // collect arguements and settings
    let mess_settings = get_settings();
    let messengers = get_messengers(&mess_settings);
    let duration_allowed = get_duration_allowed(&mess_settings);
    let args_passed: Vec<String> = env::args().collect();

    // run the program
    let start = Instant::now();
    run_the_program(&args_passed[1..]);
    let duration = start.elapsed();

    // send message
    println!("-------------------");
    println!("Duration: {:?}", duration);
    println!("Duration allowed: {:?}", duration_allowed);
    if duration.as_secs() >= duration_allowed { // TODO: *60
        send_message(&args_passed[1], duration.as_secs(), mess_settings);
    } else {
        println!("Execution was less than allowed duration. No message sent.");
    }
}
