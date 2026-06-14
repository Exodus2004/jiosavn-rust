use crate::common::helpers::{create_download_links, create_image_links, use_fetch};
use crate::common::types::{ApiResponse, DownloadLink};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

// --- Raw API Deserialization Models ---

#[derive(Debug, Deserialize)]
pub struct RawSong {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub perma_url: String,
    pub image: String,
    pub language: String,
    pub year: Option<String>,
    pub play_count: Option<String>,
    pub explicit_content: Option<String>,
    pub more_info: Option<RawSongMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSongMoreInfo {
    pub album_id: Option<String>,
    pub album: Option<String>,
    pub album_url: Option<String>,
    pub duration: Option<String>,
    pub label: Option<String>,
    pub encrypted_media_url: Option<String>,
    pub has_lyrics: Option<String>,
    pub lyrics_id: Option<String>,
    pub copyright_text: Option<String>,
    pub artistMap: Option<RawArtistMap>,
    pub release_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawArtistMap {
    pub primary_artists: Option<Vec<RawArtistMini>>,
    pub featured_artists: Option<Vec<RawArtistMini>>,
    pub artists: Option<Vec<RawArtistMini>>,
}

#[derive(Debug, Deserialize)]
pub struct RawArtistMini {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub image: String,
    pub perma_url: String,
}

#[derive(Debug, Deserialize)]
pub struct RawSongsWrapper {
    pub songs: Option<Vec<RawSong>>,
}

#[derive(Debug, Deserialize)]
pub struct RawStationResponse {
    pub stationid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSuggestionElement {
    pub song: RawSong,
}

// --- Outbound Client API Models ---

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ArtistMini {
    pub id: String,
    pub name: String,
    pub role: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub image: Vec<DownloadLink>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SongAlbum {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SongArtists {
    pub primary: Vec<ArtistMini>,
    pub featured: Vec<ArtistMini>,
    pub all: Vec<ArtistMini>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Song {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub year: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub duration: Option<i32>,
    pub label: Option<String>,
    #[serde(rename = "explicitContent")]
    pub explicit_content: bool,
    #[serde(rename = "playCount")]
    pub play_count: Option<i32>,
    pub language: String,
    #[serde(rename = "hasLyrics")]
    pub has_lyrics: bool,
    #[serde(rename = "lyricsId")]
    pub lyrics_id: Option<String>,
    pub url: String,
    pub copyright: Option<String>,
    pub album: SongAlbum,
    pub artists: SongArtists,
    pub image: Vec<DownloadLink>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Vec<DownloadLink>,
}

// --- Mapping Helper Functions ---

pub fn map_artist_mini(raw: RawArtistMini) -> ArtistMini {
    ArtistMini {
        id: raw.id,
        name: raw.name,
        role: raw.role,
        item_type: raw.item_type,
        image: create_image_links(&raw.image),
        url: raw.perma_url,
    }
}

pub fn map_song(raw: RawSong) -> Song {
    let more_info = raw.more_info.unwrap_or_else(|| RawSongMoreInfo {
        album_id: None,
        album: None,
        album_url: None,
        duration: None,
        label: None,
        encrypted_media_url: None,
        has_lyrics: None,
        lyrics_id: None,
        copyright_text: None,
        artistMap: None,
        release_date: None,
    });

    let artist_map = more_info.artistMap.unwrap_or_else(|| RawArtistMap {
        primary_artists: None,
        featured_artists: None,
        artists: None,
    });

    let primary = artist_map
        .primary_artists
        .unwrap_or_default()
        .into_iter()
        .map(map_artist_mini)
        .collect();

    let featured = artist_map
        .featured_artists
        .unwrap_or_default()
        .into_iter()
        .map(map_artist_mini)
        .collect();

    let all = artist_map
        .artists
        .unwrap_or_default()
        .into_iter()
        .map(map_artist_mini)
        .collect();

    Song {
        id: raw.id,
        name: raw.title,
        item_type: raw.item_type,
        year: raw.year,
        release_date: more_info.release_date,
        duration: more_info.duration.and_then(|d| d.parse::<i32>().ok()),
        label: more_info.label,
        explicit_content: raw.explicit_content.map(|e| e == "1").unwrap_or(false),
        play_count: raw.play_count.and_then(|p| p.parse::<i32>().ok()),
        language: raw.language,
        has_lyrics: more_info.has_lyrics.map(|hl| hl == "true").unwrap_or(false),
        lyrics_id: more_info.lyrics_id,
        url: raw.perma_url,
        copyright: more_info.copyright_text,
        album: SongAlbum {
            id: more_info.album_id,
            name: more_info.album,
            url: more_info.album_url,
        },
        artists: SongArtists {
            primary,
            featured,
            all,
        },
        image: create_image_links(&raw.image),
        download_url: create_download_links(&more_info.encrypted_media_url.unwrap_or_default()),
    }
}

// --- Axum Request / Query Structs ---

#[derive(Debug, Deserialize, IntoParams)]
pub struct SongsQuery {
    /// Comma-separated list of song IDs
    pub ids: Option<String>,
    /// A direct link to the song on JioSaavn
    pub link: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SuggestionsQuery {
    /// Limit the number of suggestions to retrieve
    pub limit: Option<usize>,
}

// --- API Service Methods ---

pub async fn get_songs_by_ids(ids: &str) -> Result<Vec<Song>, StatusCode> {
    let result = use_fetch::<RawSongsWrapper>(
        crate::common::constants::calls::songs::ID,
        vec![("pids".to_string(), ids.to_string())],
        None,
    )
    .await;

    match result {
        Ok(wrapper) => {
            if let Some(songs) = wrapper.songs {
                if songs.is_empty() {
                    return Err(StatusCode::NOT_FOUND);
                }
                Ok(songs.into_iter().map(map_song).collect())
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_songs_by_link(link: &str) -> Result<Vec<Song>, StatusCode> {
    // Extract token
    let re = Regex::new(r"jiosaavn\.com/song/[^/]+/([^/]+)$").unwrap();
    let token = match re.captures(link) {
        Some(cap) => cap.get(1).map(|m| m.as_str()).unwrap_or(""),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let result = use_fetch::<RawSongsWrapper>(
        "webapi.get",
        vec![("token".to_string(), token.to_string()), ("type".to_string(), "song".to_string())],
        None,
    )
    .await;

    match result {
        Ok(wrapper) => {
            if let Some(songs) = wrapper.songs {
                if songs.is_empty() {
                    return Err(StatusCode::NOT_FOUND);
                }
                Ok(songs.into_iter().map(map_song).collect())
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_song_station(song_id: &str) -> Result<String, StatusCode> {
    // entity_id is JSON encoded array of urlencoded songId
    let encoded_song_id = urlencoding::encode(song_id).into_owned();
    let entity_id = serde_json::to_string(&vec![encoded_song_id]).unwrap_or_default();

    let result = use_fetch::<RawStationResponse>(
        crate::common::constants::calls::songs::STATION,
        vec![
            ("entity_id".to_string(), entity_id),
            ("entity_type".to_string(), "queue".to_string()),
        ],
        Some("android"),
    )
    .await;

    match result {
        Ok(res) => {
            if let Some(station_id) = res.stationid {
                Ok(station_id)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_song_suggestions(song_id: &str, limit: usize) -> Result<Vec<Song>, StatusCode> {
    let station_id = create_song_station(song_id).await?;

    let result = use_fetch::<serde_json::Value>(
        crate::common::constants::calls::songs::SUGGESTIONS,
        vec![
            ("stationid".to_string(), station_id),
            ("k".to_string(), limit.to_string()),
        ],
        Some("android"),
    )
    .await;

    match result {
        Ok(value) => {
            let mut suggestions_with_keys = Vec::new();
            if let serde_json::Value::Object(map) = value {
                for (key, val) in map {
                    if key != "stationid" {
                        if let Ok(elem) = serde_json::from_value::<RawSuggestionElement>(val) {
                            if let Ok(idx) = key.parse::<usize>() {
                                suggestions_with_keys.push((idx, map_song(elem.song)));
                            } else {
                                suggestions_with_keys.push((usize::MAX, map_song(elem.song)));
                            }
                        }
                    }
                }
            }
            // Sort numerically to preserve original index order
            suggestions_with_keys.sort_by_key(|(idx, _)| *idx);
            let mut suggestions: Vec<Song> = suggestions_with_keys.into_iter().map(|(_, song)| song).collect();
            suggestions.truncate(limit);
            Ok(suggestions)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- Axum Route Handlers ---

#[utoipa::path(
    get,
    path = "/api/songs",
    params(SongsQuery),
    responses(
        (status = 200, description = "Successful response with song details", body = ApiResponse<Vec<Song>>),
        (status = 400, description = "Bad request when query parameters are missing or invalid"),
        (status = 404, description = "Song not found with the given ID or link")
    ),
    tag = "Songs"
)]
pub async fn get_songs_handler(
    Query(query): Query<SongsQuery>,
) -> impl IntoResponse {
    if query.ids.is_none() && query.link.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Either song IDs or link is required"
            })),
        )
            .into_response();
    }

    let result = if let Some(link) = query.link {
        get_songs_by_link(&link).await
    } else {
        get_songs_by_ids(&query.ids.unwrap()).await
    };

    match result {
        Ok(songs) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: songs,
            }),
        )
            .into_response(),
        Err(status) => {
            let message = match status {
                StatusCode::BAD_REQUEST => "Invalid link formatting",
                StatusCode::NOT_FOUND => "Song not found",
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

#[utoipa::path(
    get,
    path = "/api/songs/{id}",
    params(
        ("id" = String, Path, description = "ID of the song to retrieve")
    ),
    responses(
        (status = 200, description = "Successful response with song details", body = ApiResponse<Vec<Song>>),
        (status = 404, description = "Song not found for the given ID")
    ),
    tag = "Songs"
)]
pub async fn get_song_by_id_handler(
    Path(id): Path<String>,
) -> impl IntoResponse {
    match get_songs_by_ids(&id).await {
        Ok(songs) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: songs,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Song not found"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/songs/{id}/suggestions",
    params(
        ("id" = String, Path, description = "ID of the song to retrieve suggestions for"),
        SuggestionsQuery
    ),
    responses(
        (status = 200, description = "Successful response with song suggestions", body = ApiResponse<Vec<Song>>),
        (status = 404, description = "No suggestions found")
    ),
    tag = "Songs"
)]
pub async fn get_song_suggestions_handler(
    Path(id): Path<String>,
    Query(query): Query<SuggestionsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    match get_song_suggestions(&id, limit).await {
        Ok(songs) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: songs,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "No suggestions found"
            })),
        )
            .into_response(),
    }
}
