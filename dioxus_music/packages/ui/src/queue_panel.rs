use crate::player_state::use_player_state;
use dioxus::prelude::*;

const QUEUE_PANEL_CSS: Asset = asset!("/assets/styling/queue-panel.css");

#[component]
pub fn QueuePanel() -> Element {
    let mut player = use_player_state();
    let show_queue = player.read().show_queue;
    let queue = player.read().queue.clone();
    let queue_index = player.read().queue_index;
    let has_track = player.read().current_track.is_some();

    let mut dragging_index: Signal<Option<usize>> = use_signal(|| None);
    let mut drag_over_index: Signal<Option<usize>> = use_signal(|| None);

    if !show_queue {
        return rsx! {
            document::Link { rel: "stylesheet", href: QUEUE_PANEL_CSS }
        };
    }

    rsx! {
        document::Link { rel: "stylesheet", href: QUEUE_PANEL_CSS }

        div { class: "queue-panel",
            div { class: "queue-panel__header",
                span { class: "queue-panel__title", "Queue" }
                span { class: "queue-panel__count", "{queue.len()} tracks" }
            }

            div { class: "queue-panel__list",
                for (i, track) in queue.iter().enumerate() {
                    {
                        let is_current = has_track && i == queue_index;
                        let is_drag_over = drag_over_index() == Some(i);
                        let track_id = track.id;
                        let track_title = track.name.clone();
                        let track_artist = track
                            .artists
                            .as_ref()
                            .and_then(|a| a.first())
                            .cloned()
                            .unwrap_or_default();

                        let class_name = if is_current && is_drag_over {
                            "queue-panel__item queue-panel__item--active queue-panel__item--drag-over"
                        } else if is_current {
                            "queue-panel__item queue-panel__item--active"
                        } else if is_drag_over {
                            "queue-panel__item queue-panel__item--drag-over"
                        } else {
                            "queue-panel__item"
                        };

                        rsx! {
                            div {
                                key: "{track_id}-{i}",
                                class: class_name,
                                draggable: true,
                                ondragstart: move |_| {
                                    dragging_index.set(Some(i));
                                },
                                ondragover: move |evt: Event<DragData>| {
                                    evt.prevent_default();
                                    drag_over_index.set(Some(i));
                                },
                                ondragleave: move |_| {
                                    if drag_over_index() == Some(i) {
                                        drag_over_index.set(None);
                                    }
                                },
                                ondrop: move |evt: Event<DragData>| {
                                    evt.prevent_default();
                                    if let Some(from) = dragging_index() {
                                        player.with_mut(|p| p.move_queue_track(from, i));
                                    }
                                    dragging_index.set(None);
                                    drag_over_index.set(None);
                                },
                                ondragend: move |_| {
                                    dragging_index.set(None);
                                    drag_over_index.set(None);
                                },
                                onclick: move |_| {
                                    player.with_mut(|p| p.jump_to(i));
                                    let _ = document::eval(r#"
                                        let a = document.getElementById('main-audio');
                                        if (a) { a.load(); a.play(); }
                                    "#);
                                },

                                span { class: "queue-panel__item-index", "{i + 1}" }
                                div { class: "queue-panel__item-info",
                                    div { class: "queue-panel__item-title", "{track_title}" }
                                    div { class: "queue-panel__item-artist", "{track_artist}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
