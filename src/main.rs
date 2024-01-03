mod post;
mod markdown;

use crate::post::{Blog, PostList, Post};

use dioxus::{prelude::*, html::footer};
use dioxus_router::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    LaunchBuilder::new(App).launch();
}

#[component]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100 dark:bg-slate-900 dark:text-white",
            script {
                "hljs.highlightAll();"
            }
            div {
                class: "flex-grow container mx-auto px-4 dark:text-white",
                Router::<Route> {}
            }
            footer {
                class: "bg-gray-100 dark:bg-slate-900 p-8",
            }
        }
    })
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/contact")]
        Contact {},
        #[nest("/blog")]
            #[layout(Blog)]
            #[route("/")]
            PostList {},
            #[route("/:id")]
            Post { id: String },
}

#[component]
fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-end py-4",
            Link { 
                class: "text-gray-700 mx-2 dark:text-slate-200",
                to: Route::Home {},
                "Home"
            }
            Link { 
                class: "text-gray-700 mx-2 dark:text-slate-200",
                to: Route::About {},
                "About"
            }
            Link { 
                class: "text-gray-700 mx-2 dark:text-slate-200",
                to: Route::Contact {},
                "Contact"
            }
            Link { 
                class: "text-gray-700 mx-2 dark:text-slate-200",
                to: Route::PostList {},
                "Blog"
            }
        }
        Outlet::<Route> {}
    })
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, World!"
        }
    })
}

#[component]
fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "About"
        }
    })
}

#[component]
fn Contact(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Contact"
        }
    })
}

