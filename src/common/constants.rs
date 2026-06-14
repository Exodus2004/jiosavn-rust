pub const ENDPOINT_URL: &str = "https://www.jiosaavn.com/api.php";

pub const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0",
    "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 18_3_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3.1 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:136.0) Gecko/20100101 Firefox/136.0",
];

// JioSaavn Endpoints Call Names
pub mod calls {
    pub mod search {
        pub const ALL: &str = "autocomplete.get";
        pub const SONGS: &str = "search.getResults";
        pub const ALBUMS: &str = "search.getAlbumResults";
        pub const ARTISTS: &str = "search.getArtistResults";
        pub const PLAYLISTS: &str = "search.getPlaylistResults";
    }
    pub mod songs {
        pub const ID: &str = "song.getDetails";
        pub const SUGGESTIONS: &str = "webradio.getSong";
        pub const LYRICS: &str = "lyrics.getLyrics";
        pub const STATION: &str = "webradio.createEntityStation";
    }
    pub mod albums {
        pub const ID: &str = "content.getAlbumDetails";
    }
    pub mod artists {
        pub const ID: &str = "artist.getArtistPageDetails";
        pub const SONGS: &str = "artist.getArtistMoreSong";
        pub const ALBUMS: &str = "artist.getArtistMoreAlbum";
    }
    pub mod playlists {
        pub const ID: &str = "playlist.getDetails";
    }
}
