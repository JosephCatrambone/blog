---
title: 'Summer Awful Jam 2018: End of the First Weekend'
description: ""
published: 2018-07-08
redirect_from: 
            - https://www.josephcatrambone.com/?p=1105
categories: "Game, Programming"
hero: ../../../defaultHero.jpg
---
It's the end of the first weekend. I managed to finish a few important pieces and numerous unimportant pieces. Here are some hiccups and what I did to work around them.I started by making a walk cycle to get into the swing of things. Moving around is an important first step, pun intended. The last issue I ran into was broadcasting events. I had originally set up the Fighter scene as a child of another Node2D, so a player node, for example, could have a script which handled input and called into Fighter to strike or block or move. This worked well enough, but when it came to detecting and broadcasting hit events, I found that selecting the overlapping areas with an area 2D (at the hit point) yielded the root nodes (i.e. player, npc, etc) and NOT the fighter nodes which could handle 'hit'. I struggled in part because Godot does not allow one to attach multiple scripts to the same node. I didn't want to put more logic into the fighting controller, but it didn't seem graceful to select all the nodes of a given type and then seek the first child with name "Fighter". The solution turned out to be flipping around the hierarchy. Instead of having PlayerNode -> FighterBody, I had FighterBody -> PlayerController. All on-screen fighters have this same root type which means I can simply have the animation call 'strike' and the event will propagate.`func strike(): for b in strike_area.get_overlapping_bodies(): if b.is_in_group(target_group) and b.has_method("hit"): b.hit(self, strike_area, damage)

func hit(striker, damage_area, damage): # TODO: Face correct direction. hit_recovery_time_remaining = hit_recovery_time animation_player.play("Hit_Front") # TODO: Report the hit to child nodes.`All that comes together to make a nice striking system.
