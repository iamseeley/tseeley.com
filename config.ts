import { WebsiteConfig } from "simpl-site";
import TableOfContentsPlugin from './plugins/TableOfContentsPlugin.ts';
import LastModifiedPlugin from './plugins/LastModifiedPlugin.ts';
import { registerPluginType } from 'simpl-site/plugin-registry';
import ContentListPlugin from "./plugins/ContentListPlugin.ts";

const monthNames = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

// register your plugins
registerPluginType("TableOfContentsPlugin", TableOfContentsPlugin);
// registerPluginType("LastModifiedPlugin", LastModifiedPlugin);
registerPluginType("ContentListPlugin", ContentListPlugin);



// configure your website
  export const config: WebsiteConfig = {
    contentSources: [
      { path: "./content/posts", type: "post", route: "posts/" },
      { path: "./content/projects", type: "project", route: "projects/" },
      { path: "./content/logs", type: "log", route: "logs/" },
      { path: "./content", type: "page", route: "" },
    ],
    plugins: [
      {
        name: "TableOfContentsPlugin",
        options: {
          routes: ["/plugin-example", "/plugin-docs"],
          minDepth: 2,
          maxDepth: 4
        }
      },
      {
        name: "ContentListPlugin",
        options: {} 
      },
    ],
    caching: {
      enabled: true,
      excludedRoutes: ['/plugin-example', '/dynamic-content']
    },   
    defaultContentType: "page",
    templateDir: "./templates",
    customPluginsDir: "./plugins",
    assetsDir: "./assets",
    siteTitle: "tseeley",
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
