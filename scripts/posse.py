#!/usr/bin/env python3
"""POSSE: syndicate posts to social media.

Usage:
    python scripts/posse.py                    # syndicate all pending posts
    python scripts/posse.py --dry-run          # preview without posting
    python scripts/posse.py --post hello       # syndicate a specific post

Mark a post for syndication in its frontmatter:

    syndicate_to = ["bluesky", "mastodon"]
    syndication_text = "Optional custom text for social posts."

After successful syndication the script writes the resulting URLs back into
the post's `syndication = [...]` array and removes `syndicate_to`, so the
script is idempotent.

Credentials live in .env at the repo root:

    BLUESKY_HANDLE=tseeley.com
    BLUESKY_APP_PASSWORD=xxxx-xxxx-xxxx-xxxx
    MASTODON_INSTANCE=https://mastodon.social
    MASTODON_ACCESS_TOKEN=xxxx
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

BASE = Path(__file__).parent.parent
CONTENT_DIR = BASE / "content" / "posts"


def load_env() -> None:
    env_path = BASE / ".env"
    if not env_path.exists():
        return
    for line in env_path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        key, _, value = line.partition("=")
        key = key.strip()
        value = value.strip().strip("\"'")
        if key and value:
            os.environ.setdefault(key, value)


def parse_toml_subset(text: str) -> dict:
    """Tiny TOML parser for the subset we need: top-level scalar keys plus
    single-line and multi-line string arrays. Ignores table headers."""
    result: dict = {}
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        i += 1
        if not line or line.startswith("#") or (line.startswith("[") and line.endswith("]")):
            continue
        if "=" not in line:
            continue
        key, _, value = line.partition("=")
        key = key.strip()
        value = value.strip()

        if value == "[":
            items = []
            while i < len(lines):
                inner = lines[i].strip().rstrip(",").strip()
                i += 1
                if inner.startswith("]"):
                    break
                if inner:
                    items.append(_parse_scalar(inner))
            result[key] = items
        elif value.startswith("[") and value.endswith("]"):
            inner = value[1:-1].strip()
            result[key] = [
                _parse_scalar(p.strip())
                for p in inner.split(",")
                if p.strip()
            ]
        else:
            result[key] = _parse_scalar(value)
    return result


def _parse_scalar(value: str):
    if value.startswith('"') and value.endswith('"'):
        return value[1:-1]
    if value == "true":
        return True
    if value == "false":
        return False
    return value


def load_base_url() -> str:
    return parse_toml_subset((BASE / "config.toml").read_text())["base_url"]


def parse_frontmatter(text: str) -> tuple[dict, str]:
    match = re.match(r"^\+\+\+\n(.*?)\n\+\+\+\n?(.*)", text, re.DOTALL)
    if not match:
        return {}, text
    return parse_toml_subset(match.group(1)), match.group(2)


def get_post_url(base_url: str, post_path: Path) -> str:
    return f"{base_url}/posts/{post_path.stem}/"


def strip_djot(body: str) -> str:
    """Best-effort plain-text reduction of djot source for use in summaries."""
    s = body.strip()
    s = re.sub(r"```=html.*?```", "", s, flags=re.DOTALL)
    s = re.sub(r"```.*?```", "", s, flags=re.DOTALL)
    s = re.sub(r"!\[.*?\]\(.*?\)\{[^}]*\}", "", s)
    s = re.sub(r"!\[.*?\]\(.*?\)", "", s)
    s = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", s)
    s = re.sub(r"^\[\w+\]:.*$", "", s, flags=re.MULTILINE)
    s = re.sub(r"\[\^[\w-]+\]", "", s)
    s = re.sub(r"<[^>]+>", "", s)
    s = re.sub(r"^#{1,6}\s+", "", s, flags=re.MULTILINE)
    s = re.sub(r"\*\*([^*]+)\*\*", r"\1", s)
    s = re.sub(r"\*([^*]+)\*", r"\1", s)
    s = re.sub(r"_([^_\n]+)_", r"\1", s)
    s = re.sub(r"^\{[^}]+\}\s*$", "", s, flags=re.MULTILINE)
    s = re.sub(r"\{[^}]+\}", "", s)
    s = re.sub(r"\n{2,}", "\n\n", s).strip()
    return s


def get_post_summary(body: str, title: str, url: str, max_len: int = 280) -> str:
    plain = strip_djot(body)
    first_para = plain.split("\n\n")[0].strip() or plain[:200]

    prefix = f"{title}\n\n" if title else ""
    suffix = f"\n\n{url}" if url else ""
    available = max_len - len(prefix) - len(suffix)

    if len(first_para) > available:
        first_para = first_para[: available - 1].rsplit(" ", 1)[0] + "…"

    return f"{prefix}{first_para}{suffix}"


def _xrpc(pds: str, method: str, token: str | None, payload: dict) -> dict:
    url = f"{pds}/xrpc/{method}"
    headers = {"Content-Type": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    req = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers=headers,
        method="POST",
    )
    with urllib.request.urlopen(req) as resp:
        return json.loads(resp.read())


def post_to_bluesky(
    title: str,
    body: str,
    url: str,
    tags: list[str],
    syndication_text: str | None = None,
    dry_run: bool = False,
) -> str | None:
    handle = os.environ.get("BLUESKY_HANDLE")
    password = os.environ.get("BLUESKY_APP_PASSWORD")
    if not handle or not password:
        print("  [bluesky] BLUESKY_HANDLE and BLUESKY_APP_PASSWORD not set, skipping")
        return None

    if syndication_text:
        text = f"{syndication_text}\n\n{url}"
    else:
        text = get_post_summary(body, title, url, max_len=300)

    if dry_run:
        print(f"  [bluesky] Would post ({len(text)} chars):")
        print(f"    {text[:200]}...")
        return "https://bsky.app/profile/DRY_RUN/post/DRY_RUN"

    pds = "https://bsky.social"

    try:
        session = _xrpc(pds, "com.atproto.server.createSession", None, {
            "identifier": handle,
            "password": password,
        })
        token = session["accessJwt"]
        did = session["did"]

        facets = []

        url_start = text.index(url)
        url_end = url_start + len(url)
        facets.append({
            "index": {
                "byteStart": len(text[:url_start].encode("utf-8")),
                "byteEnd": len(text[:url_end].encode("utf-8")),
            },
            "features": [{
                "$type": "app.bsky.richtext.facet#link",
                "uri": url,
            }],
        })

        for tag in tags[:3]:
            hashtag = f"#{tag}"
            if hashtag in text:
                tag_start = text.index(hashtag)
                tag_end = tag_start + len(hashtag)
                facets.append({
                    "index": {
                        "byteStart": len(text[:tag_start].encode("utf-8")),
                        "byteEnd": len(text[:tag_end].encode("utf-8")),
                    },
                    "features": [{
                        "$type": "app.bsky.richtext.facet#tag",
                        "tag": tag,
                    }],
                })

        record = {
            "$type": "app.bsky.feed.post",
            "text": text,
            "createdAt": datetime.now(timezone.utc).isoformat(),
            "facets": facets,
            "embed": {
                "$type": "app.bsky.embed.external",
                "external": {
                    "uri": url,
                    "title": title,
                    "description": strip_djot(body)[:200].strip(),
                },
            },
        }

        result = _xrpc(pds, "com.atproto.repo.createRecord", token, {
            "repo": did,
            "collection": "app.bsky.feed.post",
            "record": record,
        })

        rkey = result["uri"].split("/")[-1]
        post_url = f"https://bsky.app/profile/{handle}/post/{rkey}"
        print(f"  [bluesky] Posted: {post_url}")
        return post_url

    except Exception as e:
        print(f"  [bluesky] Error: {e}")
        return None


def post_to_mastodon(
    title: str,
    body: str,
    url: str,
    tags: list[str],
    syndication_text: str | None = None,
    dry_run: bool = False,
) -> str | None:
    instance = os.environ.get("MASTODON_INSTANCE")
    token = os.environ.get("MASTODON_ACCESS_TOKEN")
    if not instance or not token:
        print("  [mastodon] MASTODON_INSTANCE and MASTODON_ACCESS_TOKEN not set, skipping")
        return None

    if syndication_text:
        text = f"{syndication_text}\n\n{url}"
    else:
        text = get_post_summary(body, title, url, max_len=480)

    hashtags = " ".join(f"#{tag.replace('-', '')}" for tag in tags[:5])
    if hashtags and len(text) + len(hashtags) + 2 <= 500:
        text = f"{text}\n\n{hashtags}"

    if dry_run:
        print(f"  [mastodon] Would post ({len(text)} chars):")
        print(f"    {text[:200]}...")
        return "https://mastodon.social/@DRY_RUN/DRY_RUN"

    try:
        req = urllib.request.Request(
            f"{instance.rstrip('/')}/api/v1/statuses",
            data=json.dumps({"status": text}).encode("utf-8"),
            headers={
                "Authorization": f"Bearer {token}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        with urllib.request.urlopen(req) as resp:
            result = json.loads(resp.read())
            post_url = result["url"]
            print(f"  [mastodon] Posted: {post_url}")
            return post_url

    except Exception as e:
        print(f"  [mastodon] Error: {e}")
        return None


def write_syndication_urls(post_path: Path, urls: list[str]) -> None:
    """Surgically update the post's frontmatter:
    - Insert/replace `syndication = [...]`
    - Remove `syndicate_to = [...]` (and `syndication_text = ...`) lines so
      this post won't be re-syndicated next run.
    """
    text = post_path.read_text()
    match = re.match(r"^(\+\+\+\n)(.*?)(\n\+\+\+)", text, re.DOTALL)
    if not match:
        print(f"  Could not parse frontmatter in {post_path}")
        return

    fm_text = match.group(2)
    rest = text[match.end():]

    syndication_toml = (
        "syndication = [\n" + "".join(f'    "{u}",\n' for u in urls) + "]"
    )

    if re.search(r"^syndication\s*=", fm_text, re.MULTILINE):
        fm_text = re.sub(
            r"syndication\s*=\s*\[.*?\]",
            syndication_toml,
            fm_text,
            flags=re.DOTALL,
        )
    else:
        fm_text = fm_text.rstrip() + "\n" + syndication_toml

    fm_text = re.sub(r"\nsyndicate_to\s*=\s*\[.*?\]\n?", "\n", fm_text, flags=re.DOTALL)
    fm_text = re.sub(r"\nsyndication_text\s*=\s*\".*?\"\n?", "\n", fm_text)

    post_path.write_text(f"+++\n{fm_text}\n+++{rest}")
    print(f"  Wrote syndication URLs to {post_path.name}")


def find_posts_to_syndicate(specific_post: str | None = None) -> list[Path]:
    if specific_post:
        path = CONTENT_DIR / f"{specific_post}.dj"
        if not path.exists():
            print(f"Post not found: {path}")
            sys.exit(1)
        return [path]

    posts = []
    for path in sorted(CONTENT_DIR.glob("*.dj")):
        fm, _ = parse_frontmatter(path.read_text())
        if (
            fm.get("syndicate_to")
            and not fm.get("syndication")
            and not fm.get("draft", False)
        ):
            posts.append(path)
    return posts


def syndicate_post(base_url: str, post_path: Path, dry_run: bool = False) -> list[str]:
    fm, body = parse_frontmatter(post_path.read_text())
    targets = fm.get("syndicate_to", [])
    syndication_text = fm.get("syndication_text")
    title = fm.get("title", "Untitled")
    tags = fm.get("tags", [])
    url = get_post_url(base_url, post_path)

    print(f"\n{title}")
    print(f"  URL: {url}")
    print(f"  Targets: {', '.join(targets)}")

    syndication_urls: list[str] = []
    for target in targets:
        target = target.lower().strip()
        if target == "bluesky":
            result = post_to_bluesky(title, body, url, tags, syndication_text, dry_run)
        elif target == "mastodon":
            result = post_to_mastodon(title, body, url, tags, syndication_text, dry_run)
        else:
            print(f"  [{target}] Unknown target, skipping")
            continue
        if result:
            syndication_urls.append(result)

    if syndication_urls and not dry_run:
        write_syndication_urls(post_path, syndication_urls)
    return syndication_urls


def main() -> None:
    parser = argparse.ArgumentParser(description="POSSE: syndicate posts to social media")
    parser.add_argument("--dry-run", action="store_true", help="Preview without posting")
    parser.add_argument("--post", type=str, help="Syndicate a specific post by slug")
    args = parser.parse_args()

    load_env()
    base_url = load_base_url()

    posts = find_posts_to_syndicate(args.post)

    if not posts:
        print("No posts to syndicate.")
        print('Add syndicate_to = ["bluesky", "mastodon"] to a post\'s frontmatter.')
        return

    print(f"Found {len(posts)} post(s) to syndicate")

    all_urls = []
    for post_path in posts:
        urls = syndicate_post(base_url, post_path, dry_run=args.dry_run)
        all_urls.extend(urls)

    if all_urls and not args.dry_run:
        print(f"\nSyndicated to {len(all_urls)} platform(s).")
        print("Rebuild your site to include the syndication links:")
        print("  cargo run -- build")
    elif args.dry_run:
        print("\nDry run complete. No posts were sent.")


if __name__ == "__main__":
    main()
