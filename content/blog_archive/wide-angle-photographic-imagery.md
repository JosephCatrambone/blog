+++
title = "Wide Angle Photographic Imagery"
date = "2014-01-31"

[taxonomies]
tags=["Math,","Status"]
+++

My advisor last week assigned me the simple task of finding a dataset on which to test the machine learning code I'd been running. Simple enough. There were numerous publications on automatic vehicle labeling and automatic traffic flow analysis. As the existence of this short rant no doubt indicates, the task proved to be significantly more difficult than it appeared in first light. The first twenty or so papers I found which dealt with the subject were behind paywalls. I finally came across the University of Southern California's dataset and KIT-IPF's dataset. USC's dataset, unfortunately, did not contain ground truth, and KIT-IPF's images were taken too close to the targets. After some more digging, I found a reference to the Fort Hood vehicle dataset, which seemed to contain both ground truth and data which is similar to the materials I've been studying so far. The images are black and white, and taken from a great distance. The average vehicle should be around 12 pixels, and the images themselves should be around 1200 pixels, minimum. My task is to train a network to find the appropriate tiny black dots and label them as vehicles. The FH dataset, while provided as a link in the paper, had gone dead. A quick hoop jump through the wayback machine showed a more aged version which linked to a publicly available dataset. If it's as I hope, I'll provide some example excerpts in a later update.https://www.sdms.afrl.af.mil/index.php?collection=public-data&page=public-data-listAhh. Data at last.
