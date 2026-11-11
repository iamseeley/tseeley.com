+++
title = "Meta"
weight = 3
+++

The site's [style guide](#style-guide) and [colophon](#colophon).

## Style Guide

### Typography

The site uses [Georgia](https://en.wikipedia.org/wiki/Georgia_%28typeface%29) for the body and [Menlo](<https://en.wikipedia.org/wiki/Menlo_(typeface)>) for code. Text is sized relative to the browser's default font size.

I use three relative text sizes: `sm: 0.875rem`, `lg: 1.25rem`, and `xl: 2rem`.

### Headings

Each page has one `h1`. I rarely go past `h3`.

# h1

## h2

### h3

#### h4

### Emphasis

This is _italic_ and this is **bold**. This is **_bold and italic_**.

If I get something wrong, I'll ~~strike it through~~.

### Lists

### Unordered

- 2x russet potato
- carrots
- 2x yellow onion
- kokumaro curry (med hot)
- 2x extra firm tofu
- panko

### Ordered

1. Chop the veggies
2. Sauté the onions
3. Bread the tofu
   1. Set out bowls of flour, egg, and panko
   2. Coat the tofu in each, in that order
4. Fry the tofu

### Links

On hover, a link's text decoration indicates its destination:

- **Internal links** get a solid underline, like [this link to posts](/posts/)
- **External links** get a dotted underline, like [this link to djot](https://djot.net/)

Links in body copy use the accent color; other links inherit the color of the surrounding text.

### Code

Both inline code, like `fn main()`, and code blocks are set in Menlo.

```rust
fn main() {
    println!("Hello, tseeley.com!");
}
```

### Dinkus

Section breaks use this [dinkus](https://en.wikipedia.org/wiki/Dinkus):

---

### Layout

The site is a single centered column, like a page in a book. Spacing is done in one direction with `margin-top` on sibling elements.

### Elements

The site has a few repeat elements.

**Aside**: a callout with a left border.

<aside>This is an aside.</aside>

**Blockquote**: a styled quote with attribution.

<blockquote>
    <q>The computer is the first metamedium, and as such it has degrees of freedom for representation and expression never before encountered.</q>
    <cite>Alan Kay</cite>
</blockquote>

**Figure**: an image with a caption.

<figure style="max-width: 200px">
  <img src="/images/cute-ferris.png" alt="Cute Ferris the crab">
  <figcaption>Ferris by <a href="https://rustacean.net/">Karen Rustad Tölva</a>, CC0.</figcaption>
</figure>

## Colophon

### How it's built

I designed and built the site on [Zola](https://www.getzola.org/). Everything is intentionally simple: plain HTML, CSS, and minimal JavaScript.
Much of the design inspiration comes from the sites in my [blogroll](/blogroll).

The site is served by [Caddy](https://caddyserver.com/) on a [Hetzner](https://www.hetzner.com/) VPS.

### Mascot

The site's mascot is Sally, a [Yellow Spotted Salamander](https://auth1.dpr.ncparks.gov/amphibians/view.php?sciName=Ambystoma+maculatum) (_Ambystoma maculatum_). Her species is native to much of the eastern United States, including the Appalachian Mountains. You'll spot ;) her around the site.

### Feeds

Subscribe to the [full site feed](/atom.xml). Each [tag](/tags) also has its own feed, linked from its page.

### AI

All of the words written here are my own. The templates and styles will also always be written by me.

I sometimes use AI for research or help with code; when I do, I'll say so.

### Privacy

I use [Umami](https://umami.is/) for basic analytics like page views and referrers. It's self-hosted, collects no personal information, and uses no cookies.
