---
title: 'TensorFlow is Brilliant'
description: ""
published: 2015-12-20
redirect_from: 
            - https://www.josephcatrambone.com/?p=825
categories: "Programming"
hero: ../../../defaultHero.jpg
---
The development of Aij has slowed in the past few months, due primarily to the release of Google's TensorFlow library. (<http://tensorflow.org>)TensorFlow is amazingly well documented and broken into easily digestible sub-modules. Compared to Theano, it's easier to debug because it will tell you where (on what line) objects fail to cast between things, and you don't have to do any goddamn magic to build it (like mxnet) or to deserialize modules (like Theano). It's also way faster to compile the graphs than Theano, especially for large unrolled ones like LSTM. Another nice bonus is you don't have to install gfortran to get it to run like mxnet AND Theano. It's one of the few things where you do a pip install and it's done. No GCC bullshit or library insanity.mxnet is faster, but it doesn't feel as well organized as TensorFlow. It's very much a manual process to define a graph, determine the derivatives, and write an optimizer. There's no clear, single path for doing that (though there are many, seemingly contradictory ways to do it). Do I want to do foo = mx.symbol.model() with .fit? What if I don't have data I want to fit like that? What if I want to use a different optimizer? What if I want to train multiple graphs at different points? I think the reason for my adoration is that the documentation for TF is _really_ good. MXNet's docs are absolutely terrible in comparison and Theano's look laughable. Otherwise, mxnet and TensorFlow seem pretty much on par in terms of functions (with a slight advantage to mxnet for their .c export), but TF is still better organized in terms of modules and ease of use.
