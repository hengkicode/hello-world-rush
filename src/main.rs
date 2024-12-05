use reqwest::Client;
use std::fs::File;
use std::io::{Write};
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let download_url = "https://eramart.co.id/uploads/KASIR.exe";
    let file_name = "KASIR.exe";

    // Membuat client reqwest
    let client = Client::new();
    let mut response = client.get(download_url).send().await?;

    // Mendapatkan panjang file untuk progress bar
    let total_size = response.content_length().unwrap_or(0);
    let mut dest = File::create(file_name)?;

    // Setup progress bar
    let pb = if total_size > 0 {
        let bar = ProgressBar::new(total_size);
        bar.set_style(ProgressStyle::default_bar()
            .template("Downloading 🚀 [{bar:40}] {percent}%")
            .progress_chars("=>-"));
        bar
    } else {
        let bar = ProgressBar::new_spinner();
        bar.set_message("Downloading...");
        bar.enable_steady_tick(100);
        bar
    };

    // Membaca dan menulis data byte ke file secara chunk
    let mut downloaded = 0;
    while let Some(chunk) = response.chunk().await? {
        dest.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded); // Update progress bar
    }

    pb.finish_with_message("Download selesai");

    // Menutup terminal setelah selesai
    std::process::exit(0);
}
