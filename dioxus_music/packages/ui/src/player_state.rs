use dioxus::prelude::*;
use dioxus_music_api::types::BaseItemDto;

use crate::api_client::{ApiClient, use_api_client};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    All,
    One,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub current_track: Option<BaseItemDto>,
    pub queue: Vec<BaseItemDto>,
    pub queue_index: usize,
    pub is_playing: bool,
    pub repeat_mode: RepeatMode,
    pub is_shuffled: bool,
    pub show_queue: bool,
    pub current_time: f64,
    pub duration: f64,
    pub api_client: ApiClient,
    original_queue: Vec<BaseItemDto>,
    original_index: usize,
}

impl PlayerState {
    pub fn new(api_client: ApiClient) -> Self {
        Self {
            api_client,
            ..Default::default()
        }
    }

    pub fn play_track(&mut self, track: BaseItemDto, queue: Vec<BaseItemDto>, index: usize) {
        self.current_track = Some(track);
        self.queue = queue;
        self.queue_index = index;
        self.is_playing = true;
        self.is_shuffled = false;
        self.current_time = 0.0;
        self.duration = 0.0;
        self.original_queue.clear();

        if let Some(track) = &self.current_track {
            let client = self.api_client.clone();
            let track_id = track.id;
            spawn(async move {
                let _ = client
                    .client
                    .post(format!("{}/Sessions/Playing", client.base_url))
                    .header("Authorization", client.auth_header())
                    .json(&serde_json::json!({
                        "ItemId": track_id,
                        "PositionTicks": 0,
                        "IsPaused": false,
                    }))
                    .send()
                    .await;
            });
        }
    }

    pub fn next_track(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        match self.repeat_mode {
            RepeatMode::One => {
                // Audio loop handles replay; just keep current track playing
                self.is_playing = true;
            }
            RepeatMode::All => {
                if self.queue_index + 1 < self.queue.len() {
                    self.queue_index += 1;
                } else {
                    self.queue_index = 0;
                }
                self.current_track = Some(self.queue[self.queue_index].clone());
                self.is_playing = true;
            }
            RepeatMode::Off => {
                if self.queue_index + 1 < self.queue.len() {
                    self.queue_index += 1;
                    self.current_track = Some(self.queue[self.queue_index].clone());
                    self.is_playing = true;
                } else {
                    self.is_playing = false;
                }
            }
        }
    }

    pub fn prev_track(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        match self.repeat_mode {
            RepeatMode::All => {
                if self.queue_index > 0 {
                    self.queue_index -= 1;
                } else {
                    self.queue_index = self.queue.len() - 1;
                }
                self.current_track = Some(self.queue[self.queue_index].clone());
                self.is_playing = true;
            }
            _ => {
                if self.queue_index > 0 {
                    self.queue_index -= 1;
                    self.current_track = Some(self.queue[self.queue_index].clone());
                    self.is_playing = true;
                }
            }
        }
    }

    pub fn toggle_repeat(&mut self) {
        self.repeat_mode = match self.repeat_mode {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        };
    }

    pub fn toggle_shuffle(&mut self) {
        if self.is_shuffled {
            // Restore original order
            if let Some(current) = &self.current_track {
                let current_id = current.id;
                self.queue = self.original_queue.clone();
                self.queue_index = self
                    .queue
                    .iter()
                    .position(|t| t.id == current_id)
                    .unwrap_or(self.original_index);
            } else {
                self.queue = self.original_queue.clone();
                self.queue_index = self.original_index;
            }
            self.original_queue.clear();
            self.is_shuffled = false;
        } else {
            // Save original and shuffle remaining tracks
            self.original_queue = self.queue.clone();
            self.original_index = self.queue_index;

            if self.queue.len() > 1 && self.queue_index < self.queue.len() {
                // Keep current track at current position, shuffle the rest after it
                let mut remaining: Vec<BaseItemDto> =
                    self.queue[self.queue_index + 1..].to_vec();
                // Fisher-Yates shuffle on remaining
                for i in (1..remaining.len()).rev() {
                    let j = fastrand::usize(..=i);
                    remaining.swap(i, j);
                }
                self.queue.truncate(self.queue_index + 1);
                self.queue.extend(remaining);
            }
            self.is_shuffled = true;
        }
    }

    pub fn toggle_queue(&mut self) {
        self.show_queue = !self.show_queue;
    }

    pub fn move_queue_track(&mut self, from: usize, to: usize) {
        if from == to || from >= self.queue.len() || to >= self.queue.len() {
            return;
        }
        let track = self.queue.remove(from);
        self.queue.insert(to, track);

        // Adjust queue_index to follow the currently playing track
        if self.queue_index == from {
            self.queue_index = to;
        } else if from < self.queue_index && to >= self.queue_index {
            self.queue_index -= 1;
        } else if from > self.queue_index && to <= self.queue_index {
            self.queue_index += 1;
        }
    }

    pub fn jump_to(&mut self, index: usize) {
        if index < self.queue.len() {
            self.queue_index = index;
            self.current_track = Some(self.queue[index].clone());
            self.is_playing = true;
        }
    }
}

pub fn use_player_state_provider() {
    let api_client = use_api_client();
    use_context_provider(|| Signal::new(PlayerState::new(api_client)));
}

pub fn use_player_state() -> Signal<PlayerState> {
    use_context::<Signal<PlayerState>>()
}
