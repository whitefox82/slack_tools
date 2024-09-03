use clap::{Arg, Command};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info, warn, LevelFilter};
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting slack_messenger...");

    let matches = Command::new("webhook_service")
        .version("1.0")
        .about("A CLI app for sending messages to Slack channels")
        .arg(
            Arg::new("message")
                .long("message")
                .short('m')
                .help("The message to send")
                .required(true),
        )
        .arg(
            Arg::new("channel")
                .long("channel")
                .short('c')
                .help("The Slack channel to send the message to")
                .required(true),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Sets the level of logging to DEBUG")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("info")
                .long("info")
                .help("Sets the level of logging to INFO")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("warn")
                .long("warn")
                .help("Sets the level of logging to WARN")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("error")
                .long("error")
                .help("Sets the level of logging to ERROR")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("off")
                .long("off")
                .help("Turns off logging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Set the logging level based on the provided flags
    let log_level = if matches.get_flag("debug") {
        LevelFilter::Debug
    } else if matches.get_flag("info") {
        LevelFilter::Info
    } else if matches.get_flag("warn") {
        LevelFilter::Warn
    } else if matches.get_flag("error") {
        LevelFilter::Error
    } else if matches.get_flag("off") {
        LevelFilter::Off
    } else {
        LevelFilter::Info
    };

    log::set_max_level(log_level);
    info!("Log level set to {:?}", log_level);

    let current_dir = env::current_dir().expect("Failed to get the current directory");

    let env_path = current_dir.join(".env");

    debug!("Looking for .env file at: {:?}", env_path);

    match dotenv::from_path(&env_path) {
        Ok(_) => info!("Successfully loaded .env file."),
        Err(e) => error!("Failed to load .env file: {:?}", e),
    }

    let message = matches
        .get_one::<String>("message")
        .expect("Message is required");
    let channel = matches
        .get_one::<String>("channel")
        .expect("Channel is required");

    info!("Sending message to Slack channel: {}", channel);
    debug!("Message content: {}", message);

    let webhook_url = match env::var(channel) {
        Ok(url) => url,
        Err(_) => {
            error!("No webhook URL found for channel: {}", channel);
            return;
        }
    };

    debug!("Webhook URL: {}", webhook_url);

    if let Err(e) = send_slack_message(&webhook_url, message).await {
        error!("Failed to send message: {:?}", e);
    } else {
        info!("Message sent successfully!");
    }

    info!("slack_messenger has finished execution.");
}

async fn send_slack_message(webhook_url: &str, message: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let payload = serde_json::json!({
        "text": message,
    });

    let response = client.post(webhook_url).json(&payload).send().await?;

    debug!("Slack API response: {:?}", response);

    Ok(())
}
