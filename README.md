# bskwrite

A command-line tool for posting messages to Bluesky Social written in Rust.

## Features

- Post messages to Bluesky from the command line
- Automatic session management and credential storage
- Returns clickable web URLs for your posts
- Secure credential handling with environment variables

## Prerequisites

- [Rust](https://rustup.rs/) installed on your system
- A Bluesky account

## Installation

### Option 1: Clone and Install

1. **Clone the repository**
   ```bash
   git clone https://github.com/Zike01/bskwrite.git
   cd bskwrite
   ```

2. **Set up your Bluesky credentials**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with your preferred text editor:
   ```bash
   nano .env
   ```
   
   Add your Bluesky login details:
   ```env
   BSKY_EMAIL=your-email@example.com
   BSKY_PASSWORD=your-app-password-here
   ```

3. **Build and install**
   ```bash
   cargo install --path .
   ```

### Option 2: Direct Install from GitHub

```bash
cargo install --git https://github.com/Zike01/bskwrite.git
```

Then create your `.env` file as described above.

## Getting Your Bluesky App Password

**Important**: Use an app password, not your regular Bluesky password.

1. Go to [bsky.app](https://bsky.app) and log in
2. Navigate to **Settings** → **Privacy and Security** → **App Passwords**
3. Click **Add App Password**
4. Give it a name (e.g., "bskwrite CLI")
5. Copy the generated password
6. Use this app password in your `.env` file

## Usage

Post a message to Bluesky:

```bash
bskwrite "Hello, Bluesky!"
```

The tool will:
- Automatically log you in on first use
- Save your session for future use
- Display a clickable web URL to view your post

### Examples

```bash
# Simple message
bskwrite "Just posted from the command line!"

# Message with quotes (escape them)
bskwrite "This is a \"quoted\" message"
```

## Configuration

### Environment Variables

The tool uses these environment variables from your `.env` file:

- `BSKY_EMAIL` - Your Bluesky email address
- `BSKY_PASSWORD` - Your Bluesky app password (not your regular password)

### Session Storage

After your first successful login, the tool saves your session in `config.json`. This file is automatically created and managed - you don't need to edit it manually.

### Troubleshooting

If you encounter issues:

1. Check that your `.env` file is properly configured
2. Verify your app password is valid
3. Try deleting `config.json` to force a fresh login
4. Make sure you have the latest version of Rust

## Acknowledgments

- Built with [bsky_sdk](https://docs.rs/bsky-sdk/latest/bsky_sdk/) for Bluesky API integration
- Uses [atrium-api](https://github.com/sugyan/atrium) for AT Protocol support
