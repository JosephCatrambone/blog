---
title: 'Tag: A Public Game Design Document'
description: ""
published: 2021-08-07
redirect_from: 
            - https://www.josephcatrambone.com/?p=1341
categories: "Game, Programming"
hero: ../../../defaultHero.jpg
---
<!-- wp:paragraph -->

Happy Saturday! Let's make a game.

<!-- /wp:paragraph -->

<!-- wp:heading -->

## Why

<!-- /wp:heading -->

<!-- wp:paragraph -->

Freerunning and Parkour are pretty great. Speedruns of TitanFall 2 are pretty great. Tag is pretty great. A game that's as mechanically simple as tag means I don't have to mess with any tricky systems or item pickups. Get to focus on (1) movement systems that are easy and satisfying, and (2) making levels and maps that are fun to play. Feels like a good fit.

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

**One sentence tagline: Parkour tag online with friends.**

<!-- /wp:paragraph -->

<!-- wp:heading -->

## MVP Deliverables

<!-- /wp:heading -->

<!-- wp:list -->

- Step 0: Fun movement, even over network.
- Step 1: Easy matching with friends. (And the ability to kick bots/hackers.)
- Step 2: Enjoyable environments.

<!-- /wp:list -->

<!-- wp:heading -->

## Open questions

<!-- /wp:heading -->

<!-- wp:list -->

- Start by defining movement or by defining network play?

  - I'm not good at (low-latency) UDP network stuff. I've only really worked in TCP-land.
  - Doing movement system first might mean changing it for network play, especially if we need to do things like input-replaying.

- Freeze-tag? Horde mode? Face-off? All?

  - Freeze tag: tagging by the opponent means you're frozen and can only witness until you're unfrozen. Tagger wins when all people are frozen.
  - Horde mode: tagging means your color switches from blue team to red ream. Blue team wins when time runs out. Red team wins when there are no blue left.
  - Face-off: Freeze tag, but when you're tagged you're out. Tagger wins when there are no people left. Runners win when the time runs out.

<!-- /wp:list -->

<!-- wp:heading -->

## Proposed Plan

<!-- /wp:heading -->

<!-- wp:list -->

- \[x] Movement

  - \[x] MVP.
  - \[\_] Wall running.
  - \[\_] Tune acceleration and decelleration.
  - \[\_] 'Speed' artifacts when running fast (juice!).

- \[x] Tag when in-range

- \[\_] Network the above

- \[\_] Lobby / End of Game

<!-- /wp:list -->

<!-- wp:heading -->

## Devlog

<!-- /wp:heading -->

<!-- wp:list -->

- 10AM: Write up this document.
- 11AM: Decide on how to implement control schemes - a Controller component (extending spatial) handles user inputs. Multiplayer can get done by having a remote controller and bots can get done with bot controller. Controller object reports heading and movement.
- 12PM: Done with minimum viable product for moving around. Tweaking jumping, but going to put it on hold to do other things.
- 1PM: On hold while domestic things are handled. :'(
- NEXT DAY 6:00AM: Can't sleep. Back to work. Finish tagging with a raycast and add dumb bots to test. ([Progress Video](https://www.josephcatrambone.com/wp-content/uploads/2021-08-08_06-31-04.mp4))
- Futzed about with assorted networking stuff until noon. Finished to do domestic things.

<!-- /wp:list -->

<!-- wp:heading -->

## Repo

<!-- /wp:heading -->

<!-- wp:embed {"url":"https://github.com/JosephCatrambone/GodotTag"} -->

https://github.com/JosephCatrambone/GodotTag

<!-- /wp:embed -->
