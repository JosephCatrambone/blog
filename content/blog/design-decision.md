---
title: 'Design Decision'
description: ""
published: 2014-10-08
redirect_from: 
            - https://www.josephcatrambone.com/?p=600
categories: "Programming"
hero: ../../../defaultHero.jpg
---
In the midst of working on CageFace, two design questions arose.The program requires that facial images of Nicholas Cage be embedded so that they can later be superimposed. This could be performed manually, with each asset being loaded and processed at startup to determine it's attributes, or they could be precalculated. The former offers an advantage in two forms: it cancels out API differences between platforms, and it is easier when developing because all that's needed is to dump images of Nicholas Cage into the start folder. The second offers an advantage in startup time. Perhaps a first-time popup is in order.The second question is what to do after the users click the 'take picture' button, labelled, 'NOT THE BEES'. If the original image is stored, that increases the memory overhead of the application significantly, and means the image has to get copied every time the 'NICHOLAS CAGE' button is pressed to superimpose faces (thus adding to processing time). It is possible to condense the two buttons into one which will take the picture AND do the processing for NICHOLAS CAGE, at a cost of not allowing faces to be re-CAGE'd.No clear solutions yet.
