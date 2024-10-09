use crate::utils::{capitalize_first_letter, generate_tsx};
use std::path::Path;
use std::process::Command;
use tokio::fs;

pub async fn save_svg_and_tsx(
    svg_dir: &Path,
    tsx_dir: &Path,
    icon_name: &str,
    theme: &str,
    svg_content: &str,
) {
    let svg_file_name = format!("{}.svg", theme);
    let svg_file_path = svg_dir.join(&svg_file_name);
    fs::write(svg_file_path, svg_content).await.unwrap();

    let tsx_content = generate_tsx(icon_name, theme, svg_content);
    let tsx_file_name = format!("{}.tsx", capitalize_first_letter(theme));
    let tsx_file_path = tsx_dir.join(&tsx_file_name);
    fs::write(tsx_file_path, tsx_content).await.unwrap();
}

pub fn run_prettier_on_directory(directory: &Path) {
    let relative_path = directory.to_str().unwrap();
    let output = Command::new("npx")
        .arg("prettier")
        .arg(relative_path)
        .arg("--write")
        .arg("--no-error-on-unmatched-pattern")
        .output()
        .expect("Failed to run Prettier");

    if !output.status.success() {
        println!(
            "Prettier failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
