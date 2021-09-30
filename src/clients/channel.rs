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

use std::error::Error;

use crate::{
    structure::{Channel, Video},
    Invidious,
};

const ENDPOINT_CHANNEL: &str = "channels";
const ENDPOINT_VIDEO: &str = "videos";

#[derive(Clone)]
pub struct ChannelClient {
    invidious: Invidious,
    channel: Channel,
}

impl ChannelClient {
    pub(crate) async fn new<I: AsRef<str>>(
        invidious: &Invidious,
        id: I,
    ) -> Result<ChannelClient, Box<dyn Error>> {
        let response = invidious
            .client()
            .get(format!(
                "{}/{}/{}",
                invidious.api_url(),
                ENDPOINT_CHANNEL,
                id.as_ref()
            ))
            .send()
            .await?;

        let text = response.text().await?;

        Ok(Self {
            invidious: invidious.clone(),
            channel: serde_json::from_str(&text)?,
        })
    }

    pub fn channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn videos(&self) -> impl futures::stream::Stream<Item = Video> {
        let mut stream_info = StreamInfo {
            channel_client: self.clone(),
            video_buffer: self.channel.latest_videos.clone(),
            current_page: 1,
        };

        async_stream::stream! {
            while !stream_info.video_buffer.is_empty() {
                let video = stream_info.video_buffer.remove(0);
                yield video;

                if stream_info.video_buffer.is_empty() {
                    stream_info.current_page = stream_info.current_page + 1;
                    let new_videos = stream_info.channel_client.videos_from_page(stream_info.current_page).await;
                    stream_info.video_buffer = new_videos.unwrap_or(vec![]);
                }
            }
        }
    }

    pub(crate) async fn videos_from_page(&self, page: usize) -> Result<Vec<Video>, Box<dyn Error>> {
        let response = self
            .invidious
            .client()
            .get(format!(
                "{}/{}/{}/{}?page={}",
                self.invidious.api_url(),
                ENDPOINT_CHANNEL,
                ENDPOINT_VIDEO,
                self.channel.author_id,
                page
            ))
            .send()
            .await?;

        let text = response.text().await?;
        let videos: Vec<Video> = serde_json::from_str(&text)?;
        Ok(videos)
    }
}

struct StreamInfo {
    channel_client: ChannelClient,
    video_buffer: Vec<Video>,
    current_page: usize,
}
