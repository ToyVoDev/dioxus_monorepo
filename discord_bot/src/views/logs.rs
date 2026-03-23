use {
    chrono::prelude::*,
    dioxus::prelude::*,
    kinetic_ui::{
        KButton, KButtonVariant, KInput, KSelect, KSelectList, KSelectOption, KSelectTrigger,
        KSelectValue,
    },
    std::time::Duration,
};

const VALID_SERVICES: [&str; 3] = [
    "arion-minecraft-modded.service",
    "arion-minecraft-geyser.service",
    "arion-terraria.service",
];

#[component]
pub fn Logs() -> Element {
    let mut logs = use_signal(String::new);
    let mut unit = use_signal(|| String::from("arion-minecraft-modded.service"));
    let unit_value: Memo<Option<Option<String>>> = use_memo(move || Some(Some(unit())));
    let now = Utc::now();
    let since = now - Duration::from_secs(3600);
    let until = now;
    rsx! {
        form {
            label { "Unit" }
            KSelect::<String> {
                value: unit_value,
                on_value_change: move |val: Option<String>| {
                    if let Some(v) = val {
                        unit.set(v);
                    }
                },
                KSelectTrigger {
                    KSelectValue {}
                }
                KSelectList {
                    for (i, service) in VALID_SERVICES.iter().enumerate() {
                        KSelectOption::<String> {
                            value: service.to_string(),
                            index: i,
                            text_value: service.to_string(),
                            "{service}"
                        }
                    }
                }
            }
            label { "Since" }
            KInput {
                r#type: "datetime-local".to_string(),
                value: since.format("%Y-%m-%dT%H:%M:%S").to_string(),
            }
            label { "Until" }
            KInput {
                r#type: "datetime-local".to_string(),
                value: until.format("%Y-%m-%dT%H:%M:%S").to_string(),
            }
            small {
                "Note: time shown in UTC"
            }
            KButton {
                variant: KButtonVariant::Primary,
                onclick: move |_| async move {
                    if let Ok(new_logs) = fetch_logs(unit.to_string(), since.to_rfc3339(), until.to_rfc3339()).await {
                        logs.set(new_logs);
                    }
                },
                "Fetch Logs"
            }
        }
        pre {
            margin: 0,
            {logs}
        }
    }
}

#[server]
async fn fetch_logs(unit: String, since: String, until: String) -> Result<String, ServerFnError> {
    if !VALID_SERVICES.contains(&unit.as_str()) {
        return Err(ServerFnError::Args(String::from("invalid unit")));
    }
    let journalctl_args = [
        "--utc",
        "-u",
        unit.as_str(),
        "-S",
        since.as_str(),
        "-U",
        until.as_str(),
    ];
    tracing::info!("fetching logs for unit: {unit}");
    Ok(journalctl_args.join(" "))
}
