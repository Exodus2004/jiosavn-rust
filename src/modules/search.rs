use crate::common::helpers::{create_image_links, use_fetch};
use crate::common::types::{ApiResponse, DownloadLink};
use crate::modules::songs::{map_artist_mini, ArtistMini, RawArtistMini};
use crate::modules::songs::{map_song, RawArtistMap, RawSong, Song, SongArtists};
use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// --- Raw API Deserialization Models ---

#[derive(Debug, Deserialize)]
pub struct RawSearchAll {
    pub topquery: Option<RawSearchSection<RawSearchTopQueryItem>>,
    pub songs: Option<RawSearchSection<RawSearchSongItem>>,
    pub albums: Option<RawSearchSection<RawSearchAlbumItem>>,
    pub artists: Option<RawSearchSection<RawSearchArtistItem>>,
    pub playlists: Option<RawSearchSection<RawSearchPlaylistItem>>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchSection<T> {
    pub data: Vec<T>,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchTopQueryItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub more_info: Option<RawSearchTopQueryMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchTopQueryMoreInfo {
    pub album: Option<String>,
    pub language: Option<String>,
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchSongItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub more_info: Option<RawSearchSongMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchSongMoreInfo {
    pub album: Option<String>,
    pub language: Option<String>,
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchAlbumItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub more_info: Option<RawSearchAlbumMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchAlbumMoreInfo {
    pub music: Option<String>,
    pub year: Option<String>,
    pub song_pids: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchArtistItem {
    pub id: String,
    pub title: String,
    pub image: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchPlaylistItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub perma_url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub more_info: Option<RawSearchPlaylistMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchPlaylistMoreInfo {
    pub language: Option<String>,
}

// --- Specific Search Raw Models ---

#[derive(Debug, Deserialize)]
pub struct RawSearchSongs {
    pub total: i32,
    pub start: i32,
    pub results: Option<Vec<RawSong>>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchAlbums {
    pub total: i32,
    pub start: i32,
    pub results: Option<Vec<RawSearchAlbumsItem>>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchAlbumsItem {
    pub id: String,
    pub title: String,
    pub header_desc: Option<String>,
    pub perma_url: String,
    pub year: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub play_count: Option<String>,
    pub language: String,
    pub explicit_content: Option<String>,
    pub image: String,
    pub more_info: Option<RawSearchAlbumsMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchAlbumsMoreInfo {
    #[serde(rename = "artistMap")]
    pub artist_map: Option<RawArtistMap>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchArtists {
    pub total: i32,
    pub start: i32,
    pub results: Option<Vec<RawArtistMini>>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchPlaylists {
    pub total: i32,
    pub start: i32,
    pub results: Option<Vec<RawSearchPlaylistItemInner>>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchPlaylistItemInner {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub image: String,
    pub perma_url: String,
    pub explicit_content: Option<String>,
    pub more_info: Option<RawSearchPlaylistItemMoreInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchPlaylistItemMoreInfo {
    pub song_count: Option<String>,
    pub language: Option<String>,
}

// --- Outbound Client API Models ---

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchTopQueryItem {
    pub id: String,
    pub title: String,
    pub image: Vec<DownloadLink>,
    pub album: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub language: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "primaryArtists")]
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchSongItem {
    pub id: String,
    pub title: String,
    pub image: Vec<DownloadLink>,
    pub album: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    #[serde(rename = "primaryArtists")]
    pub primary_artists: Option<String>,
    pub singers: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchAlbumItem {
    pub id: String,
    pub title: String,
    pub image: Vec<DownloadLink>,
    pub artist: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub year: Option<String>,
    #[serde(rename = "songIds")]
    pub song_ids: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchArtistItem {
    pub id: String,
    pub title: String,
    pub image: Vec<DownloadLink>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: Option<String>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchPlaylistItem {
    pub id: String,
    pub title: String,
    pub image: Vec<DownloadLink>,
    pub url: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub language: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchSection<T> {
    pub results: Vec<T>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchAllResponse {
    #[serde(rename = "topQuery")]
    pub top_query: SearchSection<SearchTopQueryItem>,
    pub songs: SearchSection<SearchSongItem>,
    pub albums: SearchSection<SearchAlbumItem>,
    pub artists: SearchSection<SearchArtistItem>,
    pub playlists: SearchSection<SearchPlaylistItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchSongsResponse {
    pub total: i32,
    pub start: i32,
    pub results: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchAlbumResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub year: Option<i32>,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(rename = "playCount")]
    pub play_count: Option<i32>,
    pub language: String,
    #[serde(rename = "explicitContent")]
    pub explicit_content: bool,
    pub artists: SongArtists,
    pub image: Vec<DownloadLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchAlbumsResponse {
    pub total: i32,
    pub start: i32,
    pub results: Vec<SearchAlbumResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchArtistsResponse {
    pub total: i32,
    pub start: i32,
    pub results: Vec<ArtistMini>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchPlaylistResult {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub image: Vec<DownloadLink>,
    pub url: String,
    #[serde(rename = "songCount")]
    pub song_count: Option<i32>,
    pub language: Option<String>,
    #[serde(rename = "explicitContent")]
    pub explicit_content: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SearchPlaylistsResponse {
    pub total: i32,
    pub start: i32,
    pub results: Vec<SearchPlaylistResult>,
}

// --- Mapping Helpers ---

pub fn map_search_all(raw: RawSearchAll) -> SearchAllResponse {
    let top_query_raw = raw.topquery.unwrap_or_else(|| RawSearchSection { data: vec![], position: 0 });
    let top_query_results = top_query_raw.data.into_iter().map(|item| {
        let more_info = item.more_info.unwrap_or_else(|| RawSearchTopQueryMoreInfo {
            album: None,
            language: None,
            primary_artists: None,
            singers: None,
        });
        SearchTopQueryItem {
            id: item.id,
            title: item.title,
            image: create_image_links(&item.image),
            album: more_info.album,
            url: item.perma_url,
            item_type: item.item_type,
            language: more_info.language,
            description: item.description,
            primary_artists: more_info.primary_artists,
            singers: more_info.singers,
        }
    }).collect();

    let songs_raw = raw.songs.unwrap_or_else(|| RawSearchSection { data: vec![], position: 0 });
    let songs_results = songs_raw.data.into_iter().map(|item| {
        let more_info = item.more_info.unwrap_or_else(|| RawSearchSongMoreInfo {
            album: None,
            language: None,
            primary_artists: None,
            singers: None,
        });
        SearchSongItem {
            id: item.id,
            title: item.title,
            image: create_image_links(&item.image),
            album: more_info.album,
            url: item.perma_url,
            item_type: item.item_type,
            description: item.description,
            primary_artists: more_info.primary_artists,
            singers: more_info.singers,
            language: more_info.language,
        }
    }).collect();

    let albums_raw = raw.albums.unwrap_or_else(|| RawSearchSection { data: vec![], position: 0 });
    let albums_results = albums_raw.data.into_iter().map(|item| {
        let more_info = item.more_info.unwrap_or_else(|| RawSearchAlbumMoreInfo {
            music: None,
            year: None,
            song_pids: None,
            language: None,
        });
        SearchAlbumItem {
            id: item.id,
            title: item.title,
            image: create_image_links(&item.image),
            artist: more_info.music,
            url: item.perma_url,
            item_type: item.item_type,
            description: item.description,
            year: more_info.year,
            song_ids: more_info.song_pids,
            language: more_info.language,
        }
    }).collect();

    let artists_raw = raw.artists.unwrap_or_else(|| RawSearchSection { data: vec![], position: 0 });
    let artists_results = artists_raw.data.into_iter().map(|item| {
        SearchArtistItem {
            id: item.id,
            title: item.title,
            image: create_image_links(&item.image),
            item_type: item.item_type,
            description: item.description,
            position: item.position,
        }
    }).collect();

    let playlists_raw = raw.playlists.unwrap_or_else(|| RawSearchSection { data: vec![], position: 0 });
    let playlists_results = playlists_raw.data.into_iter().map(|item| {
        let more_info = item.more_info.unwrap_or_else(|| RawSearchPlaylistMoreInfo {
            language: None,
        });
        SearchPlaylistItem {
            id: item.id,
            title: item.title,
            image: create_image_links(&item.image),
            url: item.perma_url,
            item_type: item.item_type,
            language: more_info.language,
            description: item.description,
        }
    }).collect();

    SearchAllResponse {
        top_query: SearchSection { results: top_query_results, position: top_query_raw.position },
        songs: SearchSection { results: songs_results, position: songs_raw.position },
        albums: SearchSection { results: albums_results, position: albums_raw.position },
        artists: SearchSection { results: artists_results, position: artists_raw.position },
        playlists: SearchSection { results: playlists_results, position: playlists_raw.position },
    }
}

pub fn map_search_album_result(item: RawSearchAlbumsItem) -> SearchAlbumResult {
    let more_info = item.more_info.unwrap_or_else(|| RawSearchAlbumsMoreInfo {
        artist_map: None,
    });

    let artist_map = more_info.artist_map.unwrap_or_else(|| RawArtistMap {
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

    SearchAlbumResult {
        id: item.id,
        name: item.title,
        description: item.header_desc,
        url: item.perma_url,
        year: item.year.and_then(|y| y.parse::<i32>().ok()),
        item_type: item.item_type,
        play_count: item.play_count.and_then(|p| p.parse::<i32>().ok()),
        language: item.language,
        explicit_content: item.explicit_content.map(|e| e == "1").unwrap_or(false),
        artists: SongArtists {
            primary,
            featured,
            all,
        },
        image: create_image_links(&item.image),
    }
}

pub fn map_search_playlist_result(item: RawSearchPlaylistItemInner) -> SearchPlaylistResult {
    let more_info = item.more_info.unwrap_or_else(|| RawSearchPlaylistItemMoreInfo {
        song_count: None,
        language: None,
    });

    SearchPlaylistResult {
        id: item.id,
        name: item.title,
        item_type: item.item_type,
        image: create_image_links(&item.image),
        url: item.perma_url,
        song_count: more_info.song_count.and_then(|s| s.parse::<i32>().ok()),
        language: more_info.language,
        explicit_content: item.explicit_content.map(|e| e == "1").unwrap_or(false),
    }
}

// --- Axum Request / Query Structs ---

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchAllQuery {
    /// Search query string
    pub query: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchSubQuery {
    /// Search query string
    pub query: String,
    pub page: Option<i32>,
    pub limit: Option<usize>,
}

// --- API Service Methods ---

pub async fn search_all(query: &str) -> Result<SearchAllResponse, StatusCode> {
    let result = use_fetch::<RawSearchAll>(
        crate::common::constants::calls::search::ALL,
        vec![("query".to_string(), query.to_string())],
        None,
    )
    .await;

    match result {
        Ok(raw) => Ok(map_search_all(raw)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_songs(query: &str, page: i32, limit: usize) -> Result<SearchSongsResponse, StatusCode> {
    let result = use_fetch::<RawSearchSongs>(
        crate::common::constants::calls::search::SONGS,
        vec![
            ("q".to_string(), query.to_string()),
            ("p".to_string(), page.to_string()),
            ("n".to_string(), limit.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw) => {
            let mut songs: Vec<Song> = raw.results.unwrap_or_default().into_iter().map(map_song).collect();
            songs.truncate(limit);
            Ok(SearchSongsResponse {
                total: raw.total,
                start: raw.start,
                results: songs,
            })
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_albums(query: &str, page: i32, limit: usize) -> Result<SearchAlbumsResponse, StatusCode> {
    let result = use_fetch::<RawSearchAlbums>(
        crate::common::constants::calls::search::ALBUMS,
        vec![
            ("q".to_string(), query.to_string()),
            ("p".to_string(), page.to_string()),
            ("n".to_string(), limit.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw) => {
            let mut results: Vec<SearchAlbumResult> = raw.results.unwrap_or_default().into_iter().map(map_search_album_result).collect();
            results.truncate(limit);
            Ok(SearchAlbumsResponse {
                total: raw.total,
                start: raw.start,
                results,
            })
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_artists(query: &str, page: i32, limit: usize) -> Result<SearchArtistsResponse, StatusCode> {
    let result = use_fetch::<RawSearchArtists>(
        crate::common::constants::calls::search::ARTISTS,
        vec![
            ("q".to_string(), query.to_string()),
            ("p".to_string(), page.to_string()),
            ("n".to_string(), limit.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw) => {
            let mut results: Vec<ArtistMini> = raw.results.unwrap_or_default().into_iter().map(map_artist_mini).collect();
            results.truncate(limit);
            Ok(SearchArtistsResponse {
                total: raw.total,
                start: raw.start,
                results,
            })
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_playlists(query: &str, page: i32, limit: usize) -> Result<SearchPlaylistsResponse, StatusCode> {
    let result = use_fetch::<RawSearchPlaylists>(
        crate::common::constants::calls::search::PLAYLISTS,
        vec![
            ("q".to_string(), query.to_string()),
            ("p".to_string(), page.to_string()),
            ("n".to_string(), limit.to_string()),
        ],
        None,
    )
    .await;

    match result {
        Ok(raw) => {
            let mut results: Vec<SearchPlaylistResult> = raw.results.unwrap_or_default().into_iter().map(map_search_playlist_result).collect();
            results.truncate(limit);
            Ok(SearchPlaylistsResponse {
                total: raw.total,
                start: raw.start,
                results,
            })
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- Axum Route Handlers ---

#[utoipa::path(
    get,
    path = "/api/search",
    params(SearchAllQuery),
    responses(
        (status = 200, description = "Successful response with global search results", body = ApiResponse<SearchAllResponse>)
    ),
    tag = "Search"
)]
pub async fn search_all_handler(
    Query(query): Query<SearchAllQuery>,
) -> impl IntoResponse {
    match search_all(&query.query).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: res,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Search error"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/search/songs",
    params(SearchSubQuery),
    responses(
        (status = 200, description = "Successful response with song search results", body = ApiResponse<SearchSongsResponse>)
    ),
    tag = "Search"
)]
pub async fn search_songs_handler(
    Query(query): Query<SearchSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    match search_songs(&query.query, page, limit).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: res,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Search error"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/search/albums",
    params(SearchSubQuery),
    responses(
        (status = 200, description = "Successful response with album search results", body = ApiResponse<SearchAlbumsResponse>)
    ),
    tag = "Search"
)]
pub async fn search_albums_handler(
    Query(query): Query<SearchSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    match search_albums(&query.query, page, limit).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: res,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Search error"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/search/artists",
    params(SearchSubQuery),
    responses(
        (status = 200, description = "Successful response with artist search results", body = ApiResponse<SearchArtistsResponse>)
    ),
    tag = "Search"
)]
pub async fn search_artists_handler(
    Query(query): Query<SearchSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    match search_artists(&query.query, page, limit).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: res,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Search error"
            })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/search/playlists",
    params(SearchSubQuery),
    responses(
        (status = 200, description = "Successful response with playlist search results", body = ApiResponse<SearchPlaylistsResponse>)
    ),
    tag = "Search"
)]
pub async fn search_playlists_handler(
    Query(query): Query<SearchSubQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);

    match search_playlists(&query.query, page, limit).await {
        Ok(res) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: res,
            }),
        )
            .into_response(),
        Err(status) => (
            status,
            Json(serde_json::json!({
                "success": false,
                "message": "Search error"
            })),
        )
            .into_response(),
    }
}
