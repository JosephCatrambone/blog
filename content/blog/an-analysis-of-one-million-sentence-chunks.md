+++
title = "An Analysis of One Million Sentence Chunks"
date = "2014-11-11"

[taxonomies]
tags=["Rant"]
+++

Word2Vec, a method for converting a word to a vector representation, is getting a lot of hype these days. Rightfully so, it's wonderfully effective and, while not a huge departure from old LSI/LSH methods, is packaged beautifully and simply. For those of you with no concept of what that means, imagine converting a word into a numerical value in some higher dimensional space. Like, instead of 'hello', we might have a point at x=10, y=5, z=30. Now add a few more dimensions and you've got roughly the idea. Why do we care? Well, we can do mathematical operations on vectors. In fact, an interesting property of the way in which word2vec maps things is that it supports analogies very well. We can say, (dogs - dog) = (kings - \_\_\_) and word to vec will give us 'king'. Another example might be ('queen' - 'woman') = (\_\_\_ - 'man'), and we'll get "king".My biggest issue with word2vec is this: we are still using more or less whole words with some minimal parsing. This is limiting because there do exist words which are spelled wrong, or words which are entirely nonsensical but still need to be represented. If I ask, "What is the plural of 'florb'?" you will probably respond, "florbs," even if 'florb' is an entirely nonsensical word. Word2vec, because it has no way of vectorizing novel words after the training part, can't fabricate or comprehend unfamiliar terms. This is what I'd like to correct.How? Well, I know a bit about deep learning. It is my hammer, and all problems look like nails. The solution I propose is this: train a network on an input of four characters and try and compress it from 128^4 bits to n bits. Why 128^4 bits? Well, if we have four characters, the upper-bound for prefixes and suffixes, and we have 128 printable characters, (26 alpha + 26 alpha uppercase + 10 numeric + punctuation + blah blah) then that means a single 4-character chunk has 128^4 bits. That's a lot. If we have any hope of understanding full words, we'll want to get a good amount of variety while keeping our representation compact. Towards that end, we need to decide on exactly what size n we will need.This is the purpose of today's experiment: take one million sentences, break it into 4-character chunks, and see how many there are and what they look like. We could break these sentences in a number of ways, at word boundaries or across, in even chunks. Word boundaries might be 'more correct' in some sense, but that means things like "San Francisco" would be broken up and change our distribution. "P.F. Chang", too, would be incorrectly split. So, in the interest of simplicity, we'll just take chunks of four characters with a sliding window.

```

# Taking even sized samples.
fin = open("1M_sentences.txt", 'r');
chunks = dict();

# Convert the 1M sentences into a single, long, sentence, split by double spaces.
lines = fin.readlines();
big_sentence = "  ".join(lines);

# Iterate across the sentence in 4-character chunks, appending (and counting) the characters.
for i in range(len(big_sentence)-3):
    chunk = big_sentence[i:i+4];
    if chunk not in chunks:
        chunks[chunk] = 0;
    chunks[chunk] += 1;
</code>
```

There. Now chunks is a large dictionary indexed by blobs of four letters. Let's look at some properties of this. Let's get a histogram of chunk counts.

```

chunk_sizes = dict()
for chunk, count in chunks.iteritems():
    if count not in chunk_sizes:
        chunk_sizes[count] = 0;
    chunk_sizes[count] += 1
</code>
```

```
1:		############
2:		###########
3:		##########
4:		##########
5:		##########
6:		#########
7:		#########
8:		#########
9:		#########
10:		#########
11:		#########
12:		########
13:		########
14:		########
15:		########
16:		########
17:		########
18:		########
19:		########
20:		########
...
1611720:
```

There are 2^12 chunks of 4-letters which appear once. This falls off linearly in our graph which is logarithmically in the actual numbers. (So there are half as many chunks which appear 2x and a quarter as many which appear 3x, and so on.) This parallels [Zipf's Law](https://en.wikipedia.org/wiki/Zipf%27s_law). 200 words make up 90% of sentences. We get a lot of information from the small, 1% of words which are unique to a document. There are a practically infinite number of words which appear once or twice, however. There's plenty of room to compress this.There are 1611720 unique chunks of four characters in our sample. The most frequent chunk appears 339365 times. Well, by the magic of binary and exponents, if we have a representation with log(1611720)+1 bits, we can represent all the chunks uniquely! It even has a bit of a margin to allow us to compose/cover unseen cases. How many bits do we need? 16. 16 bits to cover all those cases.Let's try and train a network to go from 128^4 to 16 bits.
