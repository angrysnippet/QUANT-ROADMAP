use dioxus::prelude::*;

#[component]
pub fn Practice() -> Element {
    rsx! {
        div { class: "view active",
            div { class: "view-title", "Practice" }
            div { class: "view-sub", "Block-by-block drills + in-browser C++/Python compiler — coming in the final stage" }
        }
    }
}
