// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (token) {
        token -> Text,
        user_id -> Uuid,
        device_id -> Text,
        device_name -> Text,
        client_name -> Text,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
    }
}

diesel::table! {
    albums (id) {
        id -> Uuid,
        title -> Text,
        sort_title -> Text,
        artist_id -> Uuid,
        year -> Nullable<Int4>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    artists (id) {
        id -> Uuid,
        name -> Text,
        sort_name -> Text,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    genres (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    images (item_id, image_type) {
        item_id -> Uuid,
        image_type -> Text,
        file_path -> Text,
        tag -> Text,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
    }
}

diesel::table! {
    playlist_items (id) {
        id -> Uuid,
        playlist_id -> Uuid,
        item_id -> Uuid,
        position -> Int4,
    }
}

diesel::table! {
    playlists (id) {
        id -> Uuid,
        name -> Text,
        overview -> Nullable<Text>,
        is_smart -> Bool,
        user_id -> Nullable<Uuid>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tracks (id) {
        id -> Uuid,
        title -> Text,
        sort_title -> Text,
        artist_id -> Uuid,
        album_id -> Nullable<Uuid>,
        genre -> Text,
        duration_ticks -> Int8,
        track_number -> Nullable<Int4>,
        disc_number -> Int4,
        file_path -> Text,
        container -> Text,
        bit_rate -> Nullable<Int4>,
        sample_rate -> Nullable<Int4>,
        channels -> Nullable<Int4>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_data (user_id, item_id) {
        user_id -> Uuid,
        item_id -> Uuid,
        item_type -> Text,
        is_favorite -> Bool,
        likes -> Nullable<Bool>,
        play_count -> Int4,
        last_played_date -> Nullable<Timestamptz>,
        played -> Bool,
        playback_position_ticks -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        password_hash -> Text,
        is_admin -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(playlist_items -> playlists (playlist_id));
diesel::joinable!(playlist_items -> tracks (item_id));
diesel::joinable!(playlists -> users (user_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> artists (artist_id));
diesel::joinable!(user_data -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    albums,
    artists,
    genres,
    images,
    playlist_items,
    playlists,
    tracks,
    user_data,
    users,
);
