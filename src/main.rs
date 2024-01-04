mod about;
mod home;
mod markdown;
mod post;
mod resume;

use crate::about::About;
use crate::home::Home;
use crate::post::{Blog, Post, PostList, PostQuery};
use crate::resume::Resume;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use post::PostQuerySegments;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/resume")]
        Resume {},
        #[nest("/blog")]
            #[layout(Blog)]
            #[route("/")]
            PostList {},
            #[route("/:id")]
            Post { id: String },
            #[end_layout]
        #[end_nest]
        #[route("/search/?:query_params")]
        PostQuery { query_params: PostQuerySegments },
}

fn main() {
    LaunchBuilder::new(App).launch();
}

#[component]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col min-h-screen bg-gray-100 dark:bg-gray-900 dark:text-white",
            div {
                class: "flex-grow container mx-auto px-4 dark:text-white",
                Router::<Route> {}
            }
            footer {
                class: "bg-gray-100 dark:bg-gray-900 p-8",
            }
        }
    })
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
                to: Route::Resume {},
                "Resume"
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
