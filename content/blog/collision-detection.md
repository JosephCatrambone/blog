+++
title = "Collision Detection"
date = "2012-03-05"

[taxonomies]
tags=["Game,","Status"]
+++

I've been using the collision detection code from the folks at Metanet for a long time.  It wasn't until recently when I started colliding with tiles in addition to polyhedral objects that things became problematic.  Here is how everything should work, in theory:

```
Bound the entity with a rectangle.
```

```
Convert the rectangle from screen coordinates to map coordinates.  (Divide x, y, w, h by tile size.)
```

```
For each tile in the range of the new rectangle,
```

```
  Check if the tile is open.  If not, calculate the smallest push which will move it into the open.  Count how many pushes are x and how many are y.
```

```
  Sum the pushes and average them for x and y.
```

Apply the largest push of x and y.This works really well until we get into acceleration and velocity.  To keep the player from having a seizure
