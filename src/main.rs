mod file_ops;
mod icon;
mod utils;

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url =
        Url::parse("https://fonts.google.com/metadata/icons?key=material_symbols&incomplete=true")
            .unwrap();
    let client = Client::new();

    let response = client.get(url).send().await?;
    if response.status().is_success() {
        let txt = response.text().await?.replace(")]}'", "");
        let metadata = icon::parse_metadata(&txt).expect("Cannot parse metadata");

        let current_directory = std::env::current_dir().unwrap();
        let icon_directory = current_directory.join("material-icons");

        if !Path::new(&icon_directory).exists() {
            fs::create_dir_all(&icon_directory).await.unwrap();
        }

        let num_icons = metadata.icons.len();
        let progress_bar = ProgressBar::new(num_icons as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}/{eta}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .progress_chars("#>-")
        );

        futures::stream::iter(metadata.icons.clone())
            .map(|icon| {
                let client = client.clone();
                let metadata = metadata.clone();
                let progress_bar = progress_bar.clone();
                tokio::spawn(async move {
                    icon::process_icon(&client, &metadata, icon).await.unwrap();
                    progress_bar.inc(1); // Update progress
                })
            })
            .buffer_unordered(5)
            .for_each(|_| async {})
            .await;

        progress_bar.finish_with_message("Processing complete.");
    } else {
        eprintln!("Error fetching JSON: {}", response.status());
    }

    Ok(())
}
