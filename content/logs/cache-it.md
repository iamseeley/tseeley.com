---
title: cache it?
date: 2024-07-22
image: https://cdn.mos.cms.futurecdn.net/SMDYhDUZcKuX9YV9vCrvEk-1200-80.jpg
---

playing around with caching pages and templates for this site. 

<aside>this site uses <a href="https://github.com/iamseeley/simpl-site" target="_blank">simpl-site</a>. simpl-site is a server-side rendered website builder. it enables the creation of dynamic websites using markdown content, handlebars templates, and a plugin system. this website is powered by simpl-site.</aside>

it seems silly to go through the process of retrieving the handlebars template and markdown content then rendering the html on every page load.

i implemented a page cache in the [simplSite](https://github.com/iamseeley/simpl-site/commit/9287c696421c6e32608ad0a9f8ff5bd89bc26472) class that caches the final rendered html for each page. so, on subsequent requests, it serves the cached content if it's available.

also, i added template caching to the templateEngine class to cache the handlebars templates!

with the help of claude i made a benchmark script that ran 1000 iterations for both cached and uncached scenarios. without caching the average response time was 0.444ms, with caching enabled the average response time dropped to 0.078ms.

here's the script: 

```typescript
import { SimplSite } from "../mod.ts";
import { config } from "./config.ts";

function calculatePercentile(sortedArray: number[], percentile: number): number {
  const index = Math.ceil((percentile / 100) * sortedArray.length) - 1;
  return sortedArray[index];
}

async function runBenchmark(simplSite: SimplSite, path: string, warmupIterations: number, measuredIterations: number) {
  const times: number[] = [];

  // Warm-up phase
  for (let i = 0; i < warmupIterations; i++) {
    await simplSite.handleRequest(path);
  }

  // Measurement phase
  for (let i = 0; i < measuredIterations; i++) {
    const startTime = performance.now();
    await simplSite.handleRequest(path);
    const endTime = performance.now();
    times.push(endTime - startTime);
  }

  times.sort((a, b) => a - b);

  return {
    average: times.reduce((a, b) => a + b, 0) / times.length,
    median: calculatePercentile(times, 50),
    min: times[0],
    max: times[times.length - 1],
    p95: calculatePercentile(times, 95),
    p99: calculatePercentile(times, 99),
  };
}

async function main() {
  const warmupIterations = 50;
  const measuredIterations = 1000;
  const testPath = "/plugin-example";

  // Test with caching enabled
  const configWithCache = { ...config, caching: { enabled: true } };
  const siteWithCache = new SimplSite(configWithCache);
  console.log("Running benchmark with caching enabled...");
  const resultsWithCache = await runBenchmark(siteWithCache, testPath, warmupIterations, measuredIterations);

  // Clear cache and test with caching disabled
  siteWithCache.clearCache();
  const configWithoutCache = { ...config, caching: { enabled: false } };
  const siteWithoutCache = new SimplSite(configWithoutCache);
  console.log("Running benchmark with caching disabled...");
  const resultsWithoutCache = await runBenchmark(siteWithoutCache, testPath, warmupIterations, measuredIterations);

  console.log("Results with caching:");
  console.log(resultsWithCache);
  console.log("Results without caching:");
  console.log(resultsWithoutCache);

  const speedup = resultsWithoutCache.average / resultsWithCache.average;
  console.log(`Caching provides a ${speedup.toFixed(2)}x speedup on average`);

  if (resultsWithCache.max > resultsWithoutCache.max) {
    console.log("Note: The maximum response time with caching is higher than without caching.");
    console.log("This could be due to initial cache population or system variability.");
    console.log("Consider the median and 95th percentile for a more representative comparison.");
  }
}

main();
```

after learning caching provided a 5.69x speedup on average i decided to add caching configuration to simpl-site.

```typescript
caching?: {
  enabled: boolean;
  excludedRoutes?: string[];
};
```

this might end up turning into a full post on server-side / client-side caching strategies.

i'll probably add static asset caching next!
