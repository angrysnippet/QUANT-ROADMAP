use dioxus::prelude::*;

#[component]
pub fn Strategy() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Day-by-day strategy" }
            div { class: "view-sub", "Full daily breakdown — resources, tasks, practice problems — coming in stage 4" }
        }
    }
}
