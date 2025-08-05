use bskwrite::{get_or_create_agent, write_post};
use std::{env, error::Error, process};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let message = parse_args().await?;
    let agent = get_or_create_agent().await?;
    write_post(&agent, &message).await?;
    Ok(())
}

async fn parse_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("USAGE: bskwrite \"<TEXT>\"".into());
    }

    let message = &args[1];

    if message.trim().is_empty() {
        return Err("Message cannot be empty".into());
    }

    Ok(message.to_string())
}
