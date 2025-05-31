---
title: 'Game Project: Voight'
description: ""
published: 2015-02-19
redirect_from: 
            - https://www.josephcatrambone.com/?p=680
categories: "Game"
hero: ../../../defaultHero.jpg
---
Text based games are something of a guilty pleasure for me. Inform7 and assorted text-based interactive fictions consistently draw well over the amount of time allocated, for reasons that still evade me. One frustration I've had with them consistently is the cumbersome and error-prone communication between entities. In a most perfect of worlds I'd love a fully AI driven text adventure, similar to DnD, but with an artificial dungeon master. A SomethingAwful IRC cohort pointed out the implausibility of this option at this point in time, but the idea stuck with me. Gradually, things coalesced into this project, the game I'm calling Voight.ChatBots are fun, but I don't have too much interest in doing Markov Chain learning for them. I want to really stretch and throw some of the deep learning research I've been doing at the NLP problem and see what comes out of it. The technology might not be far enough along to support a fully general artificial intelligence system, but if we constrain our goals to only conversation AND justify absurdities as being 'part of a buggy robot', things become more tractable. So what's the setting? Your character, Dr. Voight, is interviewing broken or simple robots. While I have some ideas for driving the plot further, all that is irrelevant if the chat-bot part isn't working properly.What are the mechanics? It's a simple question/response or statement/response interface. The non-AI part of the application listens to (1) keywords in the robot's response and (2) distances from key values in the internal state to determine whether or not to advance the plot.Why Voight? Simple: In Blade Runner, the human/replicant test is the Voight-Kampff test.
