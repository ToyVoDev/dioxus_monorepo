CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin      BOOLEAN NOT NULL DEFAULT false,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT users_name_unique UNIQUE (name)
);

CREATE TABLE access_tokens (
    token        TEXT PRIMARY KEY,
    user_id      UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    device_id    TEXT NOT NULL,
    device_name  TEXT NOT NULL,
    client_name  TEXT NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX access_tokens_user_id_idx ON access_tokens (user_id);

CREATE TABLE artists (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    sort_name  TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE albums (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title      TEXT NOT NULL,
    sort_title TEXT NOT NULL,
    artist_id  UUID NOT NULL REFERENCES artists,
    year       INT4,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX albums_artist_id_idx ON albums (artist_id);

CREATE TABLE tracks (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title          TEXT NOT NULL,
    sort_title     TEXT NOT NULL,
    artist_id      UUID NOT NULL REFERENCES artists,
    album_id       UUID REFERENCES albums,
    genre          TEXT NOT NULL DEFAULT '',
    duration_ticks INT8 NOT NULL DEFAULT 0,
    track_number   INT4,
    disc_number    INT4 NOT NULL DEFAULT 1,
    file_path      TEXT NOT NULL,
    container      TEXT NOT NULL,
    bit_rate       INT4,
    sample_rate    INT4,
    channels       INT4,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT tracks_file_path_unique UNIQUE (file_path)
);
CREATE INDEX tracks_artist_id_idx ON tracks (artist_id);
CREATE INDEX tracks_album_id_idx ON tracks (album_id);

CREATE TABLE genres (
    id   UUID PRIMARY KEY,
    name TEXT NOT NULL,
    CONSTRAINT genres_name_unique UNIQUE (name)
);

CREATE TABLE images (
    item_id    UUID NOT NULL,
    image_type TEXT NOT NULL,
    file_path  TEXT NOT NULL,
    tag        TEXT NOT NULL,
    width      INT4,
    height     INT4,
    PRIMARY KEY (item_id, image_type)
);

CREATE TABLE playlists (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    overview   TEXT,
    is_smart   BOOLEAN NOT NULL DEFAULT false,
    user_id    UUID REFERENCES users,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE playlist_items (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    playlist_id UUID NOT NULL REFERENCES playlists ON DELETE CASCADE,
    item_id     UUID NOT NULL REFERENCES tracks ON DELETE CASCADE,
    position    INT4 NOT NULL,
    CONSTRAINT playlist_items_unique UNIQUE (playlist_id, item_id)
);
CREATE INDEX playlist_items_playlist_position_idx ON playlist_items (playlist_id, position);

CREATE TABLE user_data (
    user_id                 UUID NOT NULL REFERENCES users ON DELETE CASCADE,
    item_id                 UUID NOT NULL,
    item_type               TEXT NOT NULL,
    is_favorite             BOOLEAN NOT NULL DEFAULT false,
    likes                   BOOLEAN,
    play_count              INT4 NOT NULL DEFAULT 0,
    last_played_date        TIMESTAMPTZ,
    played                  BOOLEAN NOT NULL DEFAULT false,
    playback_position_ticks INT8 NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, item_id)
);
