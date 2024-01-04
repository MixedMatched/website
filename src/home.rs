use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, World!"
        }
    })
}
