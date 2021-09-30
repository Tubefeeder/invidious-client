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

pub mod clients;
pub mod structure;

use std::error::Error;

use clients::ChannelClient;
use reqwest::Client;

#[derive(Clone)]
pub struct Invidious {
    base_url: String,
    client: Client,
}

impl Invidious {
    pub fn new<S: AsRef<str>>(base_url: S, client: Client) -> Self {
        // Remove trailing /
        let mut url = base_url.as_ref().to_owned();
        if url.ends_with('/') {
            url.pop();
        }

        Self {
            base_url: url,
            client,
        }
    }

    pub(crate) fn api_url(&self) -> String {
        format!("{}/api/v1", self.base_url)
    }

    pub(crate) fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn channel_client<I: AsRef<str>>(
        &self,
        id: I,
    ) -> Result<ChannelClient, Box<dyn Error>> {
        ChannelClient::new(&self, id).await
    }
}
