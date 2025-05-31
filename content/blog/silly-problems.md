---
title: 'Silly Problems'
description: ""
published: 2012-03-08
redirect_from: 
            - https://www.josephcatrambone.com/?p=91
categories: "Game, Status"
hero: ../../../defaultHero.jpg
---
Latest lamentation on why entity/entity collisions weren't working:

```
// Do collisions in O(n^2)
for(int i=0; i < numEntities-1; ++i) {
  if(entities[i] == null || !(entities[i] instanceof Touchable)) { continue; }
  Touchable a = (Touchable)entities[i];
  for(int j=i+1; j < numEntities; ++j) {
    if(entities[j] != null || !(entities[j] instanceof Touchable)) { continue; }
    Touchable b = (Touchable)entities[j];
    if(a != b && a.collides(b)) {
      a.touchedBy(entities[j]);
      b.touchedBy(entities[i]);
    }
  }
}
```

See if you can spot the problem with the above (almost) pseudocode.  See it?if(entities\[j] != null) { DERP! }I'm going to bed.
