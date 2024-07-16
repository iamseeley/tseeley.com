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
  image?: string;
  markdown?: string;
  htmlContent?: string;
  isTruncated: boolean;
}

const routeToTypeMap: Record<string, string> = {
  "/posts": "post",
  "/projects": "project",
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
    console.log(`ContentListPlugin: Processing route ${context.route}`);
    console.log(`ContentListPlugin: Available content sources:`, context.contentSources);

    const contentType = routeToTypeMap[context.route];
    console.log(`ContentListPlugin: Determined content type: ${contentType}`);

    if (contentType) {
      const contentDir = context.contentSources[contentType];

      if (!contentDir) {
        console.error(`ContentListPlugin: Content source not found for ${contentType}`);
        return { content };
      }

      console.log(`ContentListPlugin: Processing ${contentType} from ${contentDir}`);

      try {
        const items = await this.getContentItems(contentDir);
        console.log(`ContentListPlugin: Retrieved ${items.length} items`);
        const listHtml = this.generateListHtml(items, contentType);
        content = `${content}\n${listHtml}`;
      } catch (error) {
        console.error(`ContentListPlugin: Error processing ${contentType}:`, error);
      }
    } else {
      console.log(`ContentListPlugin: No matching content type for route ${context.route}`);
    }

    return { content };
  }

  private async getContentItems(contentDir: string): Promise<ContentItem[]> {
    const items: ContentItem[] = [];
    console.log(`ContentListPlugin: Searching for content in ${contentDir}`);

    try {
      for await (const entry of walk(contentDir, { maxDepth: 1 })) {
        if (entry.isFile && extname(entry.path) === ".md" && basename(entry.path) !== "index.md") {
          console.log(`ContentListPlugin: Processing file ${entry.path}`);
          const content = await Deno.readTextFile(entry.path);
          const frontmatterMatch = content.match(/^---\n([\s\S]*?)\n---/);
          if (frontmatterMatch) {
            const frontmatter = frontmatterMatch[1];
            const metadata = parseYaml(frontmatter) as Record<string, unknown>;
            const markdownBody = content.slice(frontmatterMatch[0].length).trim();
            const { truncatedMarkdown, isTruncated } = this.truncateMarkdown(markdownBody, 600);
            const htmlContent = marked(truncatedMarkdown, { renderer: this.customRenderer });
            console.log(`ContentListPlugin: Markdown parsed, HTML length: ${htmlContent.length}`);
            items.push({
              title: metadata.title as string || "Untitled",
              date: new Date(metadata.date as string),
              slug: basename(entry.path, ".md"),
              description: metadata.description as string,
              image: metadata.image as string,
              markdown: markdownBody,
              htmlContent: htmlContent,
              isTruncated: isTruncated
            });
          }
        }
      }
    } catch (error) {
      console.error(`ContentListPlugin: Error reading directory ${contentDir}:`, error);
    }
    return items;
  }

  private generateListHtml(items: ContentItem[], contentType: string): string {
    console.log(`ContentListPlugin: Generating HTML for ${items.length} ${contentType} items`);
    const currentYear = new Date().getFullYear();
    let currentListYear: number | null = null;
    let listHtml = '';

    items.sort((a, b) => b.date.getTime() - a.date.getTime());

    items.forEach(item => {
      const formattedDate = `${monthNames[item.date.getMonth()]} ${item.date.getDate()}`;
      if (contentType === 'post') {
        const itemYear = item.date.getFullYear();
        if (itemYear !== currentListYear && itemYear !== currentYear) {
          listHtml += `<h2>${itemYear}</h2>\n`;
          currentListYear = itemYear;
        }

        listHtml += `<li>
          <a href="/${contentType}s/${item.slug}">${item.title}</a>
          <span class="date"><em>${formattedDate}</em></span>
        </li>\n`;
      } else if (contentType === 'project') {
        listHtml += `<li>
          <a href="/${contentType}s/${item.slug}">${item.title}</a>
          ${item.description ? `<p>${item.description}</p>` : ''}
        </li>\n`;
      } else if (contentType === 'log') {
        listHtml += `<li>
          <h3>${item.title} <span class="date"><em>${formattedDate}</em></span></h3>
          <div class="log-content">
            ${item.htmlContent || 'No content available'}
            ${item.isTruncated ? `<a href="/logs/${item.slug}">...</a>` : ''}
          </div>
        </li>\n`;
      }
    });

    console.log(`ContentListPlugin: Generated HTML list with ${items.length} items`);
    return `<ul class="content-list">\n${listHtml}</ul>`;
  }

  private truncateMarkdown(markdown: string, maxLength: number): { truncatedMarkdown: string, isTruncated: boolean } {
    if (markdown.length <= maxLength) {
      return { truncatedMarkdown: markdown, isTruncated: false };
    }
    
    let truncated = markdown.slice(0, maxLength);
    // Ensure we don't cut off in the middle of a word
    truncated = truncated.slice(0, truncated.lastIndexOf(' '));
    return { truncatedMarkdown: truncated, isTruncated: true };
  }

  async extendTemplate(templateContext: TemplateContext): Promise<TemplateContext> {
    return templateContext; // We're not modifying the template context
  }
}