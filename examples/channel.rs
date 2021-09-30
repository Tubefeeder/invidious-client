extern crate invidious;

use invidious::Invidious;
use std::error::Error;

const INVIDIOUS_URL: &str = "https://y.com.cm/";
const CHANNEL_ID: &str = "UCpCSAcbqs-sjEVfk_hMfY9w";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let invidious = Invidious::new(INVIDIOUS_URL, reqwest::Client::new());
    let channel_extractor = invidious.channel_client(CHANNEL_ID).await?;

    println!("Channel name: {}", channel_extractor.channel().author);

    Ok(())
}
