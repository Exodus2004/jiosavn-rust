use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod common;
mod modules;
mod pages;

#[derive(OpenApi)]
#[openapi(
    paths(
        modules::songs::get_songs_handler,
        modules::songs::get_song_by_id_handler,
        modules::songs::get_song_suggestions_handler,
        modules::albums::get_album_handler,
        modules::artists::get_artist_handler,
        modules::artists::get_artist_by_id_handler,
        modules::artists::get_artist_songs_handler,
        modules::artists::get_artist_albums_handler,
        modules::playlists::get_playlist_handler,
        modules::search::search_all_handler,
        modules::search::search_songs_handler,
        modules::search::search_albums_handler,
        modules::search::search_artists_handler,
        modules::search::search_playlists_handler,
    ),
    components(
        schemas(
            common::types::DownloadLink,
            modules::songs::Song,
            modules::songs::SongAlbum,
            modules::songs::SongArtists,
            modules::songs::ArtistMini,
            modules::albums::Album,
            modules::artists::Artist,
            modules::artists::ArtistSongs,
            modules::artists::ArtistAlbums,
            modules::artists::SimilarArtist,
            modules::playlists::Playlist,
            modules::search::SearchAllResponse,
            modules::search::SearchTopQueryItem,
            modules::search::SearchSongItem,
            modules::search::SearchAlbumItem,
            modules::search::SearchArtistItem,
            modules::search::SearchPlaylistItem,
            modules::search::SearchSongsResponse,
            modules::search::SearchAlbumResult,
            modules::search::SearchAlbumsResponse,
            modules::search::SearchArtistsResponse,
            modules::search::SearchPlaylistResult,
            modules::search::SearchPlaylistsResponse,
        )
    ),
    tags(
        (name = "Songs", description = "Song details endpoints"),
        (name = "Albums", description = "Album details endpoints"),
        (name = "Artists", description = "Artist details endpoints"),
        (name = "Playlists", description = "Playlist details endpoints"),
        (name = "Search", description = "Search query endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "jiosaavn_api_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // CORS configuration
    let cors = CorsLayer::permissive();

    // App routes
    let app = Router::new()
        // Home landing page
        .route("/", get(pages::home_handler))
        // API routes
        .route("/api/songs", get(modules::songs::get_songs_handler))
        .route("/api/songs/:id", get(modules::songs::get_song_by_id_handler))
        .route("/api/songs/:id/suggestions", get(modules::songs::get_song_suggestions_handler))
        .route("/api/albums", get(modules::albums::get_album_handler))
        .route("/api/artists", get(modules::artists::get_artist_handler))
        .route("/api/artists/:id", get(modules::artists::get_artist_by_id_handler))
        .route("/api/artists/:id/songs", get(modules::artists::get_artist_songs_handler))
        .route("/api/artists/:id/albums", get(modules::artists::get_artist_albums_handler))
        .route("/api/playlists", get(modules::playlists::get_playlist_handler))
        .route("/api/search", get(modules::search::search_all_handler))
        .route("/api/search/songs", get(modules::search::search_songs_handler))
        .route("/api/search/albums", get(modules::search::search_albums_handler))
        .route("/api/search/artists", get(modules::search::search_artists_handler))
        .route("/api/search/playlists", get(modules::search::search_playlists_handler))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Apply CORS middleware
        .layer(cors);

    // Bind to address
    let port = 8787;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
