---
title: log11
date: 2024-08-26
---

![woods](https://res.cloudinary.com/dcwnusepx/image/upload/v1724702197/tseeley/IMG_1965_zus2hp.jpg)

spent the weekend walking in the woods. 

i'm back home now and i'm ready to write some code.

~ 

it's happening. i'm going to share the web analytics tracker this week. i've been trying to come up with a way to organize the project so val town users only have to fork a few vals and change a few lines of configuration. 

i imagine this will get easier when val town introduces a ["projects"](https://github.com/val-town/val-town-product/discussions/139) concept for organizing vals. it will probably come with more advanced sharing capabilities for multi-val projects?!

the key vals in the project are:

✦ the tracking script: users need to fork this and add it to the sites they want to track.

✦ the track page view endpoint: this is important because it adds the page view data to the database, so users will need to fork it.

✦ the stream endpoint: this streams data to the application, enabling real-time updates.

✦ the server for the dashboard: this serves the actual web analytics dashboard.

it would be nice if a val's execution context contained a user object that you could access to get your own user data. i think there's been discussion about this within the community, and it doesn't exist because of security concerns.

i was trying to think of some workarounds that would let me access user data without hardcoding anything. i decided to create the `valTownUser` object to interface with the val town API, serving as a hub for user-specific operations and data retrieval. 

now, when users fork a val that uses the methods on the `valTownUser` object, they don't need to change anything—it just works in their val town environment. this eliminates the need for manual configuration, automatically adapting to each user's context and simplifying the process of setting up and running forked vals.

now i actually need to implement the methods in the key vals that will be forked to run the web analytics dashboard.

here's the `valTownUser` object: [valTownUser](https://www.val.town/v/iamseeley/valTownUser)

<!-- <div id="valTownUser" data-val-id="b6022d92-63c4-11ef-87c3-de64eea55b61"></div> -->