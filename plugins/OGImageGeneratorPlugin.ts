import type { Plugin, PluginContext, TemplateContext } from "jsr:@iamseeley/simpl-site";
import { createCanvas, CanvasRenderingContext2D, EmulatedCanvas2D } from "https://deno.land/x/canvas@v1.4.1/mod.ts";

export default class OGImageGeneratorPlugin implements Plugin {
  name = "OGImageGeneratorPlugin";
  private fontFamily = "Arial, sans-serif"; // Fallback to system fonts
  private fontPath = 'assets/fonts/Aleo-Regular.ttf';

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

      // Set up text style
      const fontSize = 60;
      ctx.font = `bold ${fontSize}px ${this.fontFamily}`;
      ctx.fillStyle = "black";
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";

      // Calculate maximum width for text wrapping
      const maxWidth = width - 100; // 50px padding on each side

      // Center and wrap the text
      this.wrapText(ctx, metadata.title, width / 2, height / 2, maxWidth, fontSize * 1.2);

      // Convert the canvas to a base64 data URL
      const dataUrl = canvas.toDataURL("image/png");
      return dataUrl;
    } catch (error) {
      console.error(`Error during OG image generation: ${error.message}`);
      throw error;
    }
  }

  private wrapText(ctx: CanvasRenderingContext2D, text: string, x: number, y: number, maxWidth: number, lineHeight: number) {
    const words = text.split(' ');
    let line = '';
    let testLine = '';
    const lines: string[] = [];

    for (let n = 0; n < words.length; n++) {
      testLine += `${words[n]} `;
      const metrics = ctx.measureText(testLine);
      const testWidth = metrics.width;
      if (testWidth > maxWidth && n > 0) {
        lines.push(line);
        line = `${words[n]} `;
        testLine = `${words[n]} `;
      } else {
        line = testLine;
      }
    }
    lines.push(line);

    // Calculate total height of text
    const totalHeight = lines.length * lineHeight;
    // Calculate starting Y position to center the text block
    let startY = y - (totalHeight / 2) + (lineHeight / 2);

    for (let i = 0; i < lines.length; i++) {
      ctx.fillText(lines[i], x, startY);
      startY += lineHeight;
    }
  }
}