use dioxus::prelude::*;

#[component]
pub fn Calendar() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Calendar" }
            div { class: "view-sub", "Activity heatmap · click any date to browse · scheduled strategy days — coming in stage 3" }
        }
    }
}
