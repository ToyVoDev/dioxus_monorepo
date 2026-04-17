//! HTTP client for the Jellyfin-compatible API.
//! Works on both native (reqwest with hyper) and WASM (reqwest with fetch).

use dioxus::prelude::*;
use dioxus_music_api::types::{
    AuthenticationResult, BaseItemDto, CreatePlaylistRequest, CreateSmartPlaylistRequest,
    ItemsResult, SearchHintsResult, SmartPlaylistRules, UpdatePlaylistRequest, UserItemDataDto,
};
use reqwest::Client;
use uuid::Uuid;

/// Shared API client. Holds the base URL and current auth token.
#[derive(Clone, Debug, Default)]
pub struct ApiClient {
    pub client: Client,
    pub base_url: String,
    pub token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            token: None,
        }
    }

    pub(crate) fn auth_header(&self) -> String {
        match &self.token {
            Some(t) => format!(
                r#"MediaBrowser Client="DioxusMusic", Device="Web", DeviceId="web-1", Version="1.0", Token="{t}""#
            ),
            None => {
                r#"MediaBrowser Client="DioxusMusic", Device="Web", DeviceId="web-1", Version="1.0""#
                    .to_string()
            }
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    // ── Auth ──────────────────────────────────────────────────────────────

    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthenticationResult, reqwest::Error> {
        let result: AuthenticationResult = self
            .client
            .post(self.url("/Users/AuthenticateByName"))
            .header("Authorization", self.auth_header())
            .json(&serde_json::json!({ "Username": username, "Pw": password }))
            .send()
            .await?
            .json()
            .await?;
        self.token = Some(result.access_token.clone());
        Ok(result)
    }

    // ── Library ───────────────────────────────────────────────────────────

    pub async fn get_albums(&self, parent_id: Option<Uuid>) -> Result<ItemsResult, reqwest::Error> {
        let mut url =
            self.url("/Items?IncludeItemTypes=MusicAlbum&SortBy=SortName&SortOrder=Ascending");
        if let Some(id) = parent_id {
            url.push_str(&format!("&ParentId={id}"));
        }
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_album_tracks(&self, album_id: Uuid) -> Result<ItemsResult, reqwest::Error> {
        let url = self.url(&format!(
            "/Items?IncludeItemTypes=Audio&ParentId={album_id}&SortBy=ParentIndexNumber,IndexNumber&SortOrder=Ascending"
        ));
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tracks(&self) -> Result<ItemsResult, reqwest::Error> {
        let url = self.url(
            "/Items?IncludeItemTypes=Audio&SortBy=AlbumArtist,Album,ParentIndexNumber,IndexNumber&SortOrder=Ascending",
        );
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_artists(&self) -> Result<ItemsResult, reqwest::Error> {
        let url = self.url("/Artists/AlbumArtists?SortBy=SortName&SortOrder=Ascending");
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_item(&self, item_id: Uuid) -> Result<BaseItemDto, reqwest::Error> {
        let url = self.url(&format!("/Items/{item_id}"));
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_genres(&self) -> Result<ItemsResult, reqwest::Error> {
        let url = self.url("/MusicGenres");
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn search(
        &self,
        term: &str,
        limit: u32,
    ) -> Result<SearchHintsResult, reqwest::Error> {
        let url = self.url(&format!(
            "/Search/Hints?SearchTerm={}&Limit={limit}",
            urlencoding::encode(term)
        ));
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    // ── Playlists ─────────────────────────────────────────────────────────

    pub async fn get_playlists(&self) -> Result<ItemsResult, reqwest::Error> {
        let url = self.url("/Items?IncludeItemTypes=Playlist");
        self.client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_playlist(&self, id: Uuid) -> Result<BaseItemDto, reqwest::Error> {
        self.client
            .get(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_playlist_items(&self, id: Uuid) -> Result<ItemsResult, reqwest::Error> {
        self.client
            .get(self.url(&format!("/Playlists/{id}/Items")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn create_playlist(&self, name: &str) -> Result<BaseItemDto, reqwest::Error> {
        self.client
            .post(self.url("/Playlists"))
            .header("Authorization", self.auth_header())
            .json(&CreatePlaylistRequest {
                name: name.to_string(),
                ids: None,
                user_id: None,
                media_type: Some("Audio".to_string()),
            })
            .send()
            .await?
            .json()
            .await
    }

    pub async fn create_smart_playlist(
        &self,
        name: &str,
        rules: SmartPlaylistRules,
    ) -> Result<BaseItemDto, reqwest::Error> {
        self.client
            .post(self.url("/custom/playlists/smart"))
            .header("Authorization", self.auth_header())
            .json(&CreateSmartPlaylistRequest {
                name: name.to_string(),
                rules,
                user_id: None,
            })
            .send()
            .await?
            .json()
            .await
    }

    pub async fn update_playlist(
        &self,
        id: Uuid,
        name: Option<String>,
    ) -> Result<(), reqwest::Error> {
        self.client
            .post(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .json(&UpdatePlaylistRequest {
                name,
                overview: None,
            })
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn delete_playlist(&self, id: Uuid) -> Result<(), reqwest::Error> {
        self.client
            .delete(self.url(&format!("/Playlists/{id}")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn add_to_playlist(
        &self,
        playlist_id: Uuid,
        track_ids: &[Uuid],
    ) -> Result<(), reqwest::Error> {
        let ids = track_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.url(&format!("/Playlists/{playlist_id}/Items?Ids={ids}"));
        self.client
            .post(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn remove_from_playlist(
        &self,
        playlist_id: Uuid,
        entry_ids: &[Uuid],
    ) -> Result<(), reqwest::Error> {
        let ids = entry_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.url(&format!("/Playlists/{playlist_id}/Items?EntryIds={ids}"));
        self.client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn update_smart_rules(
        &self,
        playlist_id: Uuid,
        rules: SmartPlaylistRules,
    ) -> Result<(), reqwest::Error> {
        self.client
            .post(self.url(&format!("/custom/playlists/{playlist_id}/rules")))
            .header("Authorization", self.auth_header())
            .json(&rules)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ── User data ─────────────────────────────────────────────────────────

    pub async fn mark_favorite(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client
            .post(self.url(&format!("/UserFavoriteItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn unmark_favorite(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client
            .delete(self.url(&format!("/UserFavoriteItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    pub async fn mark_played(&self, item_id: Uuid) -> Result<UserItemDataDto, reqwest::Error> {
        self.client
            .post(self.url(&format!("/UserPlayedItems/{item_id}")))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json()
            .await
    }

    // ── Library management ────────────────────────────────────────────────

    pub async fn rescan_library(&self) -> Result<(), reqwest::Error> {
        self.client
            .post(self.url("/custom/library/rescan"))
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    // ── Streaming ─────────────────────────────────────────────────────────

    pub fn stream_url(&self, track_id: Uuid) -> String {
        match &self.token {
            Some(t) => self.url(&format!("/Audio/{track_id}/stream?api_key={t}")),
            None => self.url(&format!("/Audio/{track_id}/stream")),
        }
    }

    pub fn image_url(&self, item_id: Uuid, image_type: &str) -> String {
        match &self.token {
            Some(t) => self.url(&format!("/Items/{item_id}/Images/{image_type}?api_key={t}")),
            None => self.url(&format!("/Items/{item_id}/Images/{image_type}")),
        }
    }
}

/// Hook to get the current [`ApiClient`] from context.
/// Reads from the [`Signal<ApiClient>`] provided by the platform layout.
pub fn use_api_client() -> ApiClient {
    use_context::<Signal<ApiClient>>().read().clone()
}
