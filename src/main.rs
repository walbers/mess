use std::process::{Command, Stdio, exit};
use std::time::Instant;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::env;

const AVAILABLE_MESSENGERS: [&str; 1] = ["desktop"];

fn get_mess_env_variables() -> HashMap<String, String> {
    let mut mess_variables: HashMap<String, String> = HashMap::new();
    for (key, value) in env::vars() {
        if key.starts_with("MESS_") {
            mess_variables.insert(key, value);
        }
    }
    mess_variables
}

fn get_messengers(mess_variables: &HashMap<String, String>) -> Vec<String> {
    if !mess_variables.contains_key("MESS_MESSENGERS") {
        eprintln!("No messengers found!");
        exit(1);
    }

    let messengers_str =  mess_variables.get("MESS_MESSENGERS").unwrap();
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
    if !mess_variables.contains_key("MESS_DURATION") {
        eprintln!("No duration allowed found!");
        exit(1);
    }

    let duration_allowed_str = mess_variables.get("MESS_DURATION").unwrap();
    let duration_allowed = duration_allowed_str.parse::<u64>().unwrap_or_else(|e| {
        eprint!("Failed to parse MESS_DURATION to integer: {}", e);
        exit(1);
    });
    duration_allowed
}

fn run_the_program(program_to_run: &[String]) {
    // println!("{}", program_to_run[0].clone());
    // println!("{}", program_to_run);
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

fn send_notify_send_message(program_name: &String, duration: u64) {
    let message = format!("Your program {} has finished after {} minutes", program_name, duration);
    let mut child = Command::new("notify-send")
    .arg("--category")
    .arg("Mess")
    .arg(&message)
    .spawn()
    .expect("Failed to execute command");

    let _status = child.wait().expect("Command wasn't running");
}

fn send_message(messengers: Vec<String>, program_name: &String, duration: u64) {
    for messenger in messengers {
        match &messenger as &str {
            "desktop" => send_notify_send_message(&program_name, duration),
            _ => eprint!("The messenger {} doesn't exist. Skipping...", messenger),
        }
    }
}

fn main() {
    // collect arguements and settings
    let mess_env_variables = get_mess_env_variables();
    let messengers = get_messengers(&mess_env_variables);
    let duration_allowed = get_duration_allowed(&mess_env_variables);


    let args_passed: Vec<String> = env::args().collect();
    let program_to_run = &args_passed[1..];
    // program_to_run.remove(0); // Starts with the name of this program
    let program_name = &program_to_run[0];

    // run the program
    let start = Instant::now();
    run_the_program(program_to_run);
    let duration = start.elapsed();

    // send message
    println!("-------------------");
    println!("Duration: {:?}", duration);
    println!("Duration allowed: {:?}", duration_allowed);
    if duration.as_secs() >= duration_allowed { // TODO: *60
        send_message(messengers, program_name, duration.as_secs());
    } else {
        println!("Execution was less than allowed duration. No message sent.");
    }
}
