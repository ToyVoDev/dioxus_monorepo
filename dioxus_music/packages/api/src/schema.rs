// @generated automatically by Diesel CLI.

diesel::table! {
    tracks (id) {
        id -> Uuid,
        title -> Text,
        artist -> Text,
        album -> Text,
        genre -> Text,
        duration_secs -> Int4,
        file_path -> Text,
    }
}

diesel::table! {
    playlists (id) {
        id -> Uuid,
        name -> Text,
        playlist_type -> Text,
        rules -> Nullable<Jsonb>,
    }
}

diesel::table! {
    playlist_tracks (id) {
        id -> Uuid,
        playlist_id -> Uuid,
        track_id -> Uuid,
        position -> Int4,
    }
}

diesel::joinable!(playlist_tracks -> playlists (playlist_id));
diesel::joinable!(playlist_tracks -> tracks (track_id));

diesel::allow_tables_to_appear_in_same_query!(tracks, playlists, playlist_tracks);
