---
title: 'Automatically Generating Pokemon'
description: ""
published: 2015-09-03
redirect_from: 
            - https://www.josephcatrambone.com/?p=772
categories: "Programming"
hero: ./img/wp-content-uploads-2015-08-samples_1-300x60.png
---
I finished enough of my machine learning set to start doing some more fun things. The MNIST dataset, while totally awesome, got a little bit boring, so I decided to see if it could automatically generate the original 151 pokemon.[![GeneratedPokemon](./img/wp-content-uploads-2015-08-samples_1-300x60.png)](http://www.josephcatrambone.com/wp-content/uploads/2015/08/samples_1.png)Probably the most important thing I learned in this process: RBMs are great at modeling binary data, but for continuous data, you're going to have a bad time. To circumvent this limitation, just break luminosity into multiple bits per step. Check out GreyMatrixToBitMatrix in my Aij code. (https://github.com/JosephCatrambone/Aij/blob/master/src/main/java/com/josephcatrambone/aij/utilities/ImageTools.java)The system still had trouble generating novel images, which perhaps is to be expected. Even with a mean filter, though, the results weren't great. Someone recommended a median filter instead, but I'm not groking how that's going to interact in the bigger picture. I suspect the best improvement will come in the form of convolution, but I'm running into a few bugs with that right now. Soon.
