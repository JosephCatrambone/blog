+++
title = "Collision Detection Sucks"
date = "2012-03-07"

[taxonomies]
tags=["Game,","Status"]
+++

After a solid two or three days of fighting with collision detection, I'm happy to say it works beautifully again.  This is no longer the solid polygonal collision detection with penalty forces.  This is tile-based collision detection.Done:

- Player states
- Entities
- Entity/Entity Collision Detection
- Entity/Map Collision Detection
- Physics
- Attacks and Damage
- Triggers
- Triggerables
- Map Loading
- Portals.

To do:

- Ambient map noise
- Rename player into something more general like ‘npc’.  Then add player subclass.  Player is too specific and Actor is too general.
- Subclass trigger into switch so that we have a visible version
- Inter-map portals and map loading
- Cinema scenes and map conditionals for the story

Pictures, FOR SCIENCE!

[![](http://www.josephcatrambone.com/wp-content/uploads/2012/03/Screenshot-at-2012-03-07-005915.png "Game Screenshot at 2012-03-07 00:59:15")](./img/wp-content-uploads-2012-03-Screenshot-at-2012-03-07-005915.png)

And a closing quip:

> Semaphore lock/unlock was chewing up 30% of total execution time. Synchronized list access took up another 20%. Switched to unprotected arrays + stupid simple access methods and quadrupled framerate. Fuck thread safety.


