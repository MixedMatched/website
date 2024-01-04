use dioxus::prelude::*;

#[component]
pub fn Resume(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Resume"
        }
    })
}
