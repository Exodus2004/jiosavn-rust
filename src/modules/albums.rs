use crate::common::helpers::{create_image_links, use_fetch};
use crate::common::types::{ApiResponse, DownloadLink};
use crate::modules::songs::{map_artist_mini, map_song, RawArtistMap, RawSong, Song, SongArtists};
use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// --- Raw API Deserialization Models ---

#[derive(Debug, Deserialize)]
pub struct RawAlbum {
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
    pub image: String,
    pub more_info: Option<RawAlbumMoreInfo>,
    pub list: Option<Vec<RawSong>>,
}

#[derive(Debug, Deserialize)]
pub struct RawAlbumMoreInfo {
    pub song_count: Option<String>,
    pub artistMap: Option<RawArtistMap>,
}

// --- Outbound Client API Models ---

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Album {
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
    pub artists: SongArtists,
    pub image: Vec<DownloadLink>,
    pub songs: Option<Vec<Song>>,
}

// --- Mapping Helpers ---

pub fn map_album(raw: RawAlbum) -> Album {
    let more_info = raw.more_info.unwrap_or_else(|| RawAlbumMoreInfo {
        song_count: None,
        artistMap: None,
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

    let songs = raw.list.map(|list| list.into_iter().map(map_song).collect());

    Album {
        id: raw.id,
        name: raw.title,
        description: raw.header_desc,
        item_type: raw.item_type,
        year: raw.year.and_then(|y| y.parse::<i32>().ok()),
        play_count: raw.play_count.and_then(|p| p.parse::<i32>().ok()),
        language: raw.language,
        explicit_content: raw.explicit_content.map(|e| e == "1").unwrap_or(false),
        url: raw.perma_url,
        song_count: more_info.song_count.and_then(|s| s.parse::<i32>().ok()),
        artists: SongArtists {
            primary,
            featured,
            all,
        },
        image: create_image_links(&raw.image),
        songs,
    }
}

// --- Axum Request / Query Structs ---

#[derive(Debug, Deserialize, IntoParams)]
pub struct AlbumQuery {
    /// The unique ID of the album
    pub id: Option<String>,
    /// A direct link to the album on JioSaavn
    pub link: Option<String>,
}

// --- API Service Methods ---

pub async fn get_album_by_id(album_id: &str) -> Result<Album, StatusCode> {
    let result = use_fetch::<RawAlbum>(
        crate::common::constants::calls::albums::ID,
        vec![("albumid".to_string(), album_id.to_string())],
        None,
    )
    .await;

    match result {
        Ok(raw_album) => {
            if raw_album.id.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(map_album(raw_album))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_album_by_link(link: &str) -> Result<Album, StatusCode> {
    let re = Regex::new(r"jiosaavn\.com/album/[^/]+/([^/]+)$").unwrap();
    let token = match re.captures(link) {
        Some(cap) => cap.get(1).map(|m| m.as_str()).unwrap_or(""),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let result = use_fetch::<RawAlbum>(
        "webapi.get",
        vec![("token".to_string(), token.to_string()), ("type".to_string(), "album".to_string())],
        None,
    )
    .await;

    match result {
        Ok(raw_album) => {
            if raw_album.id.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(map_album(raw_album))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- Axum Route Handler ---

#[utoipa::path(
    get,
    path = "/api/albums",
    params(AlbumQuery),
    responses(
        (status = 200, description = "Successful response with album details", body = ApiResponse<Album>),
        (status = 400, description = "Bad request when query parameters are missing or invalid"),
        (status = 404, description = "Album not found with the given ID or link")
    ),
    tag = "Albums"
)]
pub async fn get_album_handler(
    Query(query): Query<AlbumQuery>,
) -> impl IntoResponse {
    if query.id.is_none() && query.link.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Either album ID or link is required"
            })),
        )
            .into_response();
    }

    let result = if let Some(link) = query.link {
        get_album_by_link(&link).await
    } else {
        get_album_by_id(&query.id.unwrap()).await
    };

    match result {
        Ok(album) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: album,
            }),
        )
            .into_response(),
        Err(status) => {
            let message = match status {
                StatusCode::BAD_REQUEST => "Invalid link formatting",
                StatusCode::NOT_FOUND => "Album not found",
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
