use dioxus::prelude::*;
use kinetic_ui::{KBadge, KBadgeVariant};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct ModInfo {
    pub name: String,
    pub side: String,
    pub url: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct ModPackInfo {
    pub mods: Vec<ModInfo>,
    pub zips: Vec<String>,
}

#[component]
pub fn Modpack() -> Element {
    let info = use_resource(|| async move {
        let modpack_json = asset!("/assets/modpack_generated.json");
        let base_url = if cfg!(debug_assertions) {
            "http://localhost:8080"
        } else {
            "https://packwiz.toyvo.dev"
        };
        let url = format!("{base_url}{}", modpack_json.resolve().to_str().unwrap());
        let response = reqwest::get(url).await.unwrap().error_for_status().unwrap();
        response.json::<ModPackInfo>().await.unwrap()
    });

    rsx! {
        div {
            margin: "20px",
            div {
                display: "flex",
                justify_content: "space-between",
                align_items: "center",
                "Modpack info"
            }
            {
                let zips = info.cloned().unwrap_or_default().zips;
                if !zips.is_empty() {
                    rsx! {
                        div { "Import the appropriate zip file into prism and packwiz will take care of the rest" }
                        div { "Files hosted "
                            ul {
                                for zip in zips {
                                    li {
                                        a {
                                            href: "{zip}",
                                            "{zip}"
                                        }
                                    }
                                }
                            }
                        }
                        img {
                            max_height: "512px",
                            max_width: "100%",
                            height: "auto",
                            src: asset!("/assets/prism-import.png"),
                            alt: "import prism instance"
                        }
                    }
                } else {
                    rsx! {}
                }
            }
            div {
                "To do this your self, download the packwiz bootstrap jar from "
                a {
                    href: "https://github.com/packwiz/packwiz-installer-bootstrap/releases",
                    "Github Releases"
                }
                " and place it within the minecraft directory of a newly created prism instance."
            }
            div {"Go to Edit Instance -> Settings -> Custom commands, then check the Custom Commands box and paste the following command into the pre-launch command field:"}
            div {
                font_family: "monospace",
                font_size: "14px",
                background:"#666",
                padding:"10px",
                "\"$INST_JAVA\" -jar packwiz-installer-bootstrap.jar https://packwiz.toyvo.dev/pack.toml"
            }
            img {
                max_height: "512px",
                max_width: "100%",
                height: "auto",
                src: asset!("/assets/prism-settings.png"),
                alt: "Setup packwiz"
            }
            div {
                "Mods included: "
                div {
                    display: "grid",
                    width: "100%",
                    grid_template_columns: "1fr 1fr auto auto",
                    gap: "12px",
                    div {
                        "Mod Name"
                    }
                    div {
                        "Side"
                    }
                    div {
                        "Minecraft Versions"
                    }
                    div {
                        "Loader"
                    }
                    for item in info.cloned().unwrap_or_default().mods {
                        a {
                            href: "{item.url}",
                            "{item.name}"
                        }
                        div {
                            "{item.side}"
                        }
                        div {
                            display: "flex",
                            flex_flow: "row wrap",
                            align_content: "start",
                            gap: "4px",
                            for version in &item.game_versions {
                                KBadge { variant: KBadgeVariant::Primary, "{version}" }
                            }
                        }
                        div {
                            display: "flex",
                            flex_flow: "row wrap",
                            align_content: "start",
                            gap: "4px",
                            for loader in &item.loaders {
                                KBadge { variant: KBadgeVariant::Primary, "{loader}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
