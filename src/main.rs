use discord_image_downloader::{config, run};
use std::io;

#[tokio::main]
async fn main() {
    println!("Discord Image Downloader");
    println!("Made by Subreme :)");

    run::all(config::all().await).await;

    println!("\nMake sure to star https://github.com/subreme/discord-image-downloader if you found this useful!");
    println!("\nHit `Enter` to close!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
