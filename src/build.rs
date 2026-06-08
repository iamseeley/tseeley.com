use crate::{Config, feeds, layout, sitemap, theme};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::Theme;
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::SyntaxSet;

pub fn tag_slug(tag: &str) -> String {
    tag.to_lowercase().replace(' ', "-")
}

#[derive(Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub syndication: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PageMeta {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

pub struct Post {
    pub meta: Frontmatter,
    pub slug: String,
    pub body_html: String,
    pub word_count: usize,
}

pub struct Page {
    pub meta: PageMeta,
    pub slug: String,
    pub body_html: String,
}

#[derive(Deserialize)]
struct BlogrollFile {
    entries: Vec<BlogrollEntry>,
}

#[derive(Deserialize)]
pub struct BlogrollEntry {
    pub name: String,
    pub url: String,
    pub feed_url: String,
}

fn load_blogroll() -> Vec<BlogrollEntry> {
    let raw = match fs::read_to_string("data/blogroll.toml") {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    match toml::from_str::<BlogrollFile>(&raw) {
        Ok(b) => b.entries,
        Err(e) => {
            eprintln!("warning: data/blogroll.toml failed to parse: {e}");
            Vec::new()
        }
    }
}

#[cfg(feature = "serve")]
pub fn run_static_only() -> Result<(), Box<dyn Error>> {
    copy_dir(Path::new("static"), Path::new("public")).map_err(|e| -> Box<dyn Error> { e.into() })
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let post_paths = walk(Path::new("content/posts"), "dj")?;
    let mut posts: Vec<Post> = post_paths
        .iter()
        .map(|p| parse_post(p))
        .collect::<Result<_, _>>()?;
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    let visible_posts: Vec<&Post> = posts
        .iter()
        .filter(|p| config.show_drafts || !p.meta.draft)
        .collect();

    let page_paths = list_pages()?;
    let mut pages: Vec<Page> = page_paths
        .iter()
        .map(|p| parse_page(p))
        .collect::<Result<_, _>>()?;
    pages.sort_by(|a, b| a.meta.title.cmp(&b.meta.title));

    fs::create_dir_all("public")?;

    fs::write(
        "public/index.html",
        layout::home(config, &visible_posts).into_string(),
    )?;

    fs::write("public/404.html", layout::not_found(config).into_string())?;

    let posts_dir = PathBuf::from("public/posts");
    fs::create_dir_all(&posts_dir)?;
    fs::write(
        posts_dir.join("index.html"),
        layout::posts_index(config, &visible_posts).into_string(),
    )?;

    for (i, post) in visible_posts.iter().enumerate() {
        let newer = if i > 0 {
            Some(visible_posts[i - 1])
        } else {
            None
        };
        let older = visible_posts.get(i + 1).copied();

        let dir = posts_dir.join(&post.slug);
        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join("index.html"),
            layout::post(config, post, newer, older).into_string(),
        )?;
    }

    let blogroll = load_blogroll();
    for page in &pages {
        let dir = PathBuf::from("public").join(&page.slug);
        fs::create_dir_all(&dir)?;
        let html = if page.slug == "blogroll" {
            layout::blogroll(config, page, &blogroll).into_string()
        } else {
            layout::page(config, page).into_string()
        };
        fs::write(dir.join("index.html"), html)?;
    }

    let mut tag_map: BTreeMap<String, Vec<&Post>> = BTreeMap::new();
    for post in &visible_posts {
        for tag in &post.meta.tags {
            tag_map.entry(tag.clone()).or_default().push(post);
        }
    }

    let tags_dir = PathBuf::from("public/tags");
    fs::create_dir_all(&tags_dir)?;
    fs::write(
        tags_dir.join("index.html"),
        layout::tags_index(config, &tag_map).into_string(),
    )?;

    for (tag, tagged_posts) in &tag_map {
        let dir = tags_dir.join(tag_slug(tag));
        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join("index.html"),
            layout::tag_page(config, tag, tagged_posts).into_string(),
        )?;
        fs::write(
            dir.join("atom.xml"),
            feeds::tag_feed(config, tag, tagged_posts, &visible_posts),
        )?;
    }

    fs::write("public/atom.xml", feeds::site_feed(config, &visible_posts))?;

    fs::write(
        "public/sitemap.xml",
        sitemap::render(config, &pages, &visible_posts, &tag_map),
    )?;
    fs::write("public/robots.txt", sitemap::robots_txt(config))?;

    copy_dir(Path::new("static"), Path::new("public"))?;
    Ok(())
}

fn parse_post(path: &Path) -> Result<Post, Box<dyn Error>> {
    let raw = fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;

    let (fm_text, body_text) =
        split_frontmatter(&raw).ok_or_else(|| format!("no frontmatter in {}", path.display()))?;

    let meta: Frontmatter =
        toml::from_str(fm_text).map_err(|e| format!("frontmatter in {}: {e}", path.display()))?;

    validate_date(&meta.date).map_err(|e| format!("date in {}: {e}", path.display()))?;

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("bad filename: {}", path.display()))?
        .to_string();

    let body_html = render_djot(body_text);
    let word_count = body_text.split_whitespace().count();

    Ok(Post {
        meta,
        slug,
        body_html,
        word_count,
    })
}

fn parse_page(path: &Path) -> Result<Page, Box<dyn Error>> {
    let raw = fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))?;

    let (fm_text, body_text) =
        split_frontmatter(&raw).ok_or_else(|| format!("no frontmatter in {}", path.display()))?;

    let meta: PageMeta =
        toml::from_str(fm_text).map_err(|e| format!("frontmatter in {}: {e}", path.display()))?;

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("bad filename: {}", path.display()))?
        .to_string();

    let body_html = render_djot(body_text);

    Ok(Page {
        meta,
        slug,
        body_html,
    })
}

fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
    let rest = raw.strip_prefix("+++\n")?;
    let end = rest.find("\n+++\n")?;
    Some((&rest[..end], &rest[end + 5..]))
}

fn validate_date(s: &str) -> Result<(), &'static str> {
    let bytes = s.as_bytes();
    if bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && s[..4].chars().all(|c| c.is_ascii_digit())
        && s[5..7].chars().all(|c| c.is_ascii_digit())
        && s[8..10].chars().all(|c| c.is_ascii_digit())
    {
        Ok(())
    } else {
        Err("expected YYYY-MM-DD")
    }
}

fn render_djot(input: &str) -> String {
    let html = jotdown::html::render_to_string(jotdown::Parser::new(input));
    let html = linkify_headings(&html);
    highlight_code_blocks(&html)
}

fn syntaxes() -> &'static SyntaxSet {
    static CELL: OnceLock<SyntaxSet> = OnceLock::new();
    CELL.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn sally_theme() -> &'static Theme {
    static CELL: OnceLock<Theme> = OnceLock::new();
    CELL.get_or_init(theme::sally)
}

fn highlight_code_blocks(html: &str) -> String {
    let syntaxes = syntaxes();
    let theme = sally_theme();
    let mut out = String::with_capacity(html.len() + 1024);
    let mut pos = 0;
    let open_prefix = "<pre><code class=\"language-";
    let close = "</code></pre>";

    while let Some(rel) = html[pos..].find(open_prefix) {
        let block_start = pos + rel;
        out.push_str(&html[pos..block_start]);

        let lang_start = block_start + open_prefix.len();
        let lang_end = match html[lang_start..].find('"') {
            Some(i) => lang_start + i,
            None => break,
        };
        let lang = &html[lang_start..lang_end];

        let content_start = lang_end + 2;
        let content_end = match html[content_start..].find(close) {
            Some(i) => content_start + i,
            None => break,
        };

        let decoded = html_decode(&html[content_start..content_end]);
        let highlighted = highlight(&decoded, lang, syntaxes, theme);

        out.push_str("<pre><code data-lang=\"");
        out.push_str(lang);
        out.push_str("\">");
        out.push_str(&highlighted);
        out.push_str(close);

        pos = content_end + close.len();
    }
    out.push_str(&html[pos..]);
    out
}

fn highlight(code: &str, lang: &str, syntaxes: &SyntaxSet, theme: &Theme) -> String {
    let lang = alias_language(lang);
    let syntax = syntaxes
        .find_syntax_by_token(lang)
        .or_else(|| syntaxes.find_syntax_by_extension(lang))
        .or_else(|| syntaxes.find_syntax_by_name(lang))
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text());

    let mut hl = HighlightLines::new(syntax, theme);
    let mut out = String::with_capacity(code.len() + 256);
    for line in code.split_inclusive('\n') {
        match hl.highlight_line(line, syntaxes) {
            Ok(ranges) => match styled_line_to_highlighted_html(&ranges, IncludeBackground::No) {
                Ok(s) => out.push_str(&s),
                Err(_) => out.push_str(line),
            },
            Err(_) => out.push_str(line),
        }
    }
    out
}

fn alias_language(lang: &str) -> &str {
    match lang {
        "ts" | "typescript" | "tsx" => "javascript",
        "scheme" | "racket" | "lisp" => "clojure",
        other => other,
    }
}

fn html_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        if let Some(semi) = rest.find(';') {
            let replacement = match &rest[1..semi] {
                "lt" => Some('<'),
                "gt" => Some('>'),
                "amp" => Some('&'),
                "quot" => Some('"'),
                "apos" | "#39" => Some('\''),
                _ => None,
            };
            if let Some(c) = replacement {
                out.push(c);
                rest = &rest[semi + 1..];
                continue;
            }
        }
        out.push('&');
        rest = &rest[1..];
    }
    out.push_str(rest);
    out
}

fn linkify_headings(html: &str) -> String {
    let mut out = String::with_capacity(html.len() + 256);
    let mut pos = 0;
    while let Some(rel) = html[pos..].find("<section id=\"") {
        let sec_start = pos + rel;
        out.push_str(&html[pos..sec_start]);

        let id_start = sec_start + r#"<section id=""#.len();
        let id_end = match html[id_start..].find('"') {
            Some(i) => id_start + i,
            None => break,
        };
        let after_open = match html[id_end..].find('>') {
            Some(i) => id_end + i + 1,
            None => break,
        };
        let id = &html[id_start..id_end];

        let h_pos = after_open
            + html[after_open..]
                .bytes()
                .take_while(|b| b.is_ascii_whitespace())
                .count();

        let bytes = html.as_bytes();
        if h_pos + 4 <= bytes.len()
            && &bytes[h_pos..h_pos + 2] == b"<h"
            && bytes[h_pos + 2].is_ascii_digit()
            && bytes[h_pos + 3] == b'>'
        {
            let close = format!("</h{}>", bytes[h_pos + 2] as char);
            let content_start = h_pos + 4;
            if let Some(rel_close) = html[content_start..].find(&close) {
                let content_end = content_start + rel_close;
                out.push_str(&html[sec_start..content_start]);
                out.push_str("<a class=\"heading-anchor\" href=\"#");
                out.push_str(id);
                out.push_str("\">");
                out.push_str(&html[content_start..content_end]);
                out.push_str("</a>");
                out.push_str(&close);
                pos = content_end + close.len();
                continue;
            }
        }
        out.push_str(&html[sec_start..after_open]);
        pos = after_open;
    }
    out.push_str(&html[pos..]);
    out
}

fn walk(root: &Path, ext: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_into(root, ext, &mut out)?;
    Ok(out)
}

fn walk_into(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            walk_into(&path, ext, out)?;
        } else if ft.is_file() && path.extension().is_some_and(|e| e == ext) {
            out.push(path);
        }
    }
    Ok(())
}

fn list_pages() -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in fs::read_dir("content")? {
        let entry = entry?;
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let path = entry.path();
        if entry.file_type()?.is_file() && path.extension().is_some_and(|e| e == "dj") {
            out.push(path);
        }
    }
    Ok(out)
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}
