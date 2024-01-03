use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{Route, markdown::Markdown};

#[derive(Clone, Debug)]
struct PostMetaData {
    title: &'static str,
    author: &'static str,
    series: Option<String>,
    part: Option<u32>,
}

struct Post {
    id: &'static str,
    meta: PostMetaData,
    content: &'static str,
}

// invariant: no two posts have the same id
const POST_LIST: &[Post] = &[
    Post {
        id: "post-1",
        meta: PostMetaData {
            title: "Post 1",
            author: "Author 1",
            series: None,
            part: None,
        },
        content: include_str!("../public/post-1.md"),
    },
    Post {
        id: "post-2",
        meta: PostMetaData {
            title: "Post 2",
            author: "Author 2",
            series: None,
            part: None,
        },
        content: include_str!("../public/post-2.md"),
    },
];

#[component]
pub fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        Outlet::<Route> {}
    })
}

#[component]
pub fn PostList(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "bg-gray-200 dark:bg-slate-800 p-8",
            h1 {
                class: "text-3xl dark:text-white font-bold mb-2",
                "Blog"
            }
            for post in POST_LIST {
                p {
                    class: "dark:text-white mb-4 bg-gray-300 dark:bg-slate-700 p-4",
                    p {
                        class: "text-xl dark:text-white mb-4",
                        Link { to: Route::Post { id: post.id.to_string() }, post.meta.title }
                    }
                    p {
                        "by {post.meta.author}"
                    }
                }
            }
        }
    })
}

#[component]
pub fn Post(cx: Scope, id: String) -> Element {
    let post = POST_LIST
        .iter()
        .find(|post| post.id == id)
        .expect("Post not found");

    cx.render(rsx! {
        div {
            class: "bg-white pattern-light dark:text-white dark:bg-slate-800 p-8",
            script {
                "hljs.highlightAll();"
            }
            h1 {
                class: "text-4xl dark:text-white font-bold mb-2",
                post.meta.title
            }
            p {
                class: "text-xl dark:text-white mb-4",
                "by {post.meta.author}"
            }
            p {
                class: "mb-4 dark:text-white",
                Markdown {
                    content: post.content
                }
            }
        }
    })
}