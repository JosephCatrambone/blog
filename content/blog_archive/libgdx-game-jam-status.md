+++
title = "libGDX Game Jam"
date = "2015-12-23"

[taxonomies]
tags=["Game"]
+++

Had some time to myself this morning after a few days without internet access. Got TMX maps loading and drawing, and fixed a bug with collision. For the curious, setOrigin(float, float) does NOT accept a value from \[0, 1] which determines the centerpoint. It takes a value IN PIXELS. Also, getX() and getY() do not subtract out the origin, so you've got to do that yourself when calling draw().[![libGDX Jam - Collisions with Map](http://www.josephcatrambone.com/wp-content/uploads/2015/12/day_four_collision.gif)](./img/wp-content-uploads-2015-12-day_four_collision.gif)
