use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use tokio::runtime;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
struct BikeImage {
    id: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .worker_threads(2)
        .build()?;

    runtime.block_on(async {
        let client = reqwest::Client::new();

        let response = client
            .get("https://api.example.com/bike-images") // Replace with actual API endpoint
            .header(USER_AGENT, "BikeViewer/1.0")
            .send()
            .await?;

        let bike_images: Vec<BikeImage> = response.json().await?;

        if bike_images.is_empty() {
            eprintln!("No bike images found.");
            return Ok(());
        }

        let random_bike = bike_images.choose(&mut rand::thread_rng()).unwrap();

        let image_response = client
            .get(&random_bike.url)
            .header(USER_AGENT, "BikeViewer/1.0")
            .send()
            .await?;

        let image_bytes = image_response.bytes().await?;
        let img = image::load_from_memory(&image_bytes)?;

        let img_path = Path::new("random_bike.jpg");
        let mut img_file = File::create(img_path)?;

        img.save(&mut img_file, image::ImageOutputFormat::JPEG)?;

        println!("Random bike image saved as: {:?}", img_path);
        Ok(())
    })
}
