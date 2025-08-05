use atrium_api::app::bsky::feed::post::RecordData;
use atrium_api::types::string::Datetime;
use bsky_sdk::{
    BskyAgent,
    agent::config::{Config, FileStore},
};
use dotenvy;
use std::error::Error;

const CONFIG_FILE: &str = "config.json";

/// Write a post to Bluesky using the agent and message
pub async fn write_post(agent: &BskyAgent, message: &str) -> bsky_sdk::Result<()> {
    let session = agent.get_session().await;
    let handle = &session.unwrap().handle.to_string();
    let result = agent
        .create_record(RecordData {
            created_at: Datetime::now(),
            embed: None,
            entities: None,
            facets: None,
            labels: None,
            langs: None,
            reply: None,
            tags: None,
            text: message.to_string(),
        })
        .await?;
    if let Some(id) = extract_post_id(&result.uri) {
        let post_url = format!("https://bsky.app/profile/{}/post/{}", handle, id);
        println!("Posted, view at: {}", post_url);
    } else {
        println!("Posted, view at: {}", result.uri);
    }
    Ok(())
}

/// Saves agent config and session to the config file
pub async fn save_user_agent() -> Result<BskyAgent, Box<dyn Error>> {
    dotenvy::dotenv().map_err(|e| format!("Failed to load .env file: {}", e))?;

    let email = dotenvy::var("BSKY_EMAIL").map_err(|_| "BSKY_EMAIL not found in environment")?;
    let password =
        dotenvy::var("BSKY_PASSWORD").map_err(|_| "BSKY_PASSWORD not found in environment")?;

    let agent = BskyAgent::builder().build().await?;
    agent.login(&email, &password).await?;
    agent
        .to_config()
        .await
        .save(&FileStore::new(CONFIG_FILE))
        .await?;
    Ok(agent)
}

/// Loads agent config and session if config file exists
pub async fn load_user_agent() -> Result<BskyAgent, Box<dyn Error>> {
    let agent = BskyAgent::builder()
        .config(Config::load(&FileStore::new(CONFIG_FILE)).await?)
        .build()
        .await?;

    let result = agent.api.com.atproto.server.get_session().await;
    assert!(result.is_ok());
    Ok(agent)
}

pub async fn get_or_create_agent() -> Result<BskyAgent, Box<dyn Error>> {
    match load_user_agent().await {
        Ok(a) => Ok(a),
        Err(_) => save_user_agent().await,
    }
}

/// Converts the uri from created record data into a valid url
fn extract_post_id(post: &str) -> Option<String> {
    let parts: Vec<&str> = post.strip_prefix("at://")?.split("/").collect();

    if parts.len() == 3 && parts[1] == "app.bsky.feed.post" {
        Some(parts[2].to_string())
    } else {
        None
    }
}
