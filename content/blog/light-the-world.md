---
title: 'Light the World'
description: ""
published: 2014-07-29
redirect_from: 
            - https://www.josephcatrambone.com/?p=537
categories: "Game, Programming, Status"
hero: ../../../defaultHero.jpg
---
With SAGDC9 wrapped up, it's time to move on. I've considered returning to work on Terminus, the Metroidvania clone I've been working on since my undergrad days at college. An idea popped into my head last Friday, however, and on Saturday I started fiddling around. The game idea is simple, move lights to get rid of the shadows.[![Light The World](http://www.josephcatrambone.com/wp-content/uploads/2014/07/LightTheWorld.gif)](./img/wp-content-uploads-2014-07-LightTheWorld.gif)The working title is "Light the World," but I'll probably replace that with ReLux or something like that in the future.What will LTW have? What are the mechanics?

- Move lights to get rid of the darkness.
- Diffusers, which will shine when shone upon by lights. (Think omnidirectional mirror, since coding the reflection off mirrors is hard.)
- Light goals and dark goals. If these are present in a level, then rather than check to see if the whole thing is illuminated, make sure the lightgoals are lit and the darkgoals are in the shadows.
- Possibly movable polygons. Right now all the geometry is static, which I like, but that is subject to change in some levels.
