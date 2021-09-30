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

use serde::Deserialize;

use super::Video;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub author_banners: Vec<Banner>,
    pub author_thumbnails: Vec<Banner>,
    pub sub_count: i32,
    pub total_views: i64,
    pub joined: i64,
    #[serde(default)]
    pub paid: bool,
    pub auto_generated: bool,
    pub is_family_friendly: bool,
    pub description: String,
    pub description_html: String,
    pub allowed_regions: Vec<String>,
    pub latest_videos: Vec<Video>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub url: String,
    pub width: i32,
    pub height: i32,
}
