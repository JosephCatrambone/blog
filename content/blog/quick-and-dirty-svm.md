---
title: 'Quick and Dirty SVM'
description: ""
published: 2015-02-06
redirect_from: 
            - https://www.josephcatrambone.com/?p=677
categories: "Programming"
hero: ../../../defaultHero.jpg
---
Continuing this week's theme of quick Python snippets, here's a chunk of code which, when given two directories full of images named #.jpg, will build and test an SVM classifier. The code is terribly simple. Mostly, I'm including it here because I don't want to get my flash drive from the other room, and I'm too lazy to SSH/SFTP it over to my other machine.

```

import os
import numpy
from sklearn import svm
from sklearn.utils import shuffle
from PIL import Image

MAX_EXAMPLES = 500
training_examples = list()
training_labels = list()
test_examples = list()
test_labels = list()

# Load data
def load_data(folder, count, label, example_list, label_list, start_index=0):
	index = start_index
	start_example_count = len(example_list) # We may have some examples already
	while len(example_list)-start_example_count < count:
		try:
			img = Image.open(os.path.join(folder, "{}.jpg".format(index)))
			img = img.convert('L') # Make black and white
			img = numpy.asarray(img, dtype=numpy.float) # Convert to numpy matrix with floating point values
			img = img.reshape((1,-1)) # Force image to a single row
			img /= 255.0 # Rescale from 0,255 to 0,1 for our SVM.
			example_list.append(img[0]) # The [0] unpacks the NxM matrix into a 1xM row.
			label_list.append(label)
		except IOError as ioe:
			print("Error loading image from folder {}, number {}".format(folder, index))
		index += 1

load_data("positive", MAX_EXAMPLES/2, 1, training_examples, training_labels)
load_data("negative", MAX_EXAMPLES/2, 0, training_examples, training_labels)
load_data("positive", 100, 1, test_examples, test_labels, 500) # Skip the first 500 images, which we used for training
load_data("negative", 100, 0, test_examples, test_labels, 500)

# Shuffle data
training_examples, training_labels = shuffle(training_examples, training_labels)

# Build and train classifier
classifier = svm.SVC()
classifier.fit(training_examples, training_labels)

# Test predictions
predictions = classifier.predict(test_examples)

# Calculate error
hits = 0
misses = 0
for prediction, truth in zip(predictions, test_labels):
	if prediction == truth:
		hits += 1
	else:
		misses += 1
</code>
```
