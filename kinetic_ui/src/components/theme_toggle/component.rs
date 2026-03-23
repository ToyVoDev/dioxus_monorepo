use dioxus::prelude::*;

use crate::IconButton;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

/// Inline script that runs on page load to read the theme cookie and
/// apply CSS vars immediately (no flash). Only runs the theme application
/// once via a global guard.
const INIT_SCRIPT: &str = r#"
if (!window.__kineticThemeApplied) {
    window.__kineticThemeApplied = true;
    var m = document.cookie.match(/(?:^|; )kinetic-theme=([^;]*)/);
    var t = m ? m[1] : 'system';
    document.documentElement.setAttribute('data-kinetic-theme', t);
    if (t === 'light') {
        document.documentElement.style.setProperty('--dark', ' ');
        document.documentElement.style.setProperty('--light', 'initial');
    } else if (t === 'dark') {
        document.documentElement.style.setProperty('--dark', 'initial');
        document.documentElement.style.setProperty('--light', ' ');
    }
}
"#;

fn theme_key(mode: ThemeMode) -> &'static str {
    match mode {
        ThemeMode::System => "system",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    }
}

fn apply_theme(mode: ThemeMode) {
    let key = theme_key(mode);
    let css_js = match mode {
        ThemeMode::System => {
            "document.documentElement.style.removeProperty('--dark'); document.documentElement.style.removeProperty('--light');"
        }
        ThemeMode::Light => {
            "document.documentElement.style.setProperty('--dark', ' '); document.documentElement.style.setProperty('--light', 'initial');"
        }
        ThemeMode::Dark => {
            "document.documentElement.style.setProperty('--dark', 'initial'); document.documentElement.style.setProperty('--light', ' ');"
        }
    };
    document::eval(css_js);
    document::eval(&format!(
        "document.cookie = 'kinetic-theme={key}; max-age=31536000; path=/; SameSite=Lax'; document.documentElement.setAttribute('data-kinetic-theme', '{key}');"
    ));
}

#[component]
pub fn ThemeToggle() -> Element {
    // On mount: use dioxus.send() to push the stored theme value from JS to Rust
    let mut theme_mode = use_signal(|| ThemeMode::System);

    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                "dioxus.send(document.documentElement.getAttribute('data-kinetic-theme') || 'system');",
            );
            if let Ok(val) = eval.recv::<String>().await {
                let mode = match val.as_str() {
                    "light" => ThemeMode::Light,
                    "dark" => ThemeMode::Dark,
                    _ => ThemeMode::System,
                };
                theme_mode.set(mode);
            }
        });
    });

    rsx! {
        script { dangerous_inner_html: INIT_SCRIPT }

        IconButton {
            onclick: move |_| {
                let next = match theme_mode() {
                    ThemeMode::System => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::System,
                };
                theme_mode.set(next);
                apply_theme(next);
            },
            match theme_mode() {
                ThemeMode::System => rsx! {
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M9 18h6" }
                        path { d: "M10 22h4" }
                        path { d: "M15.09 14c.18-.98.65-1.74 1.41-2.5A4.65 4.65 0 0018 8 6 6 0 006 8c0 1 .23 2.23 1.5 3.5A4.61 4.61 0 018.91 14" }
                    }
                },
                ThemeMode::Light => rsx! {
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "12", cy: "12", r: "5" }
                        path { d: "M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" }
                    }
                },
                ThemeMode::Dark => rsx! {
                    svg { xmlns: "http://www.w3.org/2000/svg", width: "18", height: "18", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        path { d: "M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z" }
                    }
                },
            }
        }
    }
}
