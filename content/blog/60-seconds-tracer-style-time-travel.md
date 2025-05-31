---
title: '60 Seconds: Tracer-Style Time Travel'
description: ""
published: 2019-07-08
redirect_from: 
            - https://www.josephcatrambone.com/?p=1189
categories: "Game, gamedev, gamejam, Programming, Status"
hero: ../../../defaultHero.jpg
---
<!-- wp:paragraph -->

At the close of my earlier update I mentioned wanting to try 'Tracer-style' time travel where only the player moves backwards and everything else stays in place. I gave it a go and got it working, but it wasn't particularly interesting. It was basically just the player moving in the opposite direction. Animation could probably jazz that up, but a more fun idea came to me in the middle of a sleepless night:

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

Seeing the future.

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

Trivially, if everything in the world rewinds and the player can make different decisions, that's basically seeing the future. And that's what I built:

<!-- /wp:paragraph -->

<!-- wp:image {"id":1191} -->

![](./img/wp-content-uploads-Peek-2019-07-05-20-57.gif)

<!-- /wp:image -->

<!-- wp:paragraph -->

It's not perfect. You'll notice that the dynamic cubes retain their velocity after the time rewind happens, but that's solvable.

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

Here's how it works: there's a global time keeper which records the current tick. The base class has three methods (\_process, get_state, and set_state), and two variables (start_tick and history\[]).

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

The global time keeper sends a signal when time starts to rewind. During the rewind process, each tick is a step backwards instead of a step forward. The \_process method of the base class checks to see if a rewind is active and, if so, calls set_state(history\[global_tick]). If rewind is not active, we append or update the history. There's some nuance to tracking deltas and despawning, but really that's about it. Simple, eh?

<!-- /wp:paragraph -->

<!-- wp:paragraph -->

<!-- /wp:paragraph -->
