use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;

pub async fn download_from_url(
    url: String,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::get(&url).await?;

    let total_size = match response.content_length() {
        Some(size) => size,
        None => {
            return Err("Content length not found".into());
        }
    };

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap(),
    );

    let mut file = File::create(filename)?;
    let mut downloaded = 0;

    while let Ok(chunk) = response.chunk().await {
        if chunk.is_none() {
            break;
        }
        let chunk = chunk.unwrap();
        file.write(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish();
    Ok(())
}
