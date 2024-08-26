---
title: log11
date: 2024-08-26
---

![woods](https://res.cloudinary.com/dcwnusepx/image/upload/v1724702197/tseeley/IMG_1965_zus2hp.jpg)

spent the weekend walking in the woods. 

i'm back home now and i'm ready to write some code.

~ 

it's happening. i'm going to share the web analytics tracker this week. i've been trying to come up with the most pracitcal way to organize the project so val town users only have to fork a few vals and change a few lines of configuration. 

the key vals in the project are:

✦ the tracking script: users need to fork this and add it to the sites they want to track.

✦ the track page view endpoint: this is important because it adds the page view data to the database, so users will need to fork it.

✦ the stream endpoint: this streams data to the application, enabling real-time updates.

✦ the server for the dashboard: this serves the actual web analytics dashboard.

it would be nice if in a val's execution context there existed a user object that you could access to get your own user data. i think there's been discussion about this within the community, and it doesn't exist because of security concerns.

i was trying to think of some workarounds that would make my project more dynamic. i decided to create a `valTownUser` object. the idea is that when users fork the server or other vals, they won’t need to hardcode their usernames or specific URLs. 

instead, by using methods like `getUsername` in the `valTownUser` object, the project dynamically generates the correct URLs and settings based on the user's information. this approach minimizes manual configuration and reduces the potential for errors, making the project more flexible and easier to use. 

now i actually need to implement the methods in the key vals that will be forked to run the web analytics dashboard.

here's the `valTownUser` object: [valTownUser](https://www.val.town/v/iamseeley/valTownUser)

<!-- <div id="valTownUser" data-val-id="b6022d92-63c4-11ef-87c3-de64eea55b61"></div> -->