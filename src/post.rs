use dioxus::prelude::*;
use dioxus_router::prelude::*;
use time::macros::date;

use crate::{Route, markdown::Markdown};

#[derive(Clone, Debug)]
struct PostMetaData {
    title: &'static str,
    author: &'static str,
    published: time::Date,
    category: Option<&'static str>,
    series: Option<&'static str>,
    part: Option<u32>,
    description: Option<&'static str>,
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
            published: date!(2021 - 01 - 01),
            category: None,
            series: None,
            part: None,
            description: None,
        },
        content: include_str!("../public/post-1.md"),
    },
    Post {
        id: "post-2",
        meta: PostMetaData {
            title: "Post 2",
            author: "Author 2",
            published: date!(2021 - 01 - 02),
            category: None,
            series: None,
            part: None,
            description: None,
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
            class: "bg-gray-200 dark:bg-gray-800 p-8",
            h1 {
                class: "text-3xl dark:text-white font-bold mb-2",
                "Blog"
            }
            for post in POST_LIST {
                p {
                    class: "dark:text-white mb-4 bg-gray-300 dark:bg-gray-700 p-4",
                    p {
                        class: "text-xl dark:text-white mb-4",
                        Link { to: Route::Post { id: post.id.to_string() }, post.meta.title }
                    }
                    p {
                        "{post.meta.published} • "
                        "{post.meta.author}"
                        if let Some(category) = post.meta.category {
                            " • {category}"
                        }
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
            class: "bg-white pattern-light dark:text-white dark:bg-gray-800 p-8",
            script {
                "hljs.highlightAll();"
            }
            h1 {
                class: "text-4xl dark:text-white font-bold mb-2",
                post.meta.title
            }
            p {
                class: "dark:text-white mb-4",
                "{post.meta.published} • "
                "{post.meta.author}"
                if let Some(category) = post.meta.category {
                    " • {category}"
                }
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