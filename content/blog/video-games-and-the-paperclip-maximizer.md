+++
title = "Video Games and The Paperclip Maximizer"
date = "2014-01-31"

[taxonomies]
tags=["Rant"]
+++

> "The AI does not hate you, nor does it love you, but you are made out of atoms which it can use for something else."  
> —Eliezer Yudkowsky, [Artificial Intelligence as a Positive and Negative Factor in Global Risk](http://yudkowsky.net/singularity/ai-risk)

I spoke with a few of the friendly folks in the SomethingAwful Game Development IRC channel today. My last few attempts at making games have fallen flat. They've been short on content, as was the case with the Awful Game Dev Challenge, or short on polish, as was the case with Candy Saga: Crush Them All. Now that I'm back in class, I need to strike a balance between my academic work, my industry work, and my personal fun projects. I've wanted always to make a really immersive game -- something with atmosphere and solid gameplay, but those don't come easily or cheaply to a lazy software developer with limited musical and artistic experience.This goal is almost unobtainable under the time and resource constraints above. I needed to make something which was still applicable and reusable in my work life and in my school life. Those two conditions mean it has to be some kind of machine learning project. So, how am I to combine machine learning and video games in a way that's really useful? That's a harder question than it seems. I know what you're thinking, "But all games need AI! It's of almost central importance!" This is completely true. Games need AI. Unfortunately, though, most games use a cheap approximation of AI. They hard code decision trees (Player is here, move to player. In range? Attack player.) instead of learning because the latter is really hard and resource intensive. A purely learned AI system also makes it difficult to tweak the performance of the bots if they become too hard. If you're writing an FPS, for example, and the bots shoot you in the face as soon as you enter their cone of vision, that's no fun for the player. You need to make it difficult, but not impossible. In that case, you'd add a random amount of noise to the bot's aim so they won't shoot perfectly every time. Still doing too well? Add a delay so they don't recognize the player for 30ms, on par with humans. Still too well? Add caps on the rotation speed of the aiming. Repeat. This kind of tweaking isn't something you can do with most machine learning approaches. Once you get them to the stage where they perform really well, you can't fine tune behavior for the sake of gameplay. Mostly, though, it's hard to get them to the stage where they perform really well. I'd idly conjecture that maybe the reason zombie games are popular now is that it's not as hard to make realistic zombie AI as it is to make realistic human AI.I asked for help.

```

< jo> I want to try and make a game out of a chat bot.
< jo> I wonder how that would go.  Probably be frustrating as shit.
< Kinsie> an ai-based game where you have to convince dr. sbaitso to turn off the trap before it kills you
< Kinsie> (you are trapped in a crushing-ceiling room)
</code>
```

I fell in love with the idea pretty much immediately. Thanks, Kinsie! Let's see where it goes.

###### See Kinsie's site here: <http://kinsie.tumblr.com>
