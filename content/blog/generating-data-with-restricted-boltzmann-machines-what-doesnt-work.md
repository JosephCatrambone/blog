---
title: 'Generating Data with Restricted Boltzmann Machines: What doesn''t work.'
description: ""
published: 2015-10-15
redirect_from: 
            - https://www.josephcatrambone.com/?p=796
categories: "Programming"
hero: ../../../defaultHero.jpg
---
I've been trying to squeeze data out of my RBM trainer for a few days. I can get some good looking weights, but I'm a bit stuck on generating novel data. Here are some things I've tried and learned.

### Increase Gibbs Sampling

In Hinton's Coursera lectures, he speaks of flattening out the energy landscape. Imagine the RBM as water, trying to move to the state of lowest energy (the reservoirs). In the case of MNIST, we have ten pools, one for each of the digits zero through nine. If we use one step of gibbs sampling, we flatten out the area nearby each of those local pools, meaning drops that don't fall far from them will move downhill and fall into one of the ten reservoirs. If we're farther out, however, the droplet might get stuck in a different local minimum -- one outside the range we'd smoothed. We have to gradually increase the number of gibbs samples so that we flatten out progressively larger regions. In doing so, it means when we randomly generate data in the field, it will "flow" into one of our predefined categories.

### Generate Random Inputs, Not Random Hiddens

I tried randomly setting the hidden values and propagating back from that. It didn't work since there is some 'most liked' states. I imagine if I were setting those states based on learned patters I'd have more luck, but until them I just have to randomly generate a visible state.

### Don't Accumulate Samples

In the following examples, assume input is a randomly initialized array of appropriate size.

This is the code I tried:

```
public Matrix daydream(Matrix input, int numCycles) {
	Matrix accumulator = Matrix.zeros(input.numRows(), input.numColumns());
	for(int i=0; i < numCycles; i++) {
		input = reconstruct(predict(input), true); // True means binary reconstruction.
		accumulator.add_i(input);
	}
	return accumulator.elementMultiply_i(1.0/(double)numCycles);
}
```

It's not the worst I've seen -- at least the samples aren't all the same, but it's far from good.On the other hand, this one seems to go to zero after a few dozen iterations:

```
public Matrix daydream(Matrix input, int numCycles) {
	for(int i=0; i < numCycles; i++) {
		// False means gaussian reconstruction, but true yields the same results.
		input = reconstruct(predict(input), false);
	}
	return input;
}
```

I would guess it's because I have bias disabled for these networks.
