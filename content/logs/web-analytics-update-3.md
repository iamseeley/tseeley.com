---
title: web analytics update
date: 2024-08-01
---

after some bumps in the road, mainly me trying to wrap my head around when to use react suspense and lazy loading, my current thinking is that for a real-time dashboard where data needs to be displayed immediately across all components, i should opt for minimal loading states while the site connects to the data stream. so, that's what i've implemented.

<video controls><source src="https://res.cloudinary.com/dcwnusepx/video/upload/v1722547609/tseeley/1loadingstate_zp2v5q.mp4"></video>

rather than managing the dashboard state and data within the home dashboard component, i decided to use the react context api to create an analytics provider. it centralizes the data management for the dashboards (fetching, processing, and distribution), and it's where we connect to the http data stream. the analytics provider fetches and processes the data once, making it available to all its child components. if i have other routes that use the analytics data there's no need to refetch the data.

i've been dealing with the mess of components i made for the home dashboard, and it was time to clean it up and work on the site specific dashboard. my initial intention was to have a single dashboard that could be specified as either a site dashboard or an overall stats dashboard, and i finally got around to doing it. i moved all the home dashboard components into a single Dashboard component that checks whether it is a site dashboard or overall stats dashboard. 

as i'm writing this i realized i haven't setup a check to verify if the sitename actually matches a site in the database. right now, if i add /pizza to the url it tries to load the pizza dashboard and when it can't it loads the overall stats dashboard but leaves "pizza" as the dashboard name... oops, need to fix that.

slowly making progress! over the next day or two, i'm going to focus more on improving the individual components within the dashboard, and maybe change up some of the ways i'm displaying site data.

current state:

<video controls><source src="https://res.cloudinary.com/dcwnusepx/video/upload/v1722544665/tseeley/4webdashdemo_sip7ll.mp4"></video>
