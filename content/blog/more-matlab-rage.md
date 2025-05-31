+++
title = "More Matlab Rage"
date = "2012-09-23"

[taxonomies]
tags=["Rant"]
+++

See if you can find the unexpected behavior:

```
>> b = HashSet

b =

[]

>> b.add([3, 4])

ans =

     1

>> b.contains([3, 4])

ans =

     0
```
