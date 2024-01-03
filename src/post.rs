use core::fmt;

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

#[derive(Clone, Debug)]
struct Post {
    id: &'static str,
    meta: PostMetaData,
    content: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostQuerySegments {
    join: bool,
    category: Option<String>,
    series: Option<String>,
    author: Option<String>,
}

impl fmt::Display for PostQuerySegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.join {
            write!(f, "join=true")?;
        } else {
            write!(f, "join=false")?;
        }
        if let Some(category) = &self.category {
            write!(f, "&")?;
            write!(f, "category={}", category)?;
        }
        if let Some(series) = &self.series {
            write!(f, "&")?;
            write!(f, "series={}", series)?;
        }
        if let Some(author) = &self.author {
            write!(f, "&")?;
            write!(f, "author={}", author)?;
        }
        Ok(())
    }
}

impl FromQuery for PostQuerySegments {
    fn from_query(query: &str) -> Self {
        let mut join = false;
        let mut category = None;
        let mut series = None;
        let mut author = None;
        let pairs = form_urlencoded::parse(query.as_bytes());
        for (key, value) in pairs {
            match key.as_ref() {
                "join" => join = value == "true",
                "category" => category = Some(value.to_string()),
                "series" => series = Some(value.to_string()),
                "author" => author = Some(value.to_string()),
                _ => {}
            }
        }
        Self {
            join,
            category,
            series,
            author,
        }
    }
}

// invariant: no two posts have the same id
const POST_LIST: &[Post] = &[
    Post {
        id: "post-1",
        meta: PostMetaData {
            title: "Post 1",
            author: "Author 1",
            published: date!(2021 - 01 - 01),
            category: Some("Garbage"),
            series: None,
            part: None,
            description: None,
        },
        content: include_str!("../assets/post-1.md"),
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
            description: Some("This is the second test post"),
        },
        content: include_str!("../assets/post-2.md"),
    },
    Post {
        id: "post-3",
        meta: PostMetaData {
            title: "Post 3",
            author: "Author 2",
            published: date!(2021 - 01 - 03),
            category: Some("Garbage"),
            series: Some("Test Series"),
            part: Some(1),
            description: None,
        },
        content: "blank",
    },
    Post {
        id: "post-4",
        meta: PostMetaData {
            title: "Post 4",
            author: "Author 1",
            published: date!(2021 - 01 - 04),
            category: Some("Trash"),
            series: Some("Test Series"),
            part: Some(2),
            description: Some("This is the second post in the test series"),
        },
        content: "blank",
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
                class: "text-3xl dark:text-white font-bold mb-4",
                "Blog"
            }
            if POST_LIST.is_empty() {
                rsx! {
                    p {
                        class: "dark:text-white mb-4 bg-gray-300 dark:bg-gray-700 p-4",
                        "No posts here yet :p"
                    }
                }
            }
            for post in POST_LIST {
                p {
                    class: "dark:text-white mb-4 bg-gray-300 dark:bg-gray-700 p-4",
                    p {
                        class: "text-xl text-blue-400 dark:text-orange-600",
                        Link { 
                            to: Route::Post { id: post.id.to_string() },
                            u {
                                post.meta.title
                            }
                        }
                    }
                    p {
                        class: if post.meta.description.is_some() {
                            "mb-4"
                        } else {
                            ""
                        },
                        "{post.meta.published} • "
                        Link {
                            class: "text-blue-400 dark:text-orange-600",
                            to: Route::PostQuery {
                                query_params: PostQuerySegments {
                                    join: true,
                                    category: None,
                                    series: None,
                                    author: Some(post.meta.author.to_string()),
                                }
                            },
                            post.meta.author
                        }
                        if let Some(cat) = post.meta.category {
                            rsx! {
                                " • "
                                Link {
                                    class: "text-blue-400 dark:text-orange-600",
                                    to: Route::PostQuery {
                                        query_params: PostQuerySegments {
                                            join: true,
                                            category: Some(cat.to_string()),
                                            series: None,
                                            author: None,
                                        }
                                    },
                                    cat
                                }
                            }
                        }
                    }
                    if let Some(desc) = post.meta.description {
                        rsx! {
                            desc
                            " "
                            Link {
                                class: "text-blue-400 dark:text-orange-600",
                                to: Route::Post { id: post.id.to_string() },
                                u {
                                    ">>"
                                },
                            }
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
        .find(|post| post.id == id);

    if let Some(post) = post {
        cx.render(rsx! {
            div {
                class: "bg-white pattern-light dark:text-white dark:bg-gray-800 p-8",
                script {
                    "hljs.highlightAll();"
                }
                p {
                    class: "dark:text-white mb-2",
                    Link {
                        class: "text-blue-400 dark:text-orange-600",
                        to: Route::PostList {},
                        u {
                            "<< Back to Blog"
                        }
                    }
                }
                h1 {
                    class: "text-4xl dark:text-white font-bold mb-2",
                    post.meta.title
                }
                p {
                    class: "dark:text-white mb-4",
                    "{post.meta.published} • "
                    Link {
                        class: "text-blue-400 dark:text-orange-600",
                        to: Route::PostQuery {
                            query_params: PostQuerySegments {
                                join: true,
                                category: None,
                                series: None,
                                author: Some(post.meta.author.to_string()),
                            }
                        },
                        post.meta.author
                    }
                    if let Some(cat) = post.meta.category {
                        rsx! {
                            " • "
                            Link {
                                class: "text-blue-400 dark:text-orange-600",
                                to: Route::PostQuery {
                                    query_params: PostQuerySegments {
                                        join: true,
                                        category: Some(cat.to_string()),
                                        series: None,
                                        author: None,
                                    }
                                },
                                cat
                            }
                        }
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
    } else {
        cx.render(rsx! {
            div {
                class: "bg-gray-200 dark:bg-gray-800 p-8",
                p {
                    class: "dark:text-white",
                    Link {
                        class: "text-blue-400 dark:text-orange-600",
                        to: Route::PostList {},
                        u {
                            "<< Back to Blog"
                        }
                    }
                }
                h1 {
                    class: "text-3xl dark:text-white font-bold mb-4",
                    "Post not found"
                }
                p {
                    class: "dark:text-white",
                    "The post you are looking for does not exist."
                    
                }
            }
        })
    }
}

#[component]
pub fn PostQuery(cx: Scope, query_params: PostQuerySegments) -> Element {
    let mut posts = POST_LIST.to_vec();
    if query_params.join {
        posts = posts
            .into_iter()
            .filter(|post| {
                if let Some(category) = &query_params.category {
                    if post.meta.category != Some(category.as_str()) {
                        return false;
                    }
                }
                if let Some(series) = &query_params.series {
                    if post.meta.series != Some(series.as_str()) {
                        return false;
                    }
                }
                if let Some(author) = &query_params.author {
                    if post.meta.author != author.as_str() {
                        return false;
                    }
                }
                true
            })
            .collect();
    } else {
        posts = posts
            .into_iter()
            .filter(|post| {
                if let Some(category) = &query_params.category {
                    if post.meta.category == Some(category.as_str()) {
                        return true;
                    }
                }
                if let Some(series) = &query_params.series {
                    if post.meta.series == Some(series.as_str()) {
                        return true;
                    }
                }
                if let Some(author) = &query_params.author {
                    if post.meta.author == author.as_str() {
                        return true;
                    }
                }
                false
            })
            .collect();
    }

    render!{
        div {
            class: "bg-gray-200 dark:bg-gray-800 p-8",
            Link {
                to: Route::PostList {},
                class: "text-blue-400 dark:text-orange-600 mb-2",
                u {
                    "<< Back to Blog"
                }
            }
            h1 {
                class: "text-3xl dark:text-white font-bold mb-4",
                "Search"
            }
            if posts.is_empty() {
                rsx! {
                    p {
                        class: "dark:text-white mb-4 bg-gray-300 dark:bg-gray-700 p-4",
                        "No posts found. Try broadening your search."
                    }
                }
            }
            for post in posts {
                p {
                    class: "dark:text-white mb-4 bg-gray-300 dark:bg-gray-700 p-4",
                    p {
                        class: "text-xl text-blue-400 dark:text-orange-600",
                        Link { 
                            to: Route::Post { id: post.id.to_string() },
                            u {
                                post.meta.title
                            }
                        }
                    }
                    p {
                        class: if post.meta.description.is_some() {
                            "mb-4"
                        } else {
                            ""
                        },
                        "{post.meta.published} • "
                        Link {
                            class: "text-blue-400 dark:text-orange-600",
                            to: Route::PostQuery {
                                query_params: PostQuerySegments {
                                    join: query_params.join,
                                    category: query_params.category.clone(),
                                    series: query_params.series.clone(),
                                    author: Some(post.meta.author.to_string()),
                                }
                            },
                            post.meta.author
                        }
                        if let Some(cat) = post.meta.category {
                            rsx! {
                                " • "
                                Link {
                                    class: "text-blue-400 dark:text-orange-600",
                                    to: Route::PostQuery {
                                        query_params: PostQuerySegments {
                                            join: query_params.join,
                                            category: Some(cat.to_string()),
                                            series: query_params.series.clone(),
                                            author: query_params.author.clone(),
                                        }
                                    },
                                    cat
                                }
                            }
                        }
                    }
                    if let Some(desc) = post.meta.description {
                        rsx! {
                            desc
                            " "
                            Link {
                                class: "text-blue-400 dark:text-orange-600",
                                to: Route::Post { id: post.id.to_string() },
                                u {
                                    ">>"
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}