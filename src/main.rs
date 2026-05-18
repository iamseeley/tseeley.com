use serde::Deserialize;
use std::fs;

mod build;
mod feeds;
mod layout;
#[cfg(feature = "serve")]
mod serve;
mod sitemap;
mod theme;

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub author: String,
    pub base_url: String,
    pub language: String,
    pub theme_color: String,
    pub og_image: String,
    pub profile_image: String,
    pub twitter_handle: String,
    pub bookmarks_url: String,
    pub generator: String,
    #[serde(default = "default_dev_addr")]
    #[allow(dead_code)] // only read when the `serve` feature is enabled
    pub dev_addr: String,
    #[serde(default)]
    pub show_drafts: bool,
    pub rel_me: Vec<String>,
    #[serde(default)]
    pub post_socials: Vec<SocialLink>,
    pub license: License,
    #[serde(default)]
    pub analytics: Option<Analytics>,
}

#[derive(Deserialize)]
pub struct SocialLink {
    pub label: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Analytics {
    pub umami_url: String,
    pub umami_id: String,
}

fn default_dev_addr() -> String {
    "127.0.0.1:1111".into()
}

fn main() {
    let config: Config = match fs::read_to_string("config.toml")
        .map_err(|e| e.to_string())
        .and_then(|s| toml::from_str(&s).map_err(|e| e.to_string()))
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("config.toml: {e}");
            std::process::exit(1);
        }
    };

    let cmd = std::env::args().nth(1).unwrap_or_else(|| "build".into());
    let result: Result<(), Box<dyn std::error::Error>> = match cmd.as_str() {
        "build" => build::run(&config),
        "serve" => serve_cmd(config),
        other => {
            eprintln!("unknown command: {other}");
            std::process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

#[cfg(feature = "serve")]
fn serve_cmd(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    serve::run(config)
}

#[cfg(not(feature = "serve"))]
fn serve_cmd(_config: Config) -> Result<(), Box<dyn std::error::Error>> {
    Err("serve was disabled at compile time — rebuild with --features serve".into())
}
