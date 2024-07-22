---
title: cache it?
date: 2024-07-22
image: https://cdn.mos.cms.futurecdn.net/SMDYhDUZcKuX9YV9vCrvEk-1200-80.jpg
---

playing around with caching pages and templates for this site -> [simpl-site](https://github.com/iamseeley/simpl-site). 

it seems silly to go through the process of retreiving the handlebars template and markdown content then rendering the html on every page load.

i implemented a pageCache map in the [simplSite](https://github.com/iamseeley/simpl-site/commit/9287c696421c6e32608ad0a9f8ff5bd89bc26472) class that caches the final rendered html for each page. so, on subsequent requests it serves the cached content if it's available.

also, i added template caching to the templateEngine class to cache the handlebars templates!
