//! Embedded AI study planner. The planner stays independently deployable so its
//! React client and Express API can evolve without coupling them to this SPA.

use dioxus::prelude::*;

const DEFAULT_PLANNER_URL: &str = "http://localhost:5173";

fn planner_url() -> &'static str {
    option_env!("AI_STUDY_PLATFORM_URL").unwrap_or(DEFAULT_PLANNER_URL)
}

#[component]
pub fn Planner() -> Element {
    let url = planner_url();

    rsx! {
        div { class: "planner-view",
            div { class: "planner-header",
                div {
                    div { class: "view-title", "AI study planner" }
                    div {
                        class: "view-sub",
                        "Generate a course routine, save plans, and track completion."
                    }
                }
                a {
                    class: "planner-open-btn",
                    href: url,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Open full page ↗"
                }
            }

            div { class: "planner-frame-shell",
                iframe {
                    class: "planner-frame",
                    src: url,
                    title: "AI study planner",
                    allow: "clipboard-write",
                }
            }

            div { class: "planner-help",
                "If the embedded planner does not load, start its client locally or use “Open full page”. "
                "For production, build Quant Roadmap with "
                code { "AI_STUDY_PLATFORM_URL=https://your-planner.vercel.app" }
                "."
            }
        }
    }
}
