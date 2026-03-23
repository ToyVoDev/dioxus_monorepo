DROP TRIGGER IF EXISTS tracks_updated_at ON tracks;
DROP TRIGGER IF EXISTS playlists_updated_at ON playlists;
DROP FUNCTION IF EXISTS update_updated_at_column();
ALTER TABLE tracks DROP COLUMN IF EXISTS updated_at;
ALTER TABLE playlists DROP COLUMN IF EXISTS updated_at;
