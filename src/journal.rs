use dioxus::prelude::*;

#[component]
pub fn Journal() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Reflection journal" }
            div { class: "view-sub", "Five daily reflection prompts — coming in stage 4" }
        }
    }
}
