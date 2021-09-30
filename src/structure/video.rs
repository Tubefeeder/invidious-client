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
 * along with Invidious-client.  If not, see <https://www.gnu.org/licenses/>.
 */

use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub video_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_thumbnails: Vec<Thumbnail>,
    pub description: String,
    pub description_html: String,
    pub view_count: i64,
    pub published: i64,
    pub published_text: String,
    pub length_seconds: i32,
    #[serde(default)]
    pub paid: bool,
    pub premium: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub quality: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}
