use dirs;
use dotenvy;
use std::error::Error;
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = dirs::config_dir().ok_or("Could not find config directory")?;
    path.push("bskwrite");
    std::fs::create_dir_all(&path)?;

    let env_path = path.join(".env");
    if !env_path.exists() {
        create_env_example(&env_path)?;
    }

    Ok(path)
}

/// Creates new .env file inside the config directory
pub fn create_env_example(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let example_content = r#"# Bluesky credentials for bskwrite
# Get your app password from: https://bsky.app/settings/app-passwords

BSKY_EMAIL=your-email@example.com
BSKY_PASSWORD=your-app-password-here
"#;

    std::fs::write(path, example_content)?;
    println!("Created example config at: {}", path.display());
    Ok(())
}

/// Loads the email and password credentials from the .env file
pub fn load_credentials() -> Result<(String, String), Box<dyn Error>> {
    let config_dir = get_config_dir()?;
    let env_path = config_dir.join(".env");

    if env_path.exists() {
        dotenvy::from_path(env_path)?;
    } else {
        dotenvy::dotenv().ok();
    }
    let email = dotenvy::var("BSKY_EMAIL").map_err(|_| "BSKY_EMAIL not found in environment")?;
    let password =
        dotenvy::var("BSKY_PASSWORD").map_err(|_| "BSKY_PASSWORD not found in environment")?;

    Ok((email, password))
}
