use crate::Config;
use crate::build::{BlogrollEntry, Page, Post, tag_slug};
use chrono::Datelike;
use maud::{DOCTYPE, Markup, PreEscaped, html};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub enum OgType {
    Website,
    Article,
}

impl OgType {
    fn as_str(self) -> &'static str {
        match self {
            OgType::Website => "website",
            OgType::Article => "article",
        }
    }
}

pub struct Meta<'a> {
    pub title: &'a str,
    pub path: &'a str,
    pub description: Option<&'a str>,
    pub og_type: OgType,
    pub published_time: Option<&'a str>,
}

pub fn base(config: &Config, meta: Meta, body: Markup) -> Markup {
    let full_title = if meta.title.is_empty() {
        config.title.clone()
    } else {
        format!("{} - {}", meta.title, config.title)
    };
    let description = meta.description.unwrap_or(&config.description);
    let canonical = format!("{}{}", config.base_url, meta.path);
    let og_image = format!("{}{}", config.base_url, config.og_image);
    let published_iso = meta.published_time.map(|d| format!("{d}T00:00:00Z"));
    let year = chrono::Utc::now().year();

    html! {
        (DOCTYPE)
        html lang=(config.language) {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="theme-color" content=(config.theme_color);
                meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
                title { (full_title) }
                meta name="description" content=(description);
                meta name="author" content=(config.author);
                link rel="canonical" href=(canonical);
                link href="/css/styles.css" rel="stylesheet";
                link rel="alternate" type="application/atom+xml" href="/atom.xml" title=(config.author);
                @for url in &config.rel_me {
                    link rel="me" href=(url);
                }

                meta property="og:title" content=(full_title);
                meta property="og:description" content=(description);
                meta property="og:image" content=(og_image);
                meta property="og:url" content=(canonical);
                meta property="og:type" content=(meta.og_type.as_str());
                meta property="og:site_name" content=(config.title);
                @if let Some(pt) = &published_iso {
                    meta property="article:published_time" content=(pt);
                }

                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:site" content=(config.twitter_handle);
                meta name="twitter:creator" content=(config.twitter_handle);
                meta name="twitter:title" content=(full_title);
                meta name="twitter:description" content=(description);
                meta name="twitter:image" content=(og_image);

                @if let Some(a) = &config.analytics {
                    script defer src=(a.umami_url) data-website-id=(a.umami_id) {}
                }
            }
            body {
                nav {
                    a href="/" { strong { (config.title) } }
                    span.nav-sep {}
                    a href="/posts/" { "Posts" }
                    a href="/about/" { "About" }
                }
                main { (body) }
                footer {
                    ul.footer-links {
                        li { a href="/blogroll/" { "Blogroll" } }
                        li { a href="/meta/" { "Meta" } }
                        li { a href="/bookmarks/" { "Bookmarks" } }
                        li { a href="/tags/" { "Tags" } }
                        li { a href="/atom.xml" { "Feed" } }
                    }
                    a.h-card href=(config.base_url) rel="me" { (config.author) }
                    " "
                    span.symbol { (PreEscaped("&copy;")) }
                    " " (year) " — "
                    a target="_blank" href=(config.license.url) {
                        (config.license.name)
                    }
                }
                script { (PreEscaped(LIVE_RELOAD_JS)) }
            }
        }
    }
}

pub fn home(config: &Config, posts: &[&Post]) -> Markup {
    let meta = Meta {
        title: "",
        path: "/",
        description: None,
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            section.intro {
                p {
                    "Hi, I'm "
                    a.person.thomas href="/about/" { "Thomas" }
                    ", and this is my "
                    a href="https://indieweb.org/blog" { "weblog" }
                    ". I write about software and making things."
                }
                p {
                    "You can browse my posts by "
                    a href="/tags/" { "tags" }
                    ", check out my "
                    a href="/blogroll/" { "blogroll" }
                    ", see what I'm "
                    a href=(config.bookmarks_url) { "bookmarking" }
                    ", or subscribe to my "
                    a href="/atom.xml" { "posts feed" }
                    "."
                }
                div.h-card hidden {
                    a.p-name.u-url href=(config.base_url) rel="me" { (config.author) }
                    img.u-photo src=(config.profile_image) alt=(config.author);
                    p.p-note { "I write about software and making things" }
                }
            }
            @if !posts.is_empty() {
                section.h-feed aria-labelledby="posts-heading" {
                    h2 #posts-heading .title.p-name { "Recent Posts" }
                    ul.list {
                        @for post in posts.iter().take(5) {
                            li.h-entry {
                                a.p-name.u-url href={ "/posts/" (post.slug) "/" } { (post.meta.title) }
                                " "
                                time.dt-published datetime=(post.meta.date) { (format_date(&post.meta.date)) }
                            }
                        }
                    }
                    p { a href="/posts/" { "View all" } }
                }
            }
        },
    )
}

pub fn posts_index(config: &Config, posts: &[&Post]) -> Markup {
    let meta = Meta {
        title: "Posts",
        path: "/posts/",
        description: None,
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            section.h-feed aria-labelledby="posts-heading" {
                h2 #posts-heading .title.p-name { "Posts" }
                ul.list {
                    @for post in posts {
                        li.h-entry {
                            a.p-name.u-url href={ "/posts/" (post.slug) "/" } { (post.meta.title) }
                            " "
                            time.dt-published datetime=(post.meta.date) {
                                (format_date(&post.meta.date))
                            }
                        }
                    }
                }
            }
        },
    )
}

pub fn post(config: &Config, post: &Post, newer: Option<&Post>, older: Option<&Post>) -> Markup {
    let minutes = reading_minutes(post.word_count);
    let path = format!("/posts/{}/", post.slug);
    let meta = Meta {
        title: &post.meta.title,
        path: &path,
        description: post.meta.description.as_deref(),
        og_type: OgType::Article,
        published_time: Some(&post.meta.date),
    };
    base(
        config,
        meta,
        html! {
            article.h-entry {
                h2.title.p-name { (post.meta.title) }
                p.subtitle {
                    time.dt-published datetime=(post.meta.date) {
                        (format_date(&post.meta.date))
                    }
                    " "
                    time {
                        (minutes) " minute" @if minutes != 1 { "s" }
                    }
                }
                a.u-url href={ "/posts/" (post.slug) "/" } style="display: none" {}
                div.e-content { (PreEscaped(&post.body_html)) }
                footer.post-footer {
                    @if !post.meta.syndication.is_empty() {
                        p.syndication {
                            "Also on"
                            @for (i, url) in post.meta.syndication.iter().enumerate() {
                                @if i > 0 { ", " } @else { " " }
                                a.u-syndication rel="syndication" href=(url) {
                                    (syndication_label(url))
                                }
                            }
                        }
                    }
                    @if !post.meta.tags.is_empty() {
                        p {
                            @for (i, tag) in post.meta.tags.iter().enumerate() {
                                @if i > 0 { " " }
                                a.p-category href={ "/tags/" (tag_slug(tag)) "/" } { "#" (tag) }
                            }
                        }
                    }
                    p {
                        a.p-author.h-card href="/about/" { (config.author) }
                    }
                    @if !config.post_socials.is_empty() {
                        p {
                            @for (i, s) in config.post_socials.iter().enumerate() {
                                @if i > 0 { ", " }
                                a rel="me" href=(s.url) { (s.label) }
                            }
                        }
                    }
                }
                @if newer.is_some() || older.is_some() {
                    div.post-nav {
                        @if let Some(n) = newer {
                            a href={ "/posts/" (n.slug) "/" } {
                                (PreEscaped("&larr; ")) (n.meta.title)
                            }
                        }
                        @if let Some(o) = older {
                            a href={ "/posts/" (o.slug) "/" } {
                                (o.meta.title) (PreEscaped(" &rarr;"))
                            }
                        }
                    }
                }
            }
        },
    )
}

pub fn page(config: &Config, current: &Page) -> Markup {
    let path = format!("/{}/", current.slug);
    let meta = Meta {
        title: &current.meta.title,
        path: &path,
        description: current.meta.description.as_deref(),
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            article {
                h2.title { (current.meta.title) }
                (PreEscaped(&current.body_html))
            }
        },
    )
}

pub fn blogroll(config: &Config, current: &Page, entries: &[BlogrollEntry]) -> Markup {
    let path = format!("/{}/", current.slug);
    let meta = Meta {
        title: &current.meta.title,
        path: &path,
        description: current.meta.description.as_deref(),
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            section aria-labelledby="blogroll-heading" {
                h2 #blogroll-heading .title { (current.meta.title) }
                (PreEscaped(&current.body_html))
                @if !entries.is_empty() {
                    ul.list {
                        @for entry in entries {
                            li {
                                a href=(entry.url) target="_blank" rel="noopener" { (entry.name) }
                                @if entry.feed_url != entry.url {
                                    " ("
                                    a href=(entry.feed_url) target="_blank" rel="noopener" { "feed" }
                                    ")"
                                }
                                @if let Some(desc) = &entry.description {
                                    " " span.description { (desc) }
                                }
                            }
                        }
                    }
                }
                p {
                    a href="/blogroll.opml" { "Download the OPML" }
                    " to subscribe to all of these feeds in your feed reader. I use "
                    a target="_blank" rel="noopener" href="https://miniflux.app/" { "Miniflux" }
                    "!"
                }
            }
        },
    )
}

pub fn not_found(config: &Config) -> Markup {
    let meta = Meta {
        title: "404",
        path: "/404.html",
        description: Some("Page not found."),
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            h2.title { "Oops, this page doesn't exist." }
        },
    )
}

pub fn tags_index(config: &Config, tags: &BTreeMap<String, Vec<&Post>>) -> Markup {
    let meta = Meta {
        title: "Tags",
        path: "/tags/",
        description: None,
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            h2.title { "Tags" }
            ul.list {
                @for (tag, posts) in tags {
                    li {
                        a href={ "/tags/" (tag_slug(tag)) "/" } { (tag) }
                        " "
                        span.count {
                            (posts.len()) " post"
                            @if posts.len() != 1 { "s" }
                        }
                    }
                }
            }
        },
    )
}

pub fn tag_page(config: &Config, tag: &str, posts: &[&Post]) -> Markup {
    let path = format!("/tags/{}/", tag_slug(tag));
    let title = format!("Tag: {tag}");
    let meta = Meta {
        title: &title,
        path: &path,
        description: None,
        og_type: OgType::Website,
        published_time: None,
    };
    base(
        config,
        meta,
        html! {
            div.h-feed {
                h2.title.p-name {
                    "Posts tagged " (PreEscaped("&ldquo;")) (tag) (PreEscaped("&rdquo;"))
                }
                p {
                    a href={ "/tags/" (tag_slug(tag)) "/atom.xml" } { "Feed for this tag" }
                }
                ul.list {
                    @for post in posts {
                        li.h-entry {
                            a.p-name.u-url href={ "/posts/" (post.slug) "/" } { (post.meta.title) }
                            " "
                            time.dt-published datetime=(post.meta.date) {
                                (format_date(&post.meta.date))
                            }
                        }
                    }
                }
            }
        },
    )
}

fn reading_minutes(words: usize) -> usize {
    ((words as f64 / 225.0).ceil() as usize).max(1)
}

fn syndication_label(url: &str) -> &'static str {
    if url.contains("bsky.app") {
        "Bluesky"
    } else if url.contains("mastodon") {
        "Mastodon"
    } else if url.contains("x.com") || url.contains("twitter.com") {
        "X"
    } else {
        "link"
    }
}

fn format_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    if iso.len() != 10 {
        return iso.to_string();
    }
    let month_n: usize = iso[5..7].parse().unwrap_or(0);
    let day: u32 = iso[8..10].parse().unwrap_or(0);
    if month_n == 0 || day == 0 || month_n > 12 {
        return iso.to_string();
    }
    format!("{} {}, {}", MONTHS[month_n - 1], day, &iso[..4])
}

const LIVE_RELOAD_JS: &str = r#"
    new EventSource("/__live").onmessage = () => location.reload();
"#;
