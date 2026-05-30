use dioxus::prelude::*;

#[component]
pub fn Today() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Today's routine" }
            div { class: "view-sub", "Daily blocks with auto-linked progress — coming in stage 4" }
        }
    }
}
