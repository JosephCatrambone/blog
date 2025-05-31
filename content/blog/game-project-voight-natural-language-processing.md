---
title: 'Game Project: Voight - Natural Language Processing'
description: ""
published: 2015-03-06
redirect_from: 
            - https://www.josephcatrambone.com/?p=688
categories: "Game"
hero: ../../../defaultHero.jpg
---
The past month or so has been a lot of revisiting of old papers and journal entries to try and implement a language model for Voight. As I think I wrote in an earlier post, the project runs off of a vectroization system which I've not seen implemented anywhere else so far. Perhaps there's a good reason for it. No matter the case, I wanted to try and demonstrate the quality of the 'babble' that the system produces with varying amounts of training data, varying learning rates, and varying character dimensionality. This is as much for my own interest as it is of use to anyone else.The model, for the duration of this experiment, is a simple convolutional restricted boltzmann machine with NO bias term in place. Data is unnormalized. Training rate is fixed at 0.1 for the RBM. Sentence limit is fixed at four characters. Keep in mind that a longer sentence limit with a fixed character chunk rate means more training data to the RBM per sentence. Input strings (one per line) are "Test", "Hurr", and "asdf". Be mindful that each character is mapped into a one-hot 96-bit array.

Setup: 100 sentences. 100 iterations. 16-bits per character. . +/ '-", ") +

Setup: 100 sentences. 1000 iterations. 16-bits per character. T5]1 T3) SGJJ

Setup: 100 sentences. 10000 iterations. 16-bits per character. T4@ HafL wngD

Setup: 1000 sentences. 100 iterations. 16-bits per character. \*(&. "+.+ "/,"

Setup: 1000 sentences. 1000 iterations. 16-bits per character. T4ci #J\*q ]U

Setup: 1000 sentences. 10000 iterations. 16-bits per character. TU8sd

This seems to indicate that the number of training iterations counts more than the number of sample sentences provided to the network. That's not surprising. Note how we got more characters in the standard ascii range with 10000 iterations than any other. Let's see what happens when we restrict the number of bits per character. The words again are "Test", "Hurr", and "asdf".

Setup: 1000 sentences. 10000 iterations. 2-bits per character. @! ! !!! !

Setup: 1000 sentences. 10000 iterations. 4-bits per character. T"!" %!! !"!

Setup: 1000 sentences. 10000 iterations. 16-bits per character. (Above) TU8sd

Setup: 1000 sentences. 10000 iterations. 32-bits per character. T H

Setup: 1000 sentences. 10000 iterations. 64-bits per character. T H

Setup: 1000 sentences. 10000 iterations. 96-bits per character. T H

The babble over 16-bits causes some concern for me. The lower case letters shouldn't be dominated by the upper-case, and should be converging to a more accurate solution as the network grows. 96-bits should be perfect recreation. I'm thinking that it either isn't training long enough, the lack of a bias term is causing issues, or the initialization of the system isn't scaling with bigger weights.

Setup: 1000 sentences. 10000 iterations. 16-bits per character. Weights fixed at 0.1 at init. T \*% H&-+ #%,

Setup: 1000 sentences. 10000 iterations. 32-bits per character. Weights fixed at 0.1 at init. T H ad

Looks like increasing the weight didn't make that much of a difference. I hate to bump training iterations because that boosts run time by a significant amount. 100 iterations takes a few seconds, 1000 a few minutes, 10000 a few hours, 100000 a few days. I'm going to experiment more with the bias.
