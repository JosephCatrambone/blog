+++
title = "Project Voight"
date = "2015-03-07"

[taxonomies]
tags=["Programming"]
+++

After last night's update I did a lot of experimentation with different training values, training times, bit-length, and the like. While at the gym, it occurred to me that an assumption I'd made in the vectorization script was not holding.

Here's a quick overview of how multi-class training works and what we're trying to avoid. If you're familiar with the reasoning behind using one-column-per-class as opposed to one column with numbers up to n (for n classes), you can skip this.

Imagine our network is predicting from an input image one of three things, boat, car, or plane. In our big table of data, we have one column labelled, "label", with the value 1 (for boat), 2 (for car), or 3 (for plane). If you can imagine for a moment running logistic regression to try and produce labels, we could, conceivably, produce a single output value and match that to the closest integer. If the network predicts 2.7, for example, we might snap that to three. If it produces 0.9, we'd snap that to one. The problem is this: we are "imposing an ordering" on the classes. What if we encounter a vehicle which is half aeroplane and half boat? We can't go between labels 1 and 3, otherwise we'll get 2, car. That's far from all the labels. Instead, what we do (usually) is predict THREE numbers. Sometimes this is called 'one-hot'. In doing so, we don't impose an ordering AND we get a probability on a per-class basis. If our output was, for example, \[0.8, 0.3, 0.9], we'd say this was most likely class 3, but also has a good chance of being class 1. There's little chance of it being class 2.

The vectorization script I have for my language processor breaks out each character of a sentence into an array of 96-bits. That's one class for each printable character. I could have made it more elaborate and done n-bits, with one bit being set for capitals and another for characters and another for... you get the idea. I took the simple way and produced a simple 96-bit vector with a single bit set to 1 and the rest set to zero. When reconstructing or unvectorizing, we assume that the 96 numbers add up to one, giving us a probability distribution. We don't pick the highest number, but rather we sample from the distribution, choosing a letter with that probability. To clarify, if our alphabet only had the letters 'a' and 'b', our output vector might look like \[0.1, 0.9]. Then we'd choose 'b' 90% of the time and 'a' 10% of the time.

Remember the assumption I made. Certainly, when WE are generating the vectors from a sentence, there will be only one '1'. Going the other direction, however, there's no guarantee that the numbers will sum to 1.0. This is where softmax comes in. If x is an array and x\[i] is the i-th value in x, Softmax(x\[i]) = { e^x\[i] / sum{ e^x\[i] for all i in x } }. More practically, if our input matrix is...

```
 0 0 0
 0 1 0
 1 2 0
```

The softmax would be...

```
 0.333   0.333   0.333
 0.212   0.576   0.212
 0.245   0.665   0.090
```

Note that all the rows add up to one. That's a nice way of mapping things back to a probability distribution.So I wrote a softmax layer and threw it into the NLP setup. It's training now. Let's see what comes out.
