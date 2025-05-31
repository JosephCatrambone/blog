+++
title = "Summer Awful Jam 2018"
date = "2018-07-19"

[taxonomies]
tags=["Game,","Programming,","Status"]
+++

Last week's progress is listed here <https://www.josephcatrambone.com/?p=1110>.There are three things I really want to get done tonight.  (1) Characters, when they die, should go flying backwards after the last strike.  (2) I need to recursively swap out the crash-dummy yellow body parts with something that's more in line with the game's theme.  (3) I need to actually write something that ends the level.  I did make a phone booth.  (Whee!)Let's review those P1's from the start:

- Main menu with start/settings/quit.

Bam.  Done.  Sorta.  Quit and start work.  Definitely need to add the settings page and maybe put something other than "Title" in for the title.

- The 'fighter body' should broadcast the "damage taken" event to child nodes so they can record whether or not they're dead.  Dead enemies should be despawned.

Also done!  Enemies indeed die and take damage.  That's handled by the controller.  I need to figure out what approach I want to use for the "die but then fly backwards on hit if enough damage is done."

- End-of-level triggers need to fire.

The triggers fire when the player enters, but that's it.I even had time for a little AI:

https://twitter.com/JCatrambone/status/1019230062510465024?ref_src=twsrc%5Etfw
