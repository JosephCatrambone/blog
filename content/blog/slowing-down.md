---
title: 'Slowing Down'
description: ""
published: 2012-04-06
redirect_from: 
            - https://www.josephcatrambone.com/?p=117
categories: "Game, Status"
hero: ../../../defaultHero.jpg
---
Most of the early fun stuff is now completed.  I post here to enumerate the things that I think remain to be done.  This may have been accomplished with an older update, but this will still let me collect them for public consumption.

- Respawning
- Save Game
- Inventory and Equip
- Fun Augmentations/Items
- More Enemies
- More Levels

Let me speak also about something which is obnoxious to me.  I passed the game to two friends, one of whom had no problems.  The other ran the Jar and saw an issue with a file not found, presumably because his working directory was something like C:\\My Documents.  I've seen this problem every time I write a game.  Not a difficult fix, but one of those little things I hate.  The solution is just to 'pack the build' when you release it.  That means copy everything into the Jar file or packing into the EXE.  Problem with this approach is that when users want to edit stuff, when your artist wants to change a tileset, or when your designer wants to fiddle with a level, you need the long explanation about unpacking and repacking.  Or you can just leave everything in the ./assets folder.  Which leads back to the first problem.
