use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[derive(Props, Clone, PartialEq, Debug, Copy)]
pub struct MarkdownProps {
    pub content: &'static str,
}

#[allow(non_snake_case)]
pub fn Markdown(cx: Scope<MarkdownProps>) -> Element {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(cx.props.content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // add tailwind class to p and table
    let html_output = html_output
        .replace("<p>", "<p class=\"mb-4 dark:text-white\">")
        .replace(
            "<table>",
            "<table class=\"mb-4 gray-400 dark:text-white dark:gray-800\">",
        );

    // add tailwind class to h<4-n>
    let html_output = html_output
        .replace(
            "<h1>",
            "<h1 class=\"text-4xl font-bold mb-2 dark:text-white\">",
        )
        .replace(
            "<h2>",
            "<h2 class=\"text-3xl font-bold mb-2 dark:text-white\">",
        )
        .replace(
            "<h3>",
            "<h3 class=\"text-2xl font-bold mb-2 dark:text-white\">",
        )
        .replace(
            "<h4>",
            "<h4 class=\"text-xl font-bold mb-2 dark:text-white\">",
        )
        .replace(
            "<h5>",
            "<h5 class=\"text-lg font-bold mb-2 dark:text-white\">",
        )
        .replace(
            "<h6>",
            "<h6 class=\"text-base font-bold mb-2 dark:text-white\">",
        );

    // add tailwind to code blocks
    let html_output = html_output.replace(
        "<pre>",
        "<pre class=\"mb-4 bg-gray-200 dark:text-white dark:bg-gray-900\">",
    );

    // add tailwind to td and th
    let html_output = html_output
        .replace("<th>", "<th class=\"bg-gray-300 dark:bg-gray-900\">")
        .replace("<td>", "<th class=\"bg-gray-100 dark:bg-gray-700\">");

    // add tailwind to links
    let html_output =
        html_output.replace("<a ", "<a class=\"text-blue-400 dark:text-orange-600\" ");

    cx.render(rsx! {
        div {
            dangerous_inner_html: "{html_output}"
        }
    })
}
