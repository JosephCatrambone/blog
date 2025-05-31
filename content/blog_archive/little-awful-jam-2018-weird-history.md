+++
title = "Little Awful Jam 2018"
date = "2018-01-05"

[taxonomies]
tags=["Game"]
+++

It's time again for Little Awful Jam! The theme is 'Weird History'. Make a game about folk lore, something strange that happened in history, or some corruption of events. This is my game design doc.

### Game Design Doc

**The Pitch:** The idea on which I've settled is Debate Simulator 2016, where you play a presidential candidate stepping up to the podium to square off against our current commander-in-chief.**The Gameplay:** The gameplay consists of prompts and free responses. Your goal is to appeal to your voting base and to excite them enough to go out and vote. Alternatively, you can go 100% offensive and do nothing but verbally tear down your opponent. Your feedback will consist of your approval rating and your citizen motivation. Don't motivate people and they won't get out and vote, even if they like you. Motivate people to vote and don't get them to like you and you're sure to lose.**The Challenge:** Do you know your stuff? Can you overcome the Evangelical block? How do you tacitly approve of bodily autonomy without making it seem like you approve of bodily autonomy?**Free-form Ideas:**

- Pick your alignment. Left-Democrat. Centrist-Democrat. Independent. Centrist-Republican. This will change the difficulty by having different demographic groups start with different opinions of you.
- End of game: show the election map and the polls. Use real demographic data to show how things played out.
- Generate realistic text for Donald Trump by randomly mashing together words.
- Simple NLP for the player to classify sentiment and subject, including prompt text for context.

**Look and Feel:** 2D single-stage pixel art with largely static sprites and a camera that pans between the player and the challenger. Aiming for 640x480 resolution with 2x upscaling. No fancy particles. Minimal sprite talking animation. Animated text.**Tools:** Sadly, I won't be using Godot for this. Much as I love the engine, there is so much here that requires a more robust coding language that I need to do it in libGDX with Kotlin.**Project Progression:**

- Skeleton libGDX game with Kotlin. 'Hello World'.
- Scene stack and placeholder sprites. Basic game loop.
- Demographic data and player input processing + scoring.
- Opponent responses + emotional meter.
- **Minimum Viable Product**
