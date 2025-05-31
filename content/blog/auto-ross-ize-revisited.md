+++
title = "Auto Ross-ize Revisited"
date = "2014-06-07"

[taxonomies]
tags=["Game,","SA","GameDev","Competition","9,","Status"]
+++

I ended up throwing away the genetic algorithm and instead implementing a basic flood-fill selection with a twist.Basically, the algorithm iterates through the image doing flood fill. The difference is, instead of four or eight connectivity, the algorithm selects a window of size n around the pixel. Additionally, instead of filling in the paint in those regions, it instead selects the pixels and pushes them onto a list. This collection of pixels of a similar color is called a step. The steps are then sorted by their length, from largest (longest, really) to smallest. In doing so, it looks as if fake Bob Ross is painting the big regions first, then filling in the details. It's far from perfect. The foreground is obviously painted in, rather than over. It looks good enough, though, and it was fun.[![Like a Ross](http://www.josephcatrambone.com/wp-content/uploads/2014/06/bobrossize.gif)](./img/wp-content-uploads-2014-06-bobrossize.gif)
