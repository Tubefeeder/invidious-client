/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of invidious-client.
 *
 * Invidious-client is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Invidious-client is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with invidious-client.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate invidious_client;

use futures::StreamExt;
use invidious_client::Invidious;
use std::error::Error;

const INVIDIOUS_URL: &str = "https://y.com.cm/";
const CHANNEL_ID: &str = "UCSMOQeBJ2RAnuFungnQOxLg";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let invidious = Invidious::new(INVIDIOUS_URL, reqwest::Client::new());
    let channel_extractor = invidious.channel_client(CHANNEL_ID).await?;

    let mut videos_stream = Box::pin(channel_extractor.videos()).take(100);

    while let Some(video) = videos_stream.next().await {
        println!("Video {}", video.title);
    }

    Ok(())
}
