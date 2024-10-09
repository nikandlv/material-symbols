use crate::file_ops::{run_prettier_on_directory, save_svg_and_tsx};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Icon {
    pub name: String,
    pub version: u32,
    pub popularity: u32,
    pub codepoint: u32,
    pub unsupported_families: Vec<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub sizes_px: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metadata {
    pub host: String,
    pub asset_url_pattern: String,
    pub families: Vec<String>,
    pub icons: Vec<Icon>,
}

pub async fn process_icon(
    client: &Client,
    meta: &Metadata,
    icon: Icon,
) -> Result<(), reqwest::Error> {
    let theme_map = vec![
        ("baseline", ""),
        ("outline", "_outlined"),
        ("round", "_round"),
        ("twotone", "_two_tone"),
        ("sharp", "_sharp"),
    ];
    let theme_name_map = vec![
        ("baseline", "Material Icons"),
        ("outline", "Material Icons Outlined"),
        ("round", "Material Icons Round"),
        ("twotone", "Material Icons Two Tone"),
        ("sharp", "Material Icons Sharp"),
    ];

    let all_families_unsupported = theme_name_map
        .iter()
        .all(|(_, family)| icon.unsupported_families.contains(&family.to_string()));
    if all_families_unsupported {
        return Ok(());
    }

    let current_directory = std::env::current_dir().unwrap();
    let base_icon_directory = current_directory.join("material-icons");
    let icon_directory = base_icon_directory.join(&icon.name);
    let svg_directory = icon_directory.join("svg");
    let tsx_directory = icon_directory.join("tsx");

    fs::create_dir_all(&svg_directory).await.unwrap();
    fs::create_dir_all(&tsx_directory).await.unwrap();

    for ((theme, suffix), (_, family_name)) in theme_map.iter().zip(theme_name_map.iter()) {
        if icon.unsupported_families.contains(&family_name.to_string()) {
            continue;
        }

        let theme_suffix = suffix.replace("_", "");
        let icon_url = format!(
            "https://{}/s/i/materialicons{}/{}/v{}/24px.svg",
            meta.host, theme_suffix, icon.name, icon.version
        );

        let response = client.get(&icon_url).send().await?;

        if response.status().is_success() {
            let svg_content = response.text().await.unwrap();
            save_svg_and_tsx(
                &svg_directory,
                &tsx_directory,
                &icon.name,
                theme,
                &svg_content,
            )
            .await;
        }
    }

    run_prettier_on_directory(&tsx_directory);
    Ok(())
}

pub fn parse_metadata(txt: &str) -> Result<Metadata, serde_json::Error> {
    serde_json::from_str(txt)
}
