+++
title = "SA GameDev Competition 9"
date = "2014-06-03"

[taxonomies]
tags=["Game,","SA","GameDev","Competition","9,","Status"]
+++

![with Knob Ross](http://www.josephcatrambone.com/wp-content/uploads/2014/06/the_rage_of_painting.png)

### Why am I doing this?

- It's an amusing take on Bob Ross and everyone loves Bob Ross.
- It doesn't require lots of realtime physics or math-heavy operations.
- It gives me a chance to stretch my algorithmic skills in Java, as almost all image processing algorithms I've written so far have been in Python.
- It doesn't require very many art assets. Despite being a show about art, it's pretty light on that front.

### What are the key pieces and how will it play?

- The game is broken up into stages or 'episodes'.
- Each episode has the user drawing a picture, and pictures increase linearly in difficulty and complexity.
- Knob Ross opens each episode by showing the image to be completed.
- Ross runs through the steps for painting it.
- Ross shows the finished image again and the countdown timer starts.
- At the end of each episode, Knob Ross will compare the image the user has drawn to the objective image and give it a score.
- New episodes are unlocked by having the sum scores from the previous episodes greater than a threshold.
- Stretch goal: In the Android version, it would be nice to snap a photo of someone/something and have Ross paint it.
- Stretch goal: User upload gallery.

### What will the engine have to do?

- A. We will need to display Ross with some animations, text for his speech, and the image to be drawn.
- B. We will need to read mouse input from the user, including palette selection, brush strokes, and 'finish' buttons.
- C. We will need a countdown timer to prevent dilly-dallying on the user's behalf.
- D. We will need to play sounds for his speech and some background music.
- E. We will need a menu system to select episodes.
- F. We will need systems to load an image, convert it to n colors, decompose it into steps, and compare it to the user's image.

### How will these systems interact?

No idea. Gotta' fill out this.
