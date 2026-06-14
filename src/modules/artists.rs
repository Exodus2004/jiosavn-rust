use crate::common::helpers::{create_image_links, use_fetch};
use crate::common::types::{ApiResponse, DownloadLink};
use crate::modules::albums::{map_album, Album, RawAlbum};
use crate::modules::songs::{map_song, RawSong, Song};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// --- Raw API Deserialization Models ---

#[derive(Debug, Deserialize)]
pub struct RawArtist {
    pub artistId: Option<String>,
    pub id: Option<String>,
    pub name: String,
    pub urls: Option<RawArtistUrls>,
    pub perma_url: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub follower_count: Option<String>,
    pub fan_count: Option<String>,
    pub isVerified: Option<bool>,
    pub dominantLanguage: Option<String>,
    pub dominantType: Option<String>,
    pub bio: Option<String>,
    pub dob: Option<String>,
    pub fb: Option<String>,
    pub twitter: Option<String>,
    pub wiki: Option<String>,
    pub availableLanguages: Option<Vec<String>>,
    pub isRadioPresent: Option<bool>,
    pub image: String,
    pub topSongs: Option<Vec<RawSong>>,
    pub topAlbums: Option<Vec<RawAlbum>>,
    pub singles: Option<Vec<RawSong>>,
    pub similarArtists: Option<Vec<RawSimilarArtist>>,
}

#[derive(Debug, Deserialize)]
pub struct RawArtistUrls {
    pub overview: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSimilarArtist {
    pub id: String,
    pub name: String,
    pub perma_url: String,
    pub image_url: String,
    pub languages: Option<String>,
    pub wiki: Option<String>,
    pub dob: Option<String>,
    pub fb: Option<String>,
    pub twitter: Option<String>,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub dominantType: Option<String>,
    pub aka: Option<String>,
    pub bio: Option<String>,
    pub similar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawArtistSongsWrapper {
    #[serde(rename = "topSongs")]
    pub top_songs: RawTopSongs,
}

#[derive(Debug, Deserialize)]
pub struct RawTopSongs {
    pub total: i32,
    pub songs: Vec<RawSong>,
}

#[derive(Debug, Deserialize)]
pub struct RawArtistAlbumsWrapper {
    #[serde(rename = "topAlbums")]
    pub top_albums: RawTopAlbums,
}

#[derive(Debug, Deserialize)]
pub struct RawTopAlbums {
    pub total: i32,
    pub albums: Vec<RawAlbum>,
}

// --- Outbound Client API Models ---

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SimilarArtist {
    pub id: String,
    pub name: String,
    pub url: String,
    pub image: Vec<DownloadLink>,
    pub languages: Option<serde_json::Value>,
    pub wiki: Option<String>,
    pub dob: Option<String>,
    pub fb: Option<String>,
    pub twitter: Option<String>,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(rename = "dominantType")]
    pub dominant_type: Option<String>,
    pub aka: Option<String>,
    pub bio: Option<serde_json::Value>,
    #[serde(rename = "similarArtists")]
    pub similar_artists: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(rename = "followerCount")]
    pub follower_count: Option<i32>,
    #[serde(rename = "fanCount")]
    pub fan_count: Option<String>,
    #[serde(rename = "isVerified")]
    pub is_verified: Option<bool>,
    #[serde(rename = "dominantLanguage")]
    pub dominant_language: Option<String>,
    #[serde(rename = "dominantType")]
    pub dominant_type: Option<String>,
    pub bio: Option<serde_json::Value>,
    pub dob: Option<String>,
    pub fb: Option<String>,
    pub twitter: Option<String>,
    pub wiki: Option<String>,
    #[serde(rename = "availableLanguages")]
    pub available_languages: Option<Vec<String>>,
    #[serde(rename = "isRadioPresent")]
    pub is_radio_present: Option<bool>,
    pub image: Vec<DownloadLink>,
    #[serde(rename = "topSongs")]
    pub top_songs: Option<Vec<Song>>,
    #[serde(rename = "topAlbums")]
    pub top_albums: Option<Vec<Album>>,
    pub singles: Option<Vec<Song>>,
    #[serde(rename = "similarArtists")]
    pub similar_artists: Option<Vec<SimilarArtist>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ArtistSongs {
    pub total: i32,
    pub songs: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ArtistAlbums {
    pub total: i32,
    pub albums: Vec<Album>,
}

// --- Mapping Helpers ---

pub fn map_similar_artist(raw: RawSimilarArtist) -> SimilarArtist {
    SimilarArtist {
        id: raw.id,
        name: raw.name,
        url: raw.perma_url,
        image: create_image_links(&raw.image_url),
        languages: raw.languages.and_then(|l| serde_json::from_str(&l).ok()),
        wiki: raw.wiki,
        dob: raw.dob,
        fb: raw.fb,
        twitter: raw.twitter,
        is_radio_present: raw.is_radio_present,
        item_type: raw.item_type,
        dominant_type: raw.dominantType,
        aka: raw.aka,
        bio: raw.bio.and_then(|b| serde_json::from_str(&b).ok()),
        similar_artists: raw.similar.and_then(|s| serde_json::from_str(&s).ok()),
    }
}

pub fn map_artist(raw: RawArtist) -> Artist {
    let id = raw.artistId.or(raw.id).unwrap_or_default();
    let url = raw.urls.and_then(|u| u.overview).or(raw.perma_url).unwrap_or_default();
    let follower_count = raw.follower_count.and_then(|f| f.parse::<i32>().ok());
    let bio = raw.bio.and_then(|b| serde_json::from_str(&b).ok());

    let top_songs = raw.topSongs.map(|songs| songs.into_iter().map(map_song).collect());
    let top_albums = raw.topAlbums.map(|albums| albums.into_iter().map(map_album).collect());
    let singles = raw.singles.map(|songs| songs.into_iter().map(map_song).collect());
    let similar_artists = raw.similarArtists.map(|similar| {
        similar.into_iter().map(map_similar_artist).collect()
    });

    Artist {
        id,
        name: raw.name,
        url,
        item_type: raw.item_type,
        follower_count,
        fan_count: raw.fan_count,
        is_verified: raw.isVerified,
        dominant_language: raw.dominantLanguage,
        dominant_type: raw.dominantType,
        bio,
        dob: raw.dob,
        fb: raw.fb,
        twitter: raw.twitter,
        wiki: raw.wiki,
        available_languages: raw.availableLanguages,
        is_radio_present: raw.isRadioPresent,
        image: create_image_links(&raw.image),
        top_songs,
        top_albums,
        singles,
        similar_artists,
    }
}

// --- Axum Request / Query Structs ---

#[derive(Debug, Deserialize, IntoParams)]
pub struct ArtistQuery {
    /// The unique ID of the artist
    pub id: Option<String>,
    /// A direct link to the artist overview page on JioSaavn
    pub link: Option<String>,
    pub page: Option<i32>,
    #[serde(rename = "songCount")]
    pub song_count: Option<i32>,
    #[serde(rename = "albumCount")]
    pub album_count: Option<i32>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ArtistSubQuery {
    pub page: Option<i32>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

// --- API Service Methods ---

pub async fn get_artist_by_id(
    artist_id: &str,
    page: i32,
    song_count: i32,
    album_count: i32,
    sort_by: &str,
    sort_order: &str,
) -> Result<Artist, StatusCode> {
    let result = use_fetch::<RawArtist>(
        crate::common::constants::calls::artists::ID,
        vec![
            ("artistId".to_string(), artist_id.to_string()),
            ("n_song".to_string(), song_count.to_string()),
            ("n_album".to_string(), album_count.to_string()),
            ("page".to_string(), page.to_string()),
            ("sort_order".to_string(), sort_order.to_string()),
            ("category".to_string(), sort_by.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw_artist) => {
            if raw_artist.name.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(map_artist(raw_artist))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_artist_by_link(
    link: &str,
    page: i32,
    song_count: i32,
    album_count: i32,
    sort_by: &str,
    sort_order: &str,
) -> Result<Artist, StatusCode> {
    let re = Regex::new(r"jiosaavn\.com/artist/[^/]+/([^/]+)$").unwrap();
    let token = match re.captures(link) {
        Some(cap) => cap.get(1).map(|m| m.as_str()).unwrap_or(""),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let result = use_fetch::<RawArtist>(
        "webapi.get",
        vec![
            ("token".to_string(), token.to_string()),
            ("type".to_string(), "artist".to_string()),
            ("n_song".to_string(), song_count.to_string()),
            ("n_album".to_string(), album_count.to_string()),
            ("page".to_string(), page.to_string()),
            ("sort_order".to_string(), sort_order.to_string()),
            ("category".to_string(), sort_by.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw_artist) => {
            if raw_artist.name.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            Ok(map_artist(raw_artist))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_artist_songs(
    artist_id: &str,
    page: i32,
    sort_by: &str,
    sort_order: &str,
) -> Result<ArtistSongs, StatusCode> {
    let result = use_fetch::<RawArtistSongsWrapper>(
        crate::common::constants::calls::artists::SONGS,
        vec![
            ("artistId".to_string(), artist_id.to_string()),
            ("page".to_string(), page.to_string()),
            ("sort_order".to_string(), sort_order.to_string()),
            ("category".to_string(), sort_by.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(wrapper) => Ok(ArtistSongs {
            total: wrapper.top_songs.total,
            songs: wrapper.top_songs.songs.into_iter().map(map_song).collect(),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_artist_albums(
    artist_id: &str,
    page: i32,
    sort_by: &str,
    sort_order: &str,
) -> Result<ArtistAlbums, StatusCode> {
    let result = use_fetch::<RawArtistAlbumsWrapper>(
        crate::common::constants::calls::artists::ALBUMS,
        vec![
            ("artistId".to_string(), artist_id.to_string()),
            ("page".to_string(), page.to_string()),
            ("sort_order".to_string(), sort_order.to_string()),
            ("category".to_string(), sort_by.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(wrapper) => Ok(ArtistAlbums {
            total: wrapper.top_albums.total,
            albums: wrapper.top_albums.albums.into_iter().map(map_album).collect(),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- Axum Route Handlers ---

#[utoipa::path(
    get,
    path = "/api/artists",
    params(ArtistQuery),
    responses(
        (status = 200, description = "Successful response with artist details", body = ApiResponse<Artist>),
        (status = 400, description = "Bad request when query parameters are missing or invalid"),
        (status = 404, description = "Artist not found with the given ID or link")
    ),
    tag = "Artists"
)]
pub async fn get_artist_handler(
    Query(query): Query<ArtistQuery>,
) -> impl IntoResponse {
    if query.id.is_none() && query.link.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "success": false,
                "message": "Either artist ID or link is required"
            })),
        )
            .into_response();
    }

    let page = query.page.unwrap_or(0);
    let song_count = query.song_count.unwrap_or(10);
    let album_count = query.album_count.unwrap_or(10);
    let sort_by = query.sort_by.unwrap_or_else(|| "popularity".to_string());
    let sort_order = query.sort_order.unwrap_or_else(|| "asc".to_string());

    let result = if let Some(link) = query.link {
        get_artist_by_link(&link, page, song_count, album_count, &sort_by, &sort_order).await
    } else {
        get_artist_by_id(&query.id.unwrap(), page, song_count, album_count, &sort_by, &sort_order).await
    };

    match result {
        Ok(artist) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: artist,
            }),
        )
            .into_response(),
        Err(status) => {
            let message = match status {
                StatusCode::BAD_REQUEST => "Invalid link formatting",
                StatusCode::NOT_FOUND => "Artist not found",
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
    path = "/api/artists/{id}",
    params(
        ("id" = String, Path, description = "ID of the artist to retrieve details for"),
        ArtistSubQuery
    ),
    responses(
        (status = 200, description = "Successful response with artist details", body = ApiResponse<Artist>),
        (status = 404, description = "Artist not found")
    ),
    tag = "Artists"
)]
pub async fn get_artist_by_id_handler(
    Path(id): Path<String>,
    Query(query): Query<ArtistSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let sort_by = query.sort_by.unwrap_or_else(|| "popularity".to_string());
    let sort_order = query.sort_order.unwrap_or_else(|| "asc".to_string());

    match get_artist_by_id(&id, page, 10, 10, &sort_by, &sort_order).await {
        Ok(artist) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: artist,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Artist not found"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/artists/{id}/songs",
    params(
        ("id" = String, Path, description = "ID of the artist to retrieve songs for"),
        ArtistSubQuery
    ),
    responses(
        (status = 200, description = "Successful response with artist songs", body = ApiResponse<ArtistSongs>),
        (status = 404, description = "Artist songs not found")
    ),
    tag = "Artists"
)]
pub async fn get_artist_songs_handler(
    Path(id): Path<String>,
    Query(query): Query<ArtistSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let sort_by = query.sort_by.unwrap_or_else(|| "popularity".to_string());
    let sort_order = query.sort_order.unwrap_or_else(|| "desc".to_string());

    match get_artist_songs(&id, page, &sort_by, &sort_order).await {
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
                "message": "Artist songs not found"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/artists/{id}/albums",
    params(
        ("id" = String, Path, description = "ID of the artist to retrieve albums for"),
        ArtistSubQuery
    ),
    responses(
        (status = 200, description = "Successful response with artist albums", body = ApiResponse<ArtistAlbums>),
        (status = 404, description = "Artist albums not found")
    ),
    tag = "Artists"
)]
pub async fn get_artist_albums_handler(
    Path(id): Path<String>,
    Query(query): Query<ArtistSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let sort_by = query.sort_by.unwrap_or_else(|| "popularity".to_string());
    let sort_order = query.sort_order.unwrap_or_else(|| "desc".to_string());

    match get_artist_albums(&id, page, &sort_by, &sort_order).await {
        Ok(albums) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: albums,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Artist albums not found"
            })),
        )
            .into_response(),
    }
}
