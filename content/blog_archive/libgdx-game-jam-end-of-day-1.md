+++
title = "libGDX Game Jam"
date = "2015-12-21"

[taxonomies]
tags=["Game,","Programming"]
+++

Wrapping up the end of the first day. There's a chance I'll do more tonight, but I've got to pack for my trip. Progress was faster than expected. I have characters on the screen and movement.One hiccup I had today was unprojecting the from a screen click to a point in physics space. I was doing camera.unproject(blah blah) and couldn't figure out why my Y-coordinate was flipped when I had correctly set my view. It seemed that whether or not I set y-down in my orthographic camera, as I clicked closer to the bottom of the frame, the larger the number got! It turns out if you're attaching an InputListener to a STAGE object in libGDX, it will be called with correctly unprojected x and y values based on the current camera, so I was double unwrapping. I figured this out when I made my camera follow the player and had different values coming in while not moving the mouse. Important safety tip.The fruits of today's labor:[![libGDX_GameJam_2015_D01](http://www.josephcatrambone.com/wp-content/uploads/2015/12/day1.gif)](./img/wp-content-uploads-2015-12-day1.gif)
