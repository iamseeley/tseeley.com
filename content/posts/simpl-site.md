---
title: a simple website builder
date: 2024-08-06
description: simpl site is a dynamic website builder built on deno that features markdown, handlebars, and a plugin system for transforming content and extending templates.
draft: true
---

after a month or two of tinkering on [val.town](https://val.town), i found myself really enjoying working with deno (val.town uses the deno runtime to run your js/ts). i've emjoyed deno so much that i decided to create a website builder with it.

it's called [simpl-site](https://github.com/iamseeley/simpl-site): a server-side rendered website builder that let's you create dynamic websites with markdown content, handlebars templates, and a plugin system for transforming content and extending templates. 

it's my first [JSR](https://jsr.io/) package [@iamseeley/simpl-site](https://jsr.io/@iamseeley/simpl-site), and this is my first "technical" blog post!

in this post i want to share a little background on the project, then dive into the code to explore how it all works. i'll wrap it up with next steps for the project.


### i like building the thing that builds the thing

i have a confession to make. sometimes (almost every time), when i decide to redo my personal website i end up making a new website builder for it. so, i spend all my time building the thing to build the thing. i admit this might not be the healthiest / most effective pattern, but i do think it can be pretty rewarding when it all comes together, and you have your website running on something you created.

for this project, what started as a website refresh turned into a journey of exploring server-side rendering, the deno ecosystem, and publishing modules via JavaScript Registry (jsr).

### static site generation to server-side rendering

before diving into simpl-site, i think it's worth mentioning its predecessor: [go-forth](https://github.com/iamseeley/go-forth), a static site generator i wrote in Go. go-forth was my first foray into creating a website builder, and it laid the groundwork for many of the ideas I've implemented in simpl-site. 

go-forth taught me a lot about structuring a site generator, handling markdown content, and managing templates. 

it uses a similar concept of collections for organizing content and uses Go's html/template package for templating. while go-forth is a static site generator, simpl-site takes things a step further by introducing server-side rendering and a more robust plugin system.

### code deep dive

at the heart of the project is the SimplSite class, which sets up the entire process of building and serving a website.

here's a simplified view of its structure:

```javascript
export class SimplSite {
  private plugins: Map<string, Plugin>;
  private contentSources: ContentSource[];
  private markdownProcessor: MarkdownProcessor;
  private templateEngine: TemplateEngine;
  private pageCache: Map<string, { content: string; contentType: string; status: number; lastModified: number }>;

  constructor(private config: WebsiteConfig) {
    // Initialize components
  }

  // Core methods
  async getContent(path: string, type: string): Promise<string> { /* ... */ }
  async processContent(content: string, type: string, route: string): Promise<{ content: string; metadata: Metadata }> { /* ... */ }
  async renderContent(path: string, type: string, route: string): Promise<{ content: string; contentType: string; status: number }> { /* ... */ }
  async handleRequest(path: string): Promise<{ content: string | Uint8Array; contentType: string; status: number }> { /* ... */ }

  // Helper methods
  private initializePlugins(pluginConfigs: PluginConfig[]) { /* ... */ }
  private shouldUseCache(path: string): boolean { /* ... */ }
  private async serveStaticFile(path: string): Promise<{ content: Uint8Array; contentType: string } | null> { /* ... */ }

  // Utility methods
  clearCache() { /* ... */ }
  getCacheStats(): { cacheSize: number; cachedRoutes: string[] } { /* ... */ }
}
```

the SimplSite class handles all aspects of website generation, from content processing to request handling. it also includes methods for plugin management, template rendering, and caching.

### key components of the SimplSite class and their interactions

**Plugin System** Managed through the plugins map and initialized in the constructor:

```javascript
private initializePlugins(pluginConfigs: PluginConfig[]) {
  for (const pluginConfig of pluginConfigs) {
    const PluginClass = getPluginClass(pluginConfig.name);
    const plugin = new PluginClass(pluginConfig.options);
    this.plugins.set(pluginConfig.name, plugin);
  }
}
```

plugins are applied in the content processing pipeline, allowing for content transformation and template extensions.

**contentpProcessing** handled by the `processContent` method which utilizes both the `MarkdownProcessor` and the plugin system:

```javascript
async processContent(content: string, type: string, route: string): Promise<{ content: string; metadata: Metadata }> {
  let { content: processedContent, metadata } = await this.markdownProcessor.execute(content);
  
  // Apply plugin transformations
  for (const plugin of this.plugins.values()) {
    if (plugin.transform) {
      const result = await plugin.transform(processedContent, context);
      processedContent = result.content;
      metadata = { ...metadata, ...result.metadata };
    }
  }

  return { content: processedContent, metadata };
}
```

**template rendering** for template rendering, simpl-site uses handlebars, an html templating engine. the `templateEngine` class encapsulates all the logic for compliling and rendering templates:

```javascript
import Handlebars from "npm:handlebars@4.7.8";
import { TemplateEngineConfig, HelperDelegate, RuntimeOptions } from '../types.ts';
import { join } from "jsr:@std/path@0.224.0";
import { exists } from "jsr:@std/fs@0.224.0";

export class TemplateEngine {
  private handlebars: typeof Handlebars;
  private config: TemplateEngineConfig;
  private compiledTemplates: Map<string, Handlebars.TemplateDelegate> = new Map();

  constructor(config: Partial<TemplateEngineConfig>) {
    this.config = {
      baseDir: 'templates',
      extname: '.hbs',
      layoutsDir: 'layouts/',
      partialsDir: 'partials/',
      defaultLayout: 'base',
      ...config,
    };
    this.handlebars = Handlebars.create();
    this.registerHelpers();
    this.registerPartials();
  }

  // ... other methods ...

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
}
```

the `templateEngine` class allows for customization of template directories, file extensions, and default layouts through simpl-site's config. compiled templates are cached in the `compiledTemplates` map, improving performance for frequently used templates. we don't have to recompile the template on every route change if the content type is the same. 

it also supports handlebar's layout system where content can be wrapped in layout templates.

some other useful features of handlebars are template partials and helpers. they allow for modular template design and custom template functions like `getCurrentRoute` that i use in my personal site in the main layout template to conditionally render the header:

```html
<!-- templates/layouts/base.hbs -->

<body>
  {{> header siteTitle=siteTitle}}
   {{#with (getCurrentRoute this) as |currentRoute|}}
  {{#if (eq currentRoute "/index")}}


<!-- ... -->
```

i can define template helpers in the simpl-site configuration that will make the functions available to all templates:

```javascript
import { WebsiteConfig } from "jsr:@iamseeley/simpl-site@1.4.1";
// ... other imports and plugin registrations ...

export const config: WebsiteConfig = {
  // ... other configuration options ...

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

### simplified dependency management


### jsr


### what's next?
