---
title: a simple website builder
date: 2024-08-06
description: simpl site is a dynamic website builder built on deno that features markdown, handlebars, and a plugin system for transforming content and extending templates.
draft: true
---

after a month or two of tinkering on [val.town](https://val.town), i found myself really enjoying working with deno (val.town uses the deno runtime to run your js/ts). i've enjoyed deno so much that i decided to create a website builder with it.

it's called [simpl-site](https://github.com/iamseeley/simpl-site): a server-side rendered website builder that let's you create dynamic websites with markdown content, handlebars templates, and a plugin system for transforming content and extending templates.

<aside>this website is powered by simpl-site!</aside>

it's my first [JSR](https://jsr.io/) package [@iamseeley/simpl-site](https://jsr.io/@iamseeley/simpl-site), and this is my first "technical" blog post!

in this post i want to share a little background on the project, then dive into the code to explore how it all works. i'll wrap it up with next steps for the project.


### i like building the thing that builds the thing

i have a confession to make. 

i redo my personal website way too often. sometimes, when i redo it i end up making the tool that builds the site. i spend all my time building the 'thing' that builds the 'thing.' i admit this might not be the healthiest / most effective pattern, but i think it can be pretty rewarding when it all comes together, and you get your website running with something you created.

for this project, what started as a website refresh turned into a journey of exploring server-side rendering, the deno ecosystem, and publishing modules via JavaScript Registry (jsr). 

### static site generation to server-side rendering

i think it's worth mentioning simpl-site's predecessor: [go-forth](https://github.com/iamseeley/go-forth), a static site generator i wrote in Go. go-forth was my first adventure in creating a website builder, and it laid the groundwork for many of the ideas I've implemented in simpl-site. 

go-forth taught me a lot about structuring a site generator, handling markdown content, and managing templates. 

it uses a similar concept of collections for organizing content and uses Go's html/template package for templating. while go-forth is a static site generator, simpl-site takes things a step further by introducing server-side rendering and a plugin system.

### simpl-site overview

before we get into the code specifics, let's take a high-level look at how simpl-site is built and how it leverages lots of Deno's features.

**key components and technologies:**

1. deno: simpl-site is built on deno, which provides a secure runtime for JavaScript and TypeScript. This allows us to use modern ES6+ features and TypeScript out of the box, without need for transpilation.

2. deno.serve: simpl-site uses deno's serve function, which provides a minimal way to create an HTTP server. this function handles incoming requests and routes them to the appropriate handler in our application.

3. file system operations: simpl-site uses deno's built-in apis for file system operations (Deno.readTextFile, Deno.writeTextFile, etc.) to read markdown content, templates, and other assets.

4. markdown processing: simpl-site uses marked, for markdown parsing and compiling.

5. handlebars templating: for HTML templating, simpl-site uses handlebars. this allows for dynamic content insertion and reusable template components.

6. plugin system: simpl-site implements a plugin architecture, allowing users to extend functionality. plugins can transform content or extend templates, providing a flexible way to customize the site generation process.

7. caching: To improve performance, simpl-site includes a caching system. this reduces the need to re-process unchanged content on every request.

8. static file serving: for assets like CSS, JavaScript, and images, simpl-site includes functionality to serve static files.

9. TypeScript: simpl-site is written in TypeScript.

the architecture of simpl-site is designed to be extensible. each request goes through a pipeline of processing steps: routing, content retrieval, markdown processing with marked, plugin transformations, template rendering, and finally, response sending.

### code deep dive: anatomy of a page request

let's walk through the lifecycle of a page request in simpl-site, following the process from initial request to final rendered output. this will take us through the key components of the `SimplSite` class and show how they work together to build a dynamic website.

#### **1. handling the incoming request**

when a request comes in, it's first handled by the `handleRequest` method:

```typescript
async handleRequest(path: string): Promise<{ content: string | Uint8Array; contentType: string; status: number }> {
  console.log(`Handling request for path: ${path}`);

  path = path.replace(/^\//, '');
  if (path === '') {
    path = 'index';
  }

  const staticFile = await this.serveStaticFile(path);
  if (staticFile) {
    console.log(`Serving static file: ${path}`);
    return { ...staticFile, status: 200 };
  }

  console.log(`Rendering content for path: ${path}`);
  const originalPath = path;
  path = path.endsWith('.md') ? path : path + '.md';

  let result;
  for (const source of this.contentSources) {
    if (originalPath.startsWith(source.route)) {
      const contentPath = path.slice(source.route.length);
      result = await this.renderContent(contentPath, source.type, '/' + originalPath);
      break;
    }
  }

  if (!result) {
    result = await this.renderContent(path, this.defaultContentType, '/' + originalPath);
  }

  return result;
}
```

this method first checks if the request is for a static file. if it is, it serves the file directly. if not, it determines the appropriate content source and calls `renderContent` to generate the dynamic content.

#### **2. retrieving the content**

if the request is for dynamic content, the next step is to retrieve the raw content:

```typescript
async getContent(path: string, type: string): Promise<string> {
  const source = this.contentSources.find(src => src.type === type);
  if (!source) {
    throw new Error(`Unknown content type: ${type}`);
  }
  const fullPath = join(source.path, path);
  return await Deno.readTextFile(fullPath);
}
```

this method finds the appropriate content source based on the content type and reads the file from the filesystem.

#### **3. processing the content**

once we have the raw content, it's time to process it:

```typescript
async processContent(content: string, type: string, route: string): Promise<{ content: string; metadata: Metadata }> {
  let { content: processedContent, metadata } = await this.markdownProcessor.execute(content);
  
  const context: PluginContext = {
    contentType: type,
    route: route,
    templateDir: this.templateDir,
    contentSources: Object.fromEntries(
      this.contentSources.map(source => [source.type, source.path])
    ),
    siteUrl: this.siteUrl 
  };
  
  for (const plugin of this.plugins.values()) {
    if (plugin.transform) {
      const result = await plugin.transform(processedContent, context);
      processedContent = result.content;
      if (result.metadata) {
        metadata = { ...metadata, ...result.metadata };
      }
    }
  }
  
  return { content: processedContent, metadata };
}
```

this method first processes the markdown content, then applies each plugin's transform function in sequence. this allows plugins to modify both the content and metadata, providing a way to extend the site's functionality.

#### **4. rendering the content**

with the processed content in hand, we're ready to render it using our handlebars templates:

```typescript
async renderContent(path: string, type: string, route: string): Promise<{ content: string; contentType: string; status: number }> {
  // ... (caching logic)

  const content = await this.getContent(path, type);
  const { content: processedContent, metadata } = await this.processContent(content, type, route);

  let templateContext: TemplateContext = {
    content: processedContent,
    metadata: metadata,
    route: route,
    siteTitle: this.siteTitle,
  };

  // Allow plugins to extend template context
  for (const plugin of this.plugins.values()) {
    if (plugin.extendTemplate) {
      templateContext = await plugin.extendTemplate(templateContext);
    }
  }

  const renderedContent = await this.templateEngine.render(type, templateContext);
  
  // ... (caching and return logic)
}
```

this method brings everything together. it retrieves the content, processes it, prepares the template context (including any extensions from plugins), and then renders the final HTML using the template engine.

#### **5. template rendering**

the actual rendering of templates is handled by the `TemplateEngine` class:

```typescript
async render(templateName: string, context: Record<string, unknown>): Promise<string> {
  console.log(`Rendering template: ${templateName}`);
  try {
    const templatePath = join(this.config.baseDir, `${templateName}${this.config.extname}`);
    const layoutPath = join(this.config.baseDir, this.config.layoutsDir, `${this.config.defaultLayout}${this.config.extname}`);
    
    if (!await exists(templatePath)) {
      throw new Error(`Template not found: ${templatePath}`);
    }
    const template = await this.getCompiledTemplate(templatePath);
    let result = template(context);
    if (await exists(layoutPath)) {
      const layout = await this.getCompiledTemplate(layoutPath);
      result = layout({ ...context, body: result });
    }
    console.log(`Template rendered successfully: ${templateName}`);
    return result;
  } catch (error) {
    console.error(`Error rendering template ${templateName}:`, error);
    throw error;
  }
}
```

this method compiles the template (or retrieves it from cache), renders it with the provided context, and then wraps it in a layout if one is specified. it also includes error handling to provide feedback if something goes wrong during the rendering process.

#### **6. caching for performance**

to improve performance, simpl-site implements a caching system:

```typescript
private shouldUseCache(path: string): boolean {
  if (!this.config.caching?.enabled) {
    return false;
  }
  if (this.config.caching.excludedRoutes) {
    return !this.config.caching.excludedRoutes.some(route => path.startsWith(route));
  }
  return true;
}
```

this method determines whether a particular path should be cached based on the configuration. it allows for fine-grained control over caching, including the ability to exclude specific routes.

by following this process for each request, simpl-site can efficiently handle requests, generate dynamic content, apply plugins, render templates, and serve the resulting pages.


### extending simpl-site: plugins and template helpers

one of the most useful features (i think) of simpl-site is its extensibility through plugins and template helpers.

#### **what can plugins do?**

plugins in simpl-site can:
1. transform content: modify your markdown content before it's rendered

2. extend templates: add new data or functions to you handlebars templates

#### **plugin system**

simpl-site uses a plugin registry to manage and load plugins dynamically. here's how it works:

```typescript
import type { Plugin } from "../types.ts";

const pluginRegistry: Record<string, new (options?: Record<string, unknown>) => Plugin> = {};

export function registerPluginType(name: string, pluginClass: new (options?: Record<string, unknown>) => Plugin) {
  pluginRegistry[name] = pluginClass;
}

export function getPluginClass(name: string): new (options?: Record<string, unknown>) => Plugin {
  const pluginClass = pluginRegistry[name];
  if (!pluginClass) {
    throw new Error(`Plugin ${name} not found in registry`);
  }
  return pluginClass;
}
```

this system allows you to register plugins and retrieve them by name, enabling a flexible and extensible architecture.

#### **creating a plugin**


let's look at a plugin i use in this website. the `ContentListPlugin` generates HTML lists of content items.

here's the full code for the `ContentListPlugin`:

```typescript
import type { Plugin, Metadata, PluginContext, TemplateContext } from "jsr:@iamseeley/simpl-site";
import { walk } from "https://deno.land/std/fs/mod.ts";
import { extname, basename } from "https://deno.land/std/path/mod.ts";
import { parse as parseYaml } from "https://deno.land/std/yaml/mod.ts";
import { marked } from "npm:marked@4.0.0";

interface ContentItem {
  title: string;
  date: Date;
  slug: string;
  description?: string;
  draft?: boolean;
  url?: string;
  image?: string;
  markdown?: string;
  htmlContent?: string;
  isTruncated: boolean;
}

const routeToTypeMap: Record<string, string> = {
  "/posts": "post",
  "/projects": "project",
  "/logs": "log",
  "/index": "log"
};

const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

export default class ContentListPlugin implements Plugin {
  name = "ContentListPlugin";
  private customRenderer: marked.Renderer;

  constructor() {
    this.customRenderer = new marked.Renderer();
    this.customRenderer.paragraph = (text: any) => {
      if (text.trim().startsWith('<img') && text.trim().endsWith('>')) {
        return text;
      }
      return `<p>${text}</p>`;
    };
  }

  async transform(content: string, context: PluginContext): Promise<{ content: string; metadata?: Metadata }> {
    const contentType = routeToTypeMap[context.route];

    if (contentType) {
      const contentDir = context.contentSources[contentType];

      if (!contentDir) {
        console.error(`ContentListPlugin: Content source not found for ${contentType}`);
        return { content };
      }

      try {
        const items = await this.getContentItems(contentDir);
        const listHtml = this.generateListHtml(items, contentType, context.route);
        content = `${content}\n${listHtml}`;
      } catch (error) {
        console.error(`ContentListPlugin: Error processing ${contentType}:`, error);
      }
    }

    return { content };
  }

  // ... (other methods like getContentItems, generateListHtml, and truncateMarkdown)

  async extendTemplate(templateContext: TemplateContext): Promise<TemplateContext> {
    return templateContext;
  }
}
```

this plugin does several important things:

1. it maps routes to content types, allowing different handling for posts, projects, and logs.
2. it reads and parses markdown files from the appropriate content directory.
3. it generates HTML lists of content items, formatting them differently based on the content type.
4. it handles truncation of content for preview purposes.

i use this plugin in this website to generate lists of posts, projects, and log entries. it's particularly useful for creating index pages that display summaries or links to my content.

to use this plugin, you would register it and add it to your configuration:

```typescript
import ContentListPlugin from "./plugins/ContentListPlugin.ts";
registerPluginType("ContentListPlugin", ContentListPlugin);

export const config: WebsiteConfig = {
  // ... other config options ...
  plugins: [
    {
      name: "ContentListPlugin",
      options: {} 
    },
  ],
  // ... more config options ...
};
```

#### **template helpers**

template helpers are functions you can use in your handlebars templates to perform operations or fetch data. here's an example of defining template helpers:

```typescript
export const config: WebsiteConfig = {
  // ... other config options ...
  templateHelpers: {
    getCurrentRoute: function(context: any) {
      const route = context.route || '/';        
      return route;
    },
    eq: function(a: any, b: any) {
      return a === b;
    },
    formatDate: function(dateString: string) {
      const date = new Date(dateString);
      const monthName = monthNames[date.getMonth()];
      const day = date.getDate();
      const year = date.getFullYear();
      return `${monthName} ${day}, ${year}`;
    }
  }
};
```

you can then use these helpers in your templates. here's how you might use the `getCurrentRoute` helper in a header partial:

```html
<header>
  <h4><strong>tseeley.com</strong></h4>
  <nav> 
    <a href='/'>home</a>
    <a href='/posts'>posts</a>
    <a href='/projects'>projects</a>
    <a href='/about'>about</a>
  </nav>
    
  {{#with (getCurrentRoute this) as |currentRoute|}}
    {{#if (eq currentRoute "/index")}}
      <!-- Special content for the index page -->
    {{/if}}
  {{/with}}
</header>
```

this example uses the `getCurrentRoute` helper to determine the current route and potentially display different content based on that route.

by leveraging plugins and template helpers, you can significantly extend the functionality of your simpl-site project, customizing it to meet your specific needs.

### getting started with simpl-site

now that we've explored the inner workings of simpl-site, let's look at how you can get started with it for your own projects. setting up simpl-site is straightforward, and its flexible configuration options allow you to tailor it to your specific needs.

#### **installation**

to use simpl-site, you first need to have deno installed on your system. once you have deno, you can install simpl-site globally:

```bash
deno install --allow-read --allow-write --allow-net --allow-run -n simpl-site jsr:@iamseeley/simpl-site/cli
```

#### **creating a new simpl-site project**

after installing simpl-site globally, you can create a new project by running:

```bash
simpl-site my-website
```

this command will create a new simpl-site project in a directory called `my-website` in your current location. if you want to create the project in your current directory, you can just run:

```bash
simpl-site .
```

#### **project structure**

after initializing a new simpl-site project, your directory structure will look something like this:

```
my-website/
├── assets/
├── content/
├── plugins/
├── templates/
├── config.ts
└── deno.json
```

#### **configuring your simpl-site**

the simpl-site configuration lives in the `config.ts` file. this is where you define your website's structure and behavior. 

let's look at the key configuration options:

```typescript
import { WebsiteConfig } from "jsr:@iamseeley/simpl-site@1.4.1";

export const config: WebsiteConfig = {
  contentSources: [
    { path: "./content", type: "page", route: "" },
    { path: "./content/posts", type: "post", route: "posts/" },
    { path: "./content/projects", type: "project", route: "projects/" },
  ],
  plugins: [
    // Your plugins configuration
  ],
  defaultContentType: "page",
  templateDir: "./templates",
  customPluginsDir: "./plugins",
  assetsDir: "./assets",
  siteTitle: "My Awesome Site",
  templateHelpers: {
    // Your custom template helpers
  },
  caching: {
    enabled: true,
    excludedRoutes: ['/dynamic-content']
  }
};
```

*note how the `contentSources` are configured:*

- `{ path: "./content", type: "page", route: "" }` this means that files directly in the `./content` directory will be served at the root of your site. for example, `./content/index.md` would be your home page, and `./content/about.md` would be served at `/about`.
- the other content sources for posts and projects are nested under their respective routes.

*here's what each of these options does:*

- `contentSources` defines where your content is located and how it should be routed.
- `plugins` specifies which plugins to use and their configurations.
- `defaultContentType` sets the default type for content if not specified otherwise.
- `templateDir` points to the directory containing your handlebars templates.
- `customPluginsDir` specifies a directory for custom plugins.
- `assetsDir` points to the directory containing static assets.
- `siteTitle` sets a global site title.
- `templateHelpers` allows you to define custom handlebars helpers.
- `caching` configures the caching behavior.

<br/>

#### **adding content**

with simpl-site, you create content using markdown files in the `content/` directory. 

for example:

- `content/index.md` will be your home page
- `content/about.md` will be served at `/about`
- `content/posts/first-post.md` will be served at `/posts/first-post`

*you can use frontmatter to add metadata to your content:*

```markdown
---
title: Welcome to My Site
date: 2023-07-08
tags: [welcome, intro]
---

# Welcome to My Site

This is the content of my home page.
```

#### **creating templates**

simpl-site uses handlebars for templating. 

here's an example of a base layout:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{metadata.title}} | {{siteTitle}}</title>
  <link rel="stylesheet" href="/css/styles.css">
</head>
<body>
  {{> header}}
  <main>
    {{{content}}}
  </main>
  {{> footer}}
</body>
</html>
```

#### **running your site**

once you've created your project, navigate into the project directory:

```bash
cd my-website
```

to start the development server, run:

```bash
deno task dev
```

this will start a local development server, at `http://localhost:8000`, where you can view your site.

#### **deploying your simpl-site**

once you've created your site, you might want to deploy it. there are several options for deploying your simpl-site project to production:

1. **deno deploy**

   deno deploy is the easiest and fastest way to deploy your deno applications. it's specifically designed for deno and offers seamless integration. to deploy your simpl-site project on deno deploy:

   - sign up for a deno deploy account if you haven't already.
   - create a new project in the deno deploy dashboard.
   - link your github repository or upload your project files.
   - set the entry point to your `server.ts` file.
   - configure any necessary environment variables.

   deno deploy will automatically handle the deployment and provide you with a url for your site.

2. **other cloud platforms**

   you can also deploy your simpl-site project to various cloud platforms that support deno:

   - digital ocean: use a droplet or app platform to host your deno application.
   - aws lightsail: set up a vps instance to run your deno server.
   - google cloud run: deploy your deno app as a containerized application.
   - cloudflare workers: with some adjustments, you can run your simpl-site project on cloudflare's edge network.
   - kinsta: offers deno hosting as part of their application hosting services.

   for these platforms, you'll typically need to:
   
   - set up a server or container environment.
   - install deno on the server.
   - copy your project files to the server.
   - run your `server.ts` file using a command like:
     ```typescript
     deno run --allow-read --allow-write --allow-net server.ts
     ```
   - set up a reverse proxy (like nginx) if needed.
   - configure any necessary environment variables.

3. **self-hosting**

   if you prefer to self-host, you can run your simpl-site project on any vps or dedicated server that allows you to install deno. follow these general steps:

   - set up your server and ssh access.
   - install deno on the server.
   - clone or copy your project files to the server.
   - install and configure a process manager like pm2 to keep your app running:
 
    ```typescript
     npm install -g pm2
     pm2 start --interpreter="deno" --interpreter-args="run --allow-read 
     --allow-write --allow-net" server.ts
    ```
   - set up a reverse proxy with nginx or apache to handle https and domain routing.


### simplified dependency management


### jsr


### what's next?
