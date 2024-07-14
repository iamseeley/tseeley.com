import type { Plugin, Metadata, PluginContext, TemplateContext } from "jsr:@iamseeley/simpl-site";
import { walk } from "https://deno.land/std/fs/mod.ts";
import { extname, basename } from "https://deno.land/std/path/mod.ts";
import { parse as parseYaml } from "https://deno.land/std/yaml/mod.ts";

interface ContentItem {
  title: string;
  date: Date;
  slug: string;
  description?: string;
}

const routeToTypeMap: Record<string, string> = {
  "/posts": "post",
  "/projects": "project"
};

const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

export default class ContentListPlugin implements Plugin {
  name = "ContentListPlugin";

  async transform(content: string, context: PluginContext): Promise<{ content: string; metadata?: Metadata }> {
    console.log(`ContentListPlugin: Processing route ${context.route}`);
    console.log(`ContentListPlugin: Available content sources:`, context.contentSources);

    const contentType = routeToTypeMap[context.route];
    if (contentType) {
      const contentDir = context.contentSources[contentType];

      if (!contentDir) {
        console.error(`ContentListPlugin: Content source not found for ${contentType}`);
        return { content };
      }

      console.log(`ContentListPlugin: Processing ${contentType} from ${contentDir}`);

      try {
        const items = await this.getContentItems(contentDir);
        const listHtml = this.generateListHtml(items, contentType);
        content = `${content}\n${listHtml}`;
      } catch (error) {
        console.error(`ContentListPlugin: Error processing ${contentType}:`, error);
      }
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
            const metadata = parseYaml(frontmatterMatch[1]) as Record<string, unknown>;
            items.push({
              title: metadata.title as string || "Untitled",
              date: new Date(metadata.date as string),
              slug: basename(entry.path, ".md"),
              description: metadata.description as string,
            });
          }
        }
      }
    } catch (error) {
      console.error(`ContentListPlugin: Error reading directory ${contentDir}:`, error);
    }

    return items.sort((a, b) => b.date.getTime() - a.date.getTime());
  }

  private generateListHtml(items: ContentItem[], contentType: string): string {
    const currentYear = new Date().getFullYear();
    let currentListYear: number | null = null;
    let listHtml = '';

    items.forEach(item => {
      if (contentType === 'post') {
        const itemYear = item.date.getFullYear();
        if (itemYear !== currentListYear && itemYear !== currentYear) {
          listHtml += `<h2>${itemYear}</h2>\n`;
          currentListYear = itemYear;
        }
        const formattedDate = `${monthNames[item.date.getMonth()]} ${item.date.getDate()}`;
        listHtml += `<li>
          <a href="/${contentType}s/${item.slug}">${item.title}</a>
          <span class="date"><em>${formattedDate}</em></span>
        </li>\n`;
      } else if (contentType === 'project') {
        listHtml += `<li>
          <a href="/${contentType}s/${item.slug}">${item.title}</a>
          ${item.description ? `<p>${item.description}</p>` : ''}
        </li>\n`;
      }
    });

    return `<ul class="content-list">\n${listHtml}</ul>`;
  }

  async extendTemplate(templateContext: TemplateContext): Promise<TemplateContext> {
    return templateContext; // We're not modifying the template context
  }
}