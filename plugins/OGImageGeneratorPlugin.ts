import type { Plugin, PluginContext, TemplateContext } from "jsr:@iamseeley/simpl-site";
import { Canvas, createCanvas } from "https://deno.land/x/canvas@v1.4.1/mod.ts";

export default class OGImageGeneratorPlugin implements Plugin {
  name = "OGImageGeneratorPlugin";

  constructor(private options: Record<string, unknown> = {}) {}

  async transform(content: string, context: PluginContext): Promise<{ content: string }> {
    return { content };
  }

  async extendTemplate(templateContext: TemplateContext): Promise<TemplateContext> {
    const title = templateContext.metadata.title as string;

    if (title) {
      console.log(`Generating OG image for title: ${title}`);
      try {
        const ogImageDataUrl = await this.generateOGImage({ title });
        console.log(`Generated OG image data URL.`);
        templateContext.ogImage = ogImageDataUrl;
      } catch (error) {
        console.error(`Error generating OG image: ${error.message}`);
      }
    } else {
      console.error('Title is missing in the template context metadata.');
    }

    return templateContext;
  }

  private async generateOGImage(metadata: { title: string }): Promise<string> {
    const width = 1200;
    const height = 630;

    try {
      const canvas = createCanvas(width, height);
      const ctx = canvas.getContext("2d");

      // Set background
      ctx.fillStyle = "#f05b32";
      ctx.fillRect(0, 0, width, height);

      

      // Add title
      ctx.font = "bold 60px Arial";
      ctx.fillStyle = "black";
      ctx.fillText(metadata.title, 50, 100, width - 100);

      // Convert the canvas to a base64 data URL
      const dataUrl = canvas.toDataURL("image/png");

      return dataUrl;
    } catch (error) {
      console.error(`Error during OG image generation: ${error.message}`);
      throw error;
    }
  }
}
