#!/usr/bin/env python3
"""Regenerate data/blogroll.toml and static/blogroll.opml from data/feeds.opml."""

import html
import xml.etree.ElementTree as ET
from pathlib import Path

BASE = Path(__file__).parent.parent
SITE_TITLE = "Thomas Seeley"


def derive_homepage(url: str) -> str:
    url = url.rstrip("/")
    for suffix in [
        "/feed/atom",
        "/feed.atom",
        "/atom.xml",
        "/feed.xml",
        "/rss.xml",
        "/index.xml",
        "/feed/rss",
        "/feed/",
        "/feed",
        "/rss/",
        "/rss",
        "/atom/",
    ]:
        if url.lower().endswith(suffix):
            return url[: -len(suffix)].rstrip("/") or url
    return url


def parse_opml(path: Path) -> list[dict]:
    if not path.exists() or path.stat().st_size == 0:
        return []
    root = ET.parse(path).getroot()

    def feeds(parent):
        out = []
        for o in parent.findall("outline"):
            if xml_url := o.get("xmlUrl"):
                out.append(
                    {
                        "name": (o.get("text") or o.get("title") or "").strip()
                        or "Untitled",
                        "url": (o.get("htmlUrl") or derive_homepage(xml_url)).rstrip(
                            "/"
                        ),
                        "feed_url": xml_url,
                    }
                )
            else:
                out.extend(feeds(o))
        return out

    blogroll = next(
        (
            o
            for o in root.iter("outline")
            if (o.get("text") or "").strip().lower() == "blogroll"
        ),
        None,
    )
    parent = blogroll if blogroll is not None else (root.find("body") or root)
    return feeds(parent)


def toml_escape(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def write_toml(entries: list[dict], path: Path) -> None:
    lines = []
    for e in entries:
        lines.append("[[entries]]")
        lines.append(f'name = "{toml_escape(e["name"])}"')
        lines.append(f'url = "{toml_escape(e["url"])}"')
        lines.append(f'feed_url = "{toml_escape(e["feed_url"])}"')
        lines.append("")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines))


def write_opml(entries: list[dict], path: Path) -> None:
    items = "\n".join(
        f'    <outline type="rss" text="{html.escape(e["name"])}" '
        f'xmlUrl="{html.escape(e["feed_url"])}" htmlUrl="{html.escape(e["url"])}"/>'
        for e in entries
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<opml version="2.0">\n'
        f"  <head><title>Blogroll - {SITE_TITLE}</title></head>\n"
        "  <body>\n"
        f"{items}\n"
        "  </body>\n</opml>\n"
    )


def main() -> None:
    entries = sorted(
        parse_opml(BASE / "data" / "feeds.opml"),
        key=lambda e: e["name"].lower(),
    )
    write_toml(entries, BASE / "data" / "blogroll.toml")
    write_opml(entries, BASE / "static" / "blogroll.opml")
    print(
        f"Wrote {len(entries)} entries to data/blogroll.toml and static/blogroll.opml"
    )


if __name__ == "__main__":
    main()
