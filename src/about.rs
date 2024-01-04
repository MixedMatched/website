use dioxus::prelude::*;

#[component]
pub fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "About"
        }
    })
}
