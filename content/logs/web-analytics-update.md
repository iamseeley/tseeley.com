---
title: live updates + visualizations
date: 2024-07-24
---

quick update on the web analytics service!

<aside>i'm working on a minimal web analytics service and dashboard to view all my websites' data in one place!</aside>

✦ changed the db schema to store aggregated data for each site instead of adding a new item for each page view.

✦ added origin configuration. now it only works with specified sites.

✦ created an http stream endpoint that streams the analytics data to the dashboard. this means the dashboard is now live and updates in real-time! well, i set it to update every five seconds for now.

✦ decided to add location data by country. tried a few different libraries to get the country from the ip address, but i couldn't get them to work. i whipped up [ipToCountry](https://www.val.town/v/iamseeley/ipToCountry), and it's good enough for now.

✦ added some bar charts for the browser, os, screen size, location, and views overall data.

✦ added a timestamps table to record time data for specific paths (it's made it easier to calculate sites' daily, weekly, and monthly view percentages).

i'm going to add a site page view to show all the data for specific sites!

here's a preview 

<video controls><source src="https://res.cloudinary.com/dcwnusepx/video/upload/v1721938180/tseeley/1demowebanalytics_ufauun.mp4"></video>