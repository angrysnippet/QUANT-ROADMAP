use dioxus::prelude::*;

#[component]
pub fn Progress() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Progress overview" }
            div { class: "view-sub", "Stat cards, journey, worlds & track breakdown — coming in stage 2" }
        }
    }
}
