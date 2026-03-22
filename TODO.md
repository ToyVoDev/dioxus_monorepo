# Dioxus Monorepo — Outstanding Work

## httpui

### Core Features
- [ ] Request persistence (save/load from disk — SQLite or JSON files)
- [ ] Request body editor (raw text, JSON, form-data, XML, binary)
- [ ] Request headers editor (functional key-value editing with common headers)
- [ ] Authentication flows (Basic, Bearer, API Key, OAuth)
- [ ] Environment variable substitution in URLs/headers/body
- [ ] Import/export (Postman collections, cURL commands, OpenAPI)

### UI Features
- [ ] Search functionality (TopBar search input is visual placeholder)
- [ ] Request/response history tracking
- [ ] Explorer collapse/toggle
- [ ] Multiple open request tabs (data preserved internally, tabbar UI removed)
- [ ] Status bar content (connection status, encoding, etc.)
- [ ] Response timing waterfall visualization

### Placeholder Views
- [ ] History explorer panel (shows "Coming soon")
- [ ] APIs explorer panel (shows "Coming soon")
- [ ] Mock Servers explorer panel (shows "Coming soon")
- [ ] Authorization tab in request editor (shows placeholder)
- [ ] Body tab in request editor (shows placeholder)
- [ ] Settings tab in request editor (shows placeholder)

### Advanced
- [ ] Request scripting (pre-request/post-response)
- [ ] WebSocket support
- [ ] File upload
- [ ] Collection sharing
- [ ] API documentation / Mock Server features

---

## dioxus_music

### Playback
- [ ] Seekable progress bar (JS interop with audio `currentTime`)
- [ ] Volume control (JS interop with audio `volume`)
- [ ] Album cover art (currently character placeholder — need file path or embedded art)
- [ ] Now Playing route for mobile crate
- [ ] Visualizer bars / audio waveform

### Library
- [ ] Artist detail view (Artists route shows "Coming soon")
- [ ] Album entity in database (currently derived client-side from track metadata)
- [ ] Album cover art extraction from audio files (lofty supports embedded art)
- [ ] Recently Played section (needs play history tracking)
- [ ] Search functionality (Header search input is visual placeholder)
- [ ] Popularity/play count tracking

### Metadata
- [ ] Real codec/bitrate/source in Now Playing (currently hardcoded "FLAC" / "— kbps" / "Local")
- [ ] Track metadata editing

### Offline Sync (architecture designed, not implemented)
- [ ] Server health endpoint + incremental sync
- [ ] Desktop connectivity detection + download manager
- [ ] Cache management with LRU eviction
- [ ] Playlist-level sync toggle ("Mark for offline")
- [ ] Opportunistic caching after streaming
- [ ] Download progress tracking (TrackList status column is visual placeholder)
- [ ] Storage breakdown (Sync Manager cards are placeholders)
- [ ] Transcoding settings

### Mobile
- [ ] Mobile home view (shows placeholder)
- [ ] Mobile Now Playing (swipe-up from compact PlayerBar)
- [ ] Mobile-specific views for Library/Albums/Playlists

### Platform
- [ ] Android Auto (MediaBrowserService + MediaSession)
- [ ] iOS CarPlay (Core Data + AVQueuePlayer)

---

## kinetic_ui (shared component library)

- [ ] Tooltip component runtime verification (built but untested in real usage)
- [ ] Accordion edge cases
- [ ] Component documentation / storybook-style examples
- [ ] Light mode support (currently dark-only)

---

## Other Projects

- [ ] discord_bot: reaction-based role assignment (TODO in source)
- [ ] discord_bot: game state integration (TODO in source)
- [ ] game_manager: tshock URL from state (TODO in source)
