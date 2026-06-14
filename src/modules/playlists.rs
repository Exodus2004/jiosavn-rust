use crate::common::helpers::{create_image_links, use_fetch};
use crate::common::types::{ApiResponse, DownloadLink};
use crate::modules::songs::{map_artist_mini, ArtistMini, RawArtistMini};
use crate::modules::songs::{map_song, RawSong, Song};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// --- Raw API Deserialization Models ---

#[derive(Debug, Deserialize)]
pub struct RawPlaylist {
    pub id: String,
    pub title: String,
    pub header_desc: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub year: Option<String>,
    pub play_count: Option<String>,
    pub language: String,
    pub explicit_content: Option<String>,
    pub perma_url: String,
    pub list_count: Option<String>,
    pub image: String,
    pub more_info: Option<RawPlaylistMoreInfo>,
    pub list: Option<Vec<RawSong>>,
}

#[derive(Debug, Deserialize)]
pub struct RawPlaylistMoreInfo {
    pub artists: Option<Vec<RawArtistMini>>,
}

// --- Outbound Client API Models ---

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub year: Option<i32>,
    #[serde(rename = "playCount")]
    pub play_count: Option<i32>,
    pub language: String,
    #[serde(rename = "explicitContent")]
    pub explicit_content: bool,
    pub url: String,
    #[serde(rename = "songCount")]
    pub song_count: Option<i32>,
    pub artists: Option<Vec<ArtistMini>>,
    pub image: Vec<DownloadLink>,
    pub songs: Option<Vec<Song>>,
}

// --- Mapping Helpers ---

pub fn map_playlist(raw: RawPlaylist) -> Playlist {
    let more_info = raw.more_info.unwrap_or_else(|| RawPlaylistMoreInfo {
        artists: None,
    });

    let artists = more_info.artists.map(|list| {
        list.into_iter().map(map_artist_mini).collect()
    });

    let songs = raw.list.map(|list| list.into_iter().map(map_song).collect());

    Playlist {
        id: raw.id,
        name: raw.title,
        description: raw.header_desc,
        item_type: raw.item_type,
        year: raw.year.and_then(|y| y.parse::<i32>().ok()),
        play_count: raw.play_count.and_then(|p| p.parse::<i32>().ok()),
        language: raw.language,
        explicit_content: raw.explicit_content.map(|e| e == "1").unwrap_or(false),
        url: raw.perma_url,
        song_count: raw.list_count.and_then(|s| s.parse::<i32>().ok()),
        artists,
        image: create_image_links(&raw.image),
        songs,
    }
}

// --- Axum Request / Query Structs ---

#[derive(Debug, Deserialize, IntoParams)]
pub struct PlaylistQuery {
    /// The unique ID of the playlist
    pub id: Option<String>,
    /// A direct link to the playlist on JioSaavn
    pub link: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<usize>,
}

// --- API Service Methods ---

pub async fn get_playlist_by_id(playlist_id: &str, page: i32, limit: usize) -> Result<Playlist, StatusCode> {
    let result = use_fetch::<RawPlaylist>(
        crate::common::constants::calls::playlists::ID,
        vec![
            ("listid".to_string(), playlist_id.to_string()),
            ("n".to_string(), limit.to_string()),
            ("p".to_string(), page.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw_playlist) => {
            if raw_playlist.id.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            let mut playlist = map_playlist(raw_playlist);
            if let Some(ref mut songs) = playlist.songs {
                playlist.song_count = Some(songs.len() as i32);
                songs.truncate(limit);
            }
            Ok(playlist)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_playlist_by_link(link: &str, page: i32, limit: usize) -> Result<Playlist, StatusCode> {
    // Extract token
    // Regex matches featured playlists or s/playlist routes
    let re = Regex::new(r"(?:jiosaavn\.com|saavn\.com)/(?:featured|s/playlist)/[^/]+/([^/]+)$|/([^/]+)$").unwrap();
    let token = match re.captures(link) {
        Some(cap) => {
            // Find first non-empty capture group (excluding full match)
            cap.iter()
                .skip(1)
                .flatten()
                .map(|m| m.as_str())
                .last()
                .unwrap_or("")
        }
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let result = use_fetch::<RawPlaylist>(
        "webapi.get",
        vec![
            ("token".to_string(), token.to_string()),
            ("type".to_string(), "playlist".to_string()),
            ("n".to_string(), limit.to_string()),
            ("p".to_string(), page.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw_playlist) => {
            if raw_playlist.id.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            let mut playlist = map_playlist(raw_playlist);
            if let Some(ref mut songs) = playlist.songs {
                playlist.song_count = Some(songs.len() as i32);
                songs.truncate(limit);
            }
            Ok(playlist)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- Axum Route Handler ---

#[utoipa::path(
    get,
    path = "/api/playlists",
    params(PlaylistQuery),
    responses(
        (status = 200, description = "Successful response with playlist details", body = ApiResponse<Playlist>),
        (status = 400, description = "Bad request when query parameters are missing or invalid"),
        (status = 404, description = "Playlist not found with the given ID or link")
    ),
    tag = "Playlists"
)]
pub async fn get_playlist_handler(
    Query(query): Query<PlaylistQuery>,
) -> impl IntoResponse {
    if query.id.is_none() && query.link.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Either playlist ID or link is required"
            })),
        )
            .into_response();
    }

    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    let result = if let Some(link) = query.link {
        get_playlist_by_link(&link, page, limit).await
    } else {
        get_playlist_by_id(&query.id.unwrap(), page, limit).await
    };

    match result {
        Ok(playlist) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: playlist,
            }),
        )
            .into_response(),
        Err(status) => {
            let message = match status {
                StatusCode::BAD_REQUEST => "Invalid link formatting",
                StatusCode::NOT_FOUND => "Playlist not found",
                _ => "Internal server error",
            };
            (
                status,
                Json(serde_json::json!({
                    "success": false,
                    "message": message
                })),
            )
                .into_response()
        }
    }
}
