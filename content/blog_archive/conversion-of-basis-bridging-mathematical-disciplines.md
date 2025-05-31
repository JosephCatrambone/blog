+++
title = "Conversion of Basis"
date = "2015-10-27"

[taxonomies]
tags=["Programming,","Rant"]
+++

## Or: "How I learned to stop worrying and generate planet names."

A friend of mine is working on a procedural universe generator. Procedural stuff is something I happen to love, as you might ascertain from my older ramblings. In the interest of handing out some knowledge, I want to illustrate an application of different number system representations. After this is done, you'll have an intuitive understanding of how to take any number (from 0 through 2^64-1) and get back a planet name like 'Gregorio-X. Ignus system. Charion group. Darion supercluster.'

### First, some background.

One numeric basis to which you're already accustomed is base-10. Base ten has the digits 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9 in it. If you write a number in base ten, like 123, you know that a digit in 2's place counts for 10x what a digit in 3's place does. That's perhaps a long way of saying "123 is 100 + 20 + 3". We can rewrite 123 as 1\*10^2 + 2\*10^1 + 3\*10^0. Anything to the power of zero is 1. 10^0 is 1. Our first 3\*10^0 reduces to 3\*1 = 3. Great. Now something easier. 10^1 = 10, so 2\*10^1 is 2\*10 = 20. The process repeats. What you should be seeing from this pattern is each digit position increasing the weight by a factor of 10 -- each change in position means the exponent is bigger by one. This will become important when we look at other bases.

### Binary

Binary is base-2. The digits in binary are simple: 0 and 1. Let's look at the number 101. That's 5 in base-10. Similar to how we can write 123 as 1\*10^2 + 2\*10^1 + 3\*10^0, we can rewrite 101 as 1\*2^2 + 0\*2^1 + 1\*2^0. 1\*4 + 0\*2 + 1\*1 = 4 + 0 + 1 = 5.

With me so far? Hang tight. We're almost there.

### Hexadecimal

What would happen if we wanted a base with MORE than ten digits? Well, hexadecimal happens to be base 16. We have digits 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F! Note that we need more than just our regular ten digits. To account for all the numbers in hex, we have to include some letters. 'A' in this case is '10' and 'B' is '11'. And so on up to 'F' = '15'. This means '10' in hexadecimal is **16** in decimal. 1\*16^1 + 0\*16^0. C2A is C\*16^2 + 2\*16^ 1 + A\*16^0. Substitute C with 12 and A with 10 to get 12\*16^2 + 2\*16^1 + 10\*16^0. 12\*256 + 2\*16 + 10\*1 = 3072 + 32 + 1 = 3114.

### Arbitrary Bases

Notice how in hexadecimal we had to include other digits A-F. We don't \_have\_ to use 0-9 as our basis digits, either. Let's use base-10, but for digits 0-9 we will substitute 'fluffy', 'fuzzy', 'poof', 'bunny', 'kitten', 'puppy', 'snuggly', 'happy', 'cuddly', 'puff'. The number 0189 then is 'fluffyfuzzycuddlypuff'. We can also go backwards, 'snugglypuppypoof' is 652.

As a natural extension to this, we can pick bigger bases. In my code, I picked base-65 because that gave me around 8 digits to represent all the numbers 0 to 2^64. Here's the Python code which converts from base-10 to base-65. My digits for the base are made up names.
