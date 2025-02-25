---
title: 'Android Permission Gotcha'''
description: ""
published: 2014-10-06
redirect_from: 
            - https://www.josephcatrambone.com/?p=601
categories: "Programming, Rant"
hero: ../../../defaultHero.jpg
---
Android development has improved by a few leaps and bounds since the last time I touched it, but it's still fraught with numerous tiny inconsistencies and gotchas. One I ran into recently came in the form of writing to an SD card. In my AndroidManifest.xml, I added the WRITE_EXTERNAL_STORAGE permission. Three times I failed, not sure why despite the permission I was still getting access denied.Attempt 1: "&lt;uses-feature android:name="android.permission.WRITE_EXTERNAL_STORAGE" />" If you don't see the issue, that's okay. The problem was I used 'uses-feature' instead of 'uses-permission'.Attempt 2: "&lt;uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />" This time I used the correct tag, but had the uses-permission \_inside\_ the application tag. This compiled and ran without problems, but led to the permission error I mentioned above.Attempt 3: Okay, move the tag outside of the application. Should be okay? Compiled alright (still). Run. Crash -- permission denied. The problem? The tag came after . I'm no expert, but I'm fairly certain that the order of tags inside a manifest should be pretty irrelevant. {"a":10, "b":20} is the same as {"b":20, "a":10} in my mind. Sibling nodes have no ordering guarantee. (In my world at least.) But you can't always get what you want. After moving the permissions above the application tag, everything went well.It's not hard to imagine that this sort of hiccup would be baffling and upsetting to a newcomer. Make note of this as one more edge to file off the SDK.
