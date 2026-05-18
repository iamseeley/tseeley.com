use crate::Config;
use crate::build::{Page, Post, tag_slug};
use chrono::Utc;
use std::collections::BTreeMap;
use std::fmt::Write;

pub fn render(
    config: &Config,
    pages: &[Page],
    posts: &[&Post],
    tags: &BTreeMap<String, Vec<&Post>>,
) -> String {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut out = String::with_capacity(2048);
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    write_url(&mut out, &format!("{}/", config.base_url), &today);
    write_url(&mut out, &format!("{}/posts/", config.base_url), &today);
    write_url(&mut out, &format!("{}/tags/", config.base_url), &today);

    for post in posts {
        let url = format!("{}/posts/{}/", config.base_url, post.slug);
        write_url(&mut out, &url, &post.meta.date);
    }

    for page in pages {
        let url = format!("{}/{}/", config.base_url, page.slug);
        write_url(&mut out, &url, &today);
    }

    for tag in tags.keys() {
        let url = format!("{}/tags/{}/", config.base_url, tag_slug(tag));
        write_url(&mut out, &url, &today);
    }

    out.push_str("</urlset>\n");
    out
}

pub fn robots_txt(config: &Config) -> String {
    format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        config.base_url
    )
}

fn write_url(out: &mut String, loc: &str, lastmod: &str) {
    let _ = writeln!(
        out,
        "  <url><loc>{}</loc><lastmod>{}</lastmod></url>",
        xml_escape(loc),
        lastmod
    );
}

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}
