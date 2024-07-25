---
title: web analytics strategies
date: 2024-07-24
description: common approaches to web analytics.
draft: true
---

### the dilemma: balancing insights and privacy

as i continue to work on my web analytics service, the big question that keeps coming up in my mind is how do i gather meaningful data while respecting user privacy.

to address this, i'm exploring different strategies to understand their benefits and privacy implications. here are some common approaches to web analytics, including the one i'm developing:

### cookie-based tracking

cookie-based tracking uses cookies to store unique identifiers (uids) on users' browsers to track their visits across sessions. this method is reliable and widely supported, but it raises privacy concerns and often requires user consent under regulations such as [gdpr](https://gdpr-info.eu/) and [ccpa](https://oag.ca.gov/privacy/ccpa/regs).

```javascript
// set a cookie with a unique identifier
document.cookie = "uid=" + generateUID() + "; path=/";

// function to generate a unique identifier
function generateUID() {
  return 'xxxx-xxxx-xxxx-xxxx'.replace(/[x]/g, function() {
    return (Math.random() * 16 | 0).toString(16);
  });
}
```

### server-side logging

server-side logging collects data from server logs, including ip addresses, user agents, and referrer urls. this approach does not require client-side scripts and can capture all requests, including those from bots. however, it has limited ability to track unique users and raises privacy concerns due to ip address tracking.

```javascript
// example of server-side logging in node.js
const http = require('http');

http.createServer((req, res) => {
  console.log(`${req.method} ${req.url} ${req.headers['user-agent']} ${req.connection.remoteAddress}`);
  res.writeHead(200, {'Content-Type': 'text/plain'});
  res.end('logged');
}).listen(8080);
```

### fingerprinting

fingerprinting uses a combination of browser and device characteristics, such as user agent and screen resolution, to create a unique identifier. it can uniquely identify users without cookies and works even if users clear cookies, but it raises privacy concerns and is increasingly mitigated by browsers. 

```javascript
// generate a fingerprint based on user agent and screen resolution
function generateFingerprint() {
  return btoa(navigator.userAgent + screen.height + screen.width + screen.colorDepth);
}

// send fingerprint to the server
fetch('https://example.com/track', {
  method: 'POST',
  body: JSON.stringify({ fingerprint: generateFingerprint() }),
  headers: {
    'Content-Type': 'application/json'
  }
});
```

### etag tracking

etag tracking uses the etag header to store a unique identifier in the browser cache. this method can persist across sessions and even after clearing cookies but is considered invasive and less commonly supported.

```javascript
// example of setting an etag header in node.js
const http = require('http');

http.createServer((req, res) => {
  const etag = '12345'; // unique identifier
  res.setHeader('ETag', etag);
  res.writeHead(200, {'Content-Type': 'text/plain'});
  res.end('etag set');
}).listen(8080);
```

### cache-control and last-modified headers (normally notes method)

the normally notes method uses http headers to store a counter in the browser cache without cookies or uids. this approach is highly privacy-focused and does not store any data on the server side. it is limited in the type of data that can be collected. more details on this method can be found [here](https://notes.normally.com/cookieless-unique-visitor-counts/). thanks [andrew](https://healeycodes.com) for sharing this with me! 

```javascript
// example of setting cache-control and last-modified headers in node.js
const http = require('http');

http.createServer((req, res) => {
  const lastModified = new Date().toUTCString();
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Last-Modified', lastModified);
  res.writeHead(200, {'Content-Type': 'text/plain'});
  res.end('headers set');
}).listen(8080);
```

### url parameter tracking

url parameter tracking adds unique identifiers to urls to track users across different pages and sessions. while this method can track users without cookies and is easy to implement, it is visible to users, can be manipulated, and may clutter urls. more on query parameters can be found [here](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams).

```javascript
// add a unique identifier to the url
(function() {
  const uid = 'xxxx-xxxx-xxxx-xxxx'.replace(/[x]/g, function() {
    return (Math.random() * 16 | 0).toString(16);
  });
  window.location.href = window.location.href + '?uid=' + uid;
})();
```

### my approach (no cookies)

my approach uses a javascript-based tracking method that collects site url, path, referrer, browser user agent, os, screen size, language, and a generic "unknown" country, sending this data to the server without using cookies, uids, or fingerprinting. this method respects the "do not track" setting and avoids persistent identifiers.

```javascript
// my web analytics script
(function() {
  if (navigator.doNotTrack !== "1") {
    var img = new Image();
    img.src = 'https://iamseeley-webanalyticstracking.web.val.run/trackPageView?' + 
      'site=' + encodeURIComponent(window.location.origin) +
      '&path=' + encodeURIComponent(window.location.pathname) +
      '&referrer=' + encodeURIComponent(document.referrer) +
      '&browser=' + encodeURIComponent(navigator.userAgent) +
      '&os=' + encodeURIComponent(navigator.platform) +
      '&screenSize=' + encodeURIComponent(window.screen.width + 'x' + window.screen.height) +
      '&language=' + encodeURIComponent(navigator.language) +
      '&country=' + encodeURIComponent('Unknown');
  }
})();
```

my aim is to make a web analytics solution that is both effective and respectful of user privacy. 

### resources

<div class="resources">
  <ul>
    <li><a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies" target="_blank">understanding cookies and tracking</a></li>
    <li><a href="https://en.wikipedia.org/wiki/Logfile" target="_blank">web server logs</a></li>
    <li><a href="https://panopticlick.eff.org/" target="_blank">see how trackers view your browser</a></li>
    <li><a href="https://www.fastly.com/blog/etags-what-they-are-and-how-to-use-them/" target="_blank">etags and web tracking</a></li>
  </ul>
</div>