use crate::Config;
use crate::build::{Post, tag_slug};
use atom_syndication::{Content, Entry, Feed, Generator, Link, Person, Text};
use chrono::{DateTime, FixedOffset, Utc};
use std::fmt::Write;

pub fn site_feed(config: &Config, posts: &[&Post]) -> String {
    render(config, "", "/atom.xml", posts, posts)
}

pub fn tag_feed(config: &Config, tag: &str, tagged: &[&Post], all_posts: &[&Post]) -> String {
    let suffix = format!(" - {tag}");
    let path = format!("/tags/{}/atom.xml", tag_slug(tag));
    render(config, &suffix, &path, tagged, all_posts)
}

fn render(
    config: &Config,
    title_suffix: &str,
    feed_path: &str,
    posts: &[&Post],
    all_posts: &[&Post],
) -> String {
    let feed_url = format!("{}{}", config.base_url, feed_path);
    let title = format!("{}{}", config.title, title_suffix);

    let last_updated = posts
        .iter()
        .filter_map(|p| parse_date(&p.meta.date))
        .max()
        .unwrap_or_else(|| Utc::now().fixed_offset());

    let entries: Vec<Entry> = posts
        .iter()
        .map(|p| {
            let (newer, older) = neighbors(all_posts, p);
            make_entry(config, p, newer, older)
        })
        .collect();

    let feed = Feed {
        title: Text::plain(title),
        id: feed_url.clone(),
        subtitle: (!config.description.is_empty()).then(|| Text::plain(config.description.clone())),
        links: vec![
            Link {
                href: feed_url,
                rel: "self".into(),
                mime_type: Some("application/atom+xml".into()),
                ..Default::default()
            },
            Link {
                href: config.base_url.clone(),
                rel: "alternate".into(),
                mime_type: Some("text/html".into()),
                ..Default::default()
            },
        ],
        updated: last_updated,
        generator: Some(Generator {
            value: config.generator.clone(),
            ..Default::default()
        }),
        entries,
        ..Default::default()
    };

    feed.to_string()
}

fn make_entry(config: &Config, post: &Post, newer: Option<&Post>, older: Option<&Post>) -> Entry {
    let permalink = format!("{}/posts/{}/", config.base_url, post.slug);
    let date = parse_date(&post.meta.date).unwrap_or_else(|| Utc::now().fixed_offset());
    let content_html = build_entry_content(config, post, newer, older);

    Entry {
        title: Text::plain(post.meta.title.clone()),
        id: permalink.clone(),
        updated: date,
        published: Some(date),
        authors: vec![Person {
            name: config.author.clone(),
            ..Default::default()
        }],
        links: vec![Link {
            href: permalink,
            rel: "alternate".into(),
            mime_type: Some("text/html".into()),
            ..Default::default()
        }],
        content: Some(Content {
            value: Some(content_html),
            content_type: Some("html".into()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_entry_content(
    config: &Config,
    post: &Post,
    newer: Option<&Post>,
    older: Option<&Post>,
) -> String {
    let mut out = String::with_capacity(post.body_html.len() + 1024);
    out.push_str(&post.body_html);
    out.push_str("\n<hr>\n");

    if !post.meta.tags.is_empty() {
        out.push_str("<p>");
        for (i, tag) in post.meta.tags.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            let _ = write!(out, "<code>{}</code>", esc(tag));
        }
        out.push_str("</p>\n");
    }

    let _ = writeln!(
        out,
        r#"<p><a href="{}/about/">{}</a></p>"#,
        esc(&config.base_url),
        esc(&config.author),
    );

    if !config.post_socials.is_empty() {
        out.push_str("<p>");
        for (i, s) in config.post_socials.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, r#"<a href="{}">{}</a>"#, esc(&s.url), esc(&s.label));
        }
        out.push_str("</p>\n");
    }

    if newer.is_some() || older.is_some() {
        out.push_str("<p>");
        if let Some(n) = newer {
            let _ = write!(
                out,
                r#"&larr; <a href="{base}/posts/{slug}/">{title}</a>"#,
                base = esc(&config.base_url),
                slug = esc(&n.slug),
                title = esc(&n.meta.title),
            );
        }
        if newer.is_some() && older.is_some() {
            out.push_str(" · ");
        }
        if let Some(o) = older {
            let _ = write!(
                out,
                r#"<a href="{base}/posts/{slug}/">{title}</a> &rarr;"#,
                base = esc(&config.base_url),
                slug = esc(&o.slug),
                title = esc(&o.meta.title),
            );
        }
        out.push_str("</p>\n");
    }

    out
}

fn neighbors<'a>(all: &'a [&'a Post], target: &Post) -> (Option<&'a Post>, Option<&'a Post>) {
    let Some(idx) = all.iter().position(|p| p.slug == target.slug) else {
        return (None, None);
    };
    let newer = if idx > 0 { Some(all[idx - 1]) } else { None };
    let older = all.get(idx + 1).copied();
    (newer, older)
}

fn parse_date(iso: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(&format!("{iso}T00:00:00Z")).ok()
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}
