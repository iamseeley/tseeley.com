---
title: a simple website builder
date: 2024-08-14
description: simpl site is a dynamic website builder built on deno that features markdown, handlebars, and a plugin system for transforming content and extending templates.
draft: false
---

after a month or two of tinkering on [val.town](https://val.town), i found myself really enjoying working with Deno (val.town uses the Deno runtime to run your JS/TS). i've enjoyed Deno so much, i decided to make a website builder with it.

it's called [simpl-site](https://github.com/iamseeley/simpl-site): it lets you create dynamic websites with markdown content, handlebars templates, and a plugin system for transforming content and extending templates.

this website is powered by simpl-site!

it's my first [JavaScript Registrty](https://jsr.io/) (JSR) package: [@iamseeley/simpl-site](https://jsr.io/@iamseeley/simpl-site), and this is my first "technical" blog post!

in this post i want to share a little background on the project, then dive into the code to explore how it all works. i'll wrap it up with next steps for the project.


### i like building the thing that builds the thing

i have a confession to make. 

i redo my personal website way too often, and sometimes when i do... i end up making the tool that builds the site. i spend all my time building the 'thing' that builds the 'thing.' i admit this might not be the healthiest / most efficient use of my time, but i think it can be pretty rewarding when it all comes together, and you get your website running with something you've created.

for this project, what started as a website refresh turned into an exploration of server-side rendering, the Deno ecosystem, and publishing modules via JSR. 

### static site generation to server-side rendering

i think it's worth mentioning simpl-site's predecessor: [go-forth](https://github.com/iamseeley/go-forth), a static site generator i wrote in Go. go-forth was my first adventure in creating a website builder, and it laid the groundwork for many of the ideas i've implemented in simpl-site. 

go-forth taught me a lot about structuring a site generator, handling markdown content, and managing templates. 

it uses a similar concept of collections for organizing content and uses Go's [html/template](https://pkg.go.dev/html/template) package for templating. while go-forth is a static site generator, simpl-site takes a different approach by implementing server-side rendering and a plugin system.

### simpl-site overview

before we get into the code specifics, let's take a high-level look at how simpl-site is built and how it leverages some of Deno's features.

**key components and technologies:**

1. **Deno**:
 simpl-site is built on [Deno](https://deno.com/), which provides a secure runtime for JavaScript and TypeScript. this allows us to use modern ES6+ features and TypeScript out of the box, without need for transpilation.

2. **Deno.serve**: 
simpl-site uses [Deno's serve function](https://docs.deno.com/api/deno/~/Deno.serve), which provides a minimal way to create an HTTP server. this function handles incoming requests and routes them to the appropriate handler.

3. **file system operations**:
 simpl-site uses Deno's built-in APIs for file system operations ([Deno.readTextFile](https://docs.deno.com/api/deno/~/Deno.readTextFile), [Deno.writeTextFile](https://docs.deno.com/api/deno/~/Deno.writeTextFile), etc.) to read markdown content, templates, and other assets.

4. **markdown processing**: 
simpl-site uses [Marked](https://github.com/markedjs/marked) for markdown parsing and compiling.

5. **Handlebars templating**:
 for HTML templating, simpl-site uses [Handlebars](https://handlebarsjs.com/). this allows for dynamic content insertion and reusable template components.

6. **plugin system**:
 simpl-site implements a plugin architecture that allows users to transform content or extend templates, providing a flexible way to customize the site generation process.

7. **caching**: 
to improve performance, simpl-site includes a caching system. this reduces the need to re-process unchanged content on every request.

8. **static file serving**:
 for assets like CSS, JavaScript, and images, simpl-site includes functionality to serve static files.

9. **TypeScript**: 
 simpl-site is written in TypeScript.

each request goes through a pipeline of processing steps: routing, content retrieval, markdown processing with marked, plugin transformations, template rendering, and finally, response sending.

### code deep dive: anatomy of a page request

let's walk through the lifecycle of a page request in simpl-site, following the process from initial request to final rendered output. this will take us through the key components of the `SimplSite` class and show how they work together to build a dynamic website.

#### **1. handling the incoming request**

when a request comes in, it's first handled by the `handleRequest` method:

```typescript
async handleRequest(path: string): Promise<{ content: string | ReadableStream<Uint8Array>; 
contentType: string; status: number; size?: number }> {
    // Check for static file first
    const staticFile = await this.serveStaticFile(path);
    if (staticFile) {
      return { ...staticFile, status: 200 };
    }

    // If not a static file, proceed with content rendering
    path = path.replace(/^\//, '');
    if (path === '') {
      path = 'index';
    }

    const originalPath = path;
    path = path.endsWith('.md') ? path : path + '.md';

    let result;
    for (const source of this.contentSources) {
      if (originalPath.startsWith(source.route)) {
        const contentPath = path.slice(source.route.length);
        result = await this.renderContent(contentPath, source.type,
        '/' + originalPath);
        break;
      }
    }

    if (!result) {
      result = await this.renderContent(path, this.defaultContentType, 
      '/' + originalPath);
    }

    return result;
  }
```

this method first checks if the request is for a static file using the `serveStaticFile` method. if it is, it serves the file directly. if not, it determines the appropriate content source and calls `renderContent` to generate the dynamic content.

#### **2. serving static files**

the `serveStaticFile` method efficiently handles static file requests:

```typescript
private async serveStaticFile(path: string): Promise<{ content: ReadableStream<Uint8Array>; contentType: string; size: number } | null> {
  const relativePath = path.startsWith('/') ? path.slice(1) : path;
  if (!this.staticFilePaths.has(relativePath)) {
    return null;
  }

  const fullPath = join(this.assetsDir, relativePath);
  try {
    const file = await Deno.open(fullPath, { read: true });
    const fileInfo = await file.stat();
    const mimeType = contentType(extname(fullPath)) || "application/octet-stream";
    
    return { 
      content: file.readable, 
      contentType: mimeType,
      size: fileInfo.size
    };
  } catch (error) {
    console.error(`Error opening static file ${fullPath}:`, error);
    return null;
  }
}
```

this method checks if the requested file exists in the pre-cached `staticFilePaths` set. if it does, it opens the file as a readable stream, determines its MIME type, and returns the necessary information for serving the file.

#### **3. rendering content**

if the request is for dynamic content, the `renderContent` method is called. this method orchestrates the entire process of retrieving, processing, and rendering the content:

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

within `renderContent`, three crucial steps occur:

#### **3.1 retrieving the content**

the `getContent` method is called to fetch the raw content:

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

#### **3.2 processing the content**

once the raw content is retrieved, the `processContent` method is called to process it:

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


#### **3.3 rendering the template**

after processing the content, `renderContent` calls the `render` method of the TemplateEngine class to render the final HTML:

```typescript
const renderedContent = await this.templateEngine.render(type, templateContext);
```

the TemplateEngine's `render` method handles the actual rendering:

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

#### **4. caching for performance**

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

by following this process for each request, simpl-site can efficiently handle requests for both static and dynamic content, apply plugins, render templates, and serve the resulting pages or files.

*for more visual readers, here's a diagram depicting the process*

<iframe src="https://iamseeley-simplsiterequestgraph.web.val.run/"></iframe>


### extending simpl-site: plugins and template helpers

one of the most useful features of simpl-site is its extensibility through plugins and template helpers.

#### **what can plugins do?**

plugins in simpl-site can:
1. **transform content**: modify your markdown content before it's rendered

2. **extend templates**: add new data or functions to your handlebars templates

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

let's look at how to create a plugin. we'll use a simplified version of the `ContentListPlugin` that i use in this website as an example. this plugin generates HTML lists of content items.

here's a basic implementation of the `ContentListPlugin`:

```typescript
import type { Plugin, PluginContext } from "simpl-site";

export default class ContentListPlugin implements Plugin {
  name = "ContentListPlugin";

  async transform(content: string, context: PluginContext): Promise<{ content: string }> {
    // Check if we're on a route that should display a content list
    if (context.route === "/posts" || context.route === "/projects") {
      // Generate a list of content items (simplified for this example)
      const listHtml = this.generateListHtml(context.route);
      
      // Append the list to the existing content
      content = `${content}\n${listHtml}`;
    }
    return { content };
  }

  private generateListHtml(route: string): string {
    // This is a simplified example. In a real plugin, you'd fetch and process actual content.
    const items = [
      { title: "Item 1", date: new Date() },
      { title: "Item 2", date: new Date() },
    ];

    let html = "<ul>";
    for (const item of items) {
      html += `<li>${item.title} - ${item.date.toDateString()}</li>`;
    }
    html += "</ul>";

    return html;
  }
}
```

this plugin demonstrates several key concepts:

1. **implementing the plugin interface**: the class implements the `Plugin` interface from `SimplSite`, which requires a `name` property and a `transform` method.

2. **the transform method**: it receives the current content and a context object, and returns modified content. here's what it does:
   - it checks the current route to decide whether to add a content list.
   - if appropriate, it generates an HTML list and appends it to the existing content.
   - it returns an object with the modified content.

3. **using the context**: the `PluginContext` provides information about the current request, including the route. in a more complex plugin, you could use other properties of the context, such as `contentSources` to access actual content files.

4. **generating Ccntent**: the `generateListHtml` method is a simplified example of how you might generate HTML to inject into the page. in a real plugin, this would involve reading and processing actual content files.

to use this plugin, you would register it and add it to your simpl-site configuration:

```typescript
import ContentListPlugin from "./plugins/ContentListPlugin.ts";
import { registerPluginType } from "simpl-site";

registerPluginType("ContentListPlugin", ContentListPlugin);

export const config = {
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

this example demonstrates the basic structure of a simpl-site plugin. the `transform` method is called for each page, allowing you to modify or extend the content as needed. by implementing more complex logic in the `transform` method and utilizing the full `PluginContext`, you can create even more powerful plugins!

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

### a simplSite instance

you can visit the [README](https://github.com/iamseeley/simpl-site) for detailed setup and deployment instructions!

#### **project structure**

after initializing a new simpl-site project, your directory structure will look something like this:

```
my-website/
├── assets/
├── content/
├── plugins/
├── templates/
├── config.ts
├── server.ts
└── deno.json
```

#### **configuring your simpl-site**

the simpl-site configuration lives in the `config.ts` file. this is where you define your website's structure and behavior. 

let's look at the key configuration options:

```typescript
import { WebsiteConfig } from "simpl-site";

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
  siteTitle: "My Simpl Site",
  templateHelpers: {
    // Your custom template helpers
  },
  caching: {
    enabled: true,
    excludedRoutes: ['/dynamic-content']
  }
};
```

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

*note how the `contentSources` are configured:*

simpl-site uses content sources to define how files are served. for example:
`{ path: "./content", type: "page", route: "" }` means that files directly in the `./content` directory will be served at the root of your site. 

other content sources, for different content types like posts and projects, can be nested under their respective routes.

#### **adding content**

to create content add markdown files in the `content/` directory. 

here are some examples:

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

### JSR for distribution

i published the simpl-site module to [JSR](https://jsr.io/) because it simplifies the development and distribution process for TypeScript projects. JSR is designed to work with multiple runtimes, including Node.js, Deno, and browsers, but still maintains backwards compatibility with npm.

the combination of TypeScript-first development, multi-runtime support, and npm compatibility made JSR an ideal choice for distributing simpl-site.

### Smallweb

i'm a big fan of [pomdtr's](https://github.com/pomdtr) project [Smallweb](https://github.com/pomdtr/smallweb). it's a web server based on deno, that lets you create your own little self-hosted serverless platform. you "host websites from your internet folders" (Smallweb maps domains to folders in your filesystem). 

simpl-site is compatible with Smallweb! by adding the --smallweb flag to simpl-site's init commands, you initialize a simpl-site project with a `main.ts` file that's structured appropriately for serving a website via Smallweb. 

pomdtr simplified the integration further and made simpl-site a Smallweb plugin.

if you already have Smallweb installed you can navigate to your Smallweb folder, and run the following command:

```bash
deno install -Agf jsr:@iamseeley/simpl-site/smallweb-simpl-site
smallweb simpl-site
```

this sets up a simpl-site project within your Smallweb environment.


### what's next?

simpl-site is mostly a personal project, but it's also a tool i hope others will find useful. i'd love for people to try it and give me feedback so i can improve it! 

moving forward, i'm planning on experimenting with new plugins, more complex sites, more advanced ssr techniques, and implementing a testing suite.

stay tuned for updates!