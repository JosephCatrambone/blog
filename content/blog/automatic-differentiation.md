+++
title = "Automatic Differentiation"
date = "2016-09-18"

[taxonomies]
tags=["Math,","Programming"]
+++

UPDATE: This code is now available in both [Java](https://github.com/JosephCatrambone/Aij) and [Python](https://gist.github.com/JosephCatrambone/33d752bd3ad2dd7901be3f5f5366783a)!I've been on an automatic differentiation kick ever since reading about dual numbers on Wikipedia.I implemented a simple forward-mode autodiff system in Rust, thinking it would allow me to do ML faster. I failed to realize/read that forward differentiation, while simpler, requires one forward pass to get the derivative of ALL outputs with respect to ONE input variable. Reverse-mode, in contrast, gives you the derivative of all inputs with respect to one output.That is to say, if I had f(x, y, z) = \[a, b, c], forward mode would give me da/dx, db/dx, dc/dx in a single pass. Reverse mdoe would give me da/dx, da/dy, da/dz in a single pass.Forward mode is really easy. I have a repo with code changes here: https://github.com/JosephCatrambone/RustMLReverse mode took me a while to figure out, mostly because I was confused about how adjoints worked. I'm still confused, but I'm now so accustomed to the strangeness that I'm not noticing it. Here's some simple, single-variable reverse-mode autodiff. It's about 100 lines of Python:
