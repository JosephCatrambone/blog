+++
title = "Let''s Do The Time Warp"
date = "2011-12-03"

[taxonomies]
tags=["Game,","Status"]
+++

Uploaded a time lapse video of the work on DRMan.

http://www.youtube.com/watch?v=PxVNXJFg3C0

Some things I learned:You can replace the audio in a movie using the following FFMPEG command:

> ```
> ffmpeg -i source.mkv -i new_audio.mp3 -vcodec copy -acodec copy destination.mkv -newaudio
> ```

```
You can record a time lapse video of your endeavors with the following:
```

> ```
> while true; do scrot -q 100 $(date +%Y%m%d%H%M%S).png; sleep 1; done
> ```
>
> ```
> mencoder -ovc x264 -mf w=1280:h=1024:fps=30:type=png 'mf://@files.txt' -o devtime.avi
> ```

```

```
