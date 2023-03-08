use std::collections::HashMap;

use dioxus::prelude::*;
use serde_json::Value;

use crate::{
    components::{footer::Footer, nav::Navbar},
    pages::_404,
};

#[allow(dead_code)]
const BLOG_REPO: &'static str = "mrxiaozhuox/blog.mrxzx.info";

pub fn BlogList(cx: Scope) -> Element {
    let list = use_future(&cx, (), |_| async move {
        let res = get_blog_list().await;
        let res = if let Some(v) = res { v } else { vec![] };
        res
    });

    match list.value() {
        Some(v) => {
            let list = v.iter().map(|v| {

                let category = v.category.clone().unwrap_or("Default".to_string()); 

                let tags = v.tags.iter().map(|tag| {
                    rsx! {
                        span {
                            class: "text-xs inline-block py-1 px-2.5 leading-none text-center whitespace-nowrap align-baseline font-bold bg-gray-700 text-white rounded",
                            "{tag}"
                        }
                    }
                });

                rsx! {
                    Link {
                        to: "/blog/{v.path}",
                        h1 {
                            class: "text-3xl font-bold text-gray-500 hover:text-gray-600 dark:text-gray-200 dark:hover:text-white",
                            "{v.title}"
                        }
                        p {
                            class: "text-gray-400 dark:text-gray-100",
                            "{v.date} & {category}"
                        }
                        p {
                            class: "mt-2",
                            tags
                        }
                        hr { class: "mt-2" }
                    }
                }
            });
            cx.render(rsx! {
                section {
                    class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
                    Navbar {}
                    div {
                        class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                        div {
                            class: "max-w-5xl text-center",
                            h1 {
                                class: "text-xl font-bold",
                                "\"YuKun's Blog\""
                            }
                            div {
                                class: "mt-6",
                                list
                            }
                            Footer {}
                        }
                    }
                }
            })
        }
        None => cx.render(rsx! {
            section {
                class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
                Navbar {}
                div {
                    class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                    div {
                        class: "max-w-5xl text-center",
                        "Loading..."
                        Footer {}
                    }
                }
            }
        }),
    }
}

#[derive(Debug)]
struct BlogInfo {
    pub title: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub date: String,
    pub path: String,
}

async fn get_blog_list() -> Option<Vec<BlogInfo>> {
    let resp = reqwasm::http::Request::get(&format!(
        "https://api.github.com/repos/{BLOG_REPO}/contents"
    ))
    .send()
    .await
    .ok()?;
    let data = resp.json::<Value>().await.ok()?;

    let mut result = vec![];

    if let Value::Array(vec) = data {
        for value in vec {
            if let Value::Object(obj) = value {
                if obj.get("type")?.as_str()? != "file" {
                    continue;
                }

                let file_name = obj.get("name")?.as_str()?;
                if file_name == "_template.md" {
                    continue;
                }
                let file_path = obj.get("path")?.as_str()?;

                let file_url = obj.get("download_url")?.as_str()?;

                let meta_info = reqwasm::http::Request::get(file_url).send().await.ok()?;
                let meta_info = meta_info.text().await.ok()?;

                let mut type_mark = HashMap::new();

                type_mark.insert("title".into(), "string");
                type_mark.insert("tags".into(), "array");
                type_mark.insert("category".into(), "string");
                type_mark.insert("date".into(), "string");
                type_mark.insert("released".into(), "bool");

                let (meta_info, _) = markdown_meta_parser::MetaData {
                    content: meta_info,
                    required: vec!["title".to_string()],
                    type_mark,
                }
                .parse()
                .ok()?;

                if meta_info.get("released").is_some()
                    && meta_info
                        .get("released")
                        .unwrap()
                        .clone()
                        .as_bool()
                        .unwrap()
                        == false
                {
                    continue;
                }

                let title = meta_info.get("title").unwrap().clone();

                let date = meta_info.get("date");
                let date = if let Some(d) = date {
                    d.clone().as_string().unwrap()
                } else {
                    "".to_string()
                };

                let tags = meta_info.get("tags");
                let tags = if let Some(v) = tags {
                    v.clone().as_array().unwrap()
                } else {
                    vec![]
                };

                let category = meta_info.get("category");
                let category = if let Some(v) = category {
                    v.clone().as_string()
                } else {
                    None
                };

                let title = title.as_string().unwrap();

                let path = file_path.split(".").collect::<Vec<&str>>();
                let path = path[0..path.len() - 1].to_vec();
                let path = path.join(".");

                let blog_info = BlogInfo {
                    title,
                    tags,
                    category,
                    date,
                    path,
                };
                result.push(blog_info);
            }
        }
    }
    Some(result)
}

pub fn BlogPage(cx: Scope) -> Element {
    let route = use_route(&cx);
    let path = route.segment("path").unwrap();

    let name = path.to_string();
    let info = use_future(&cx, (), |_| async move { get_info(&name).await });

    match info.value() {
        Some(Some((info, content))) => {
            let mut options = pulldown_cmark::Options::empty();
            options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
            let parser = pulldown_cmark::Parser::new_ext(content, options);

            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);

            let category = info.category.clone().unwrap_or("Default".to_string());

            let tags = info.tags.iter().map(|tag| {
                rsx! {
                    span {
                        class: "text-xs inline-block py-1 px-2.5 leading-none text-center whitespace-nowrap align-baseline font-bold bg-gray-700 text-white rounded",
                        "{tag}"
                    }
                }
            });

            cx.render(rsx! {
                section {
                    class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
                    Navbar {}
                    div {
                        class: "md:flex h-full w-full justify-center px-6",
                        div {
                            class: "max-w-5xl",
                            h1 {
                                class: "text-4xl font-bold text-gray-600 dark:text-gray-200",
                                "{info.title}"
                            }
                            p {
                                class: "mt-1 text-gray-400 dark:text-gray-300",
                                "{info.date} & {category}"
                            }
                            hr {
                                class: "mt-2 w-60",
                            }
                            div {
                                class: "prose mt-4 dark:text-white",
                                dangerous_inner_html: "{html_output}"
                            }
                            hr {
                                class: "mt-4",
                            }
                            p {
                                class: "mt-4",
                                tags
                            }
                            Footer {}
                        }
                    }
                }
            })
        }
        Some(None) => cx.render(rsx! {
            _404::NotFound {}
        }),
        None => cx.render(rsx! {
            section {
                class: "bg-cover bg-white dark:bg-gray-600 dark:text-white",
                Navbar {}
                div {
                    class: "flex h-full w-full items-center justify-center container mx-auto px-8",
                    div {
                        class: "max-w-5xl text-center",
                        "Loading..."
                        Footer {}
                    }
                }
            }
        }),
    }
}

async fn get_info(name: &str) -> Option<(BlogInfo, String)> {
    let resp = reqwasm::http::Request::get(&format!(
        "https://raw.githubusercontent.com/{BLOG_REPO}/main/{name}.md"
    ))
    .send()
    .await
    .ok()?;

    if resp.status() != 200 {
        return None;
    }

    let content = resp.text().await.ok()?;

    let mut type_mark = HashMap::new();
    type_mark.insert("title".into(), "string");
    type_mark.insert("tags".into(), "array");
    type_mark.insert("category".into(), "string");
    type_mark.insert("date".into(), "string");
    type_mark.insert("released".into(), "bool");

    let (meta_info, content) = markdown_meta_parser::MetaData {
        content,
        required: vec!["title".to_string()],
        type_mark,
    }
    .parse()
    .ok()?;

    if meta_info.get("released").is_some()
        && meta_info
            .get("released")
            .unwrap()
            .clone()
            .as_bool()
            .unwrap()
            == false
    {
        return None;
    }

    let title = meta_info.get("title").unwrap().clone();

    let date = meta_info.get("date");
    let date = if let Some(d) = date {
        d.clone().as_string().unwrap()
    } else {
        "".to_string()
    };

    let tags = meta_info.get("tags");
    let tags = if let Some(v) = tags {
        v.clone().as_array().unwrap()
    } else {
        vec![]
    };

    let category = meta_info.get("category");
    let category = if let Some(v) = category {
        v.clone().as_string()
    } else {
        None
    };

    let title = title.as_string().unwrap();

    let blog_info = BlogInfo {
        title,
        tags,
        category,
        date,
        path: Default::default(),
    };
    Some((blog_info, content))
}
