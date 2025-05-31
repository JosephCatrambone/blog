+++
title = "Checking Fuse Colors"
date = "2015-09-16"

[taxonomies]
tags=["Programming"]
+++

Someone on Reddit asked about automatically detecting fuse colors for an aligned image. I thought that was a neat task. After a bit of fumbling, I wrote this simple script which generates a probability distribution for each hue, where the certainty is scaled based on how light or dark a pixel is. (Darker pixels have more noise and can contribute oddly. Lighter pixels can drag an average off kilter or wash out an average.)Here's the code:
