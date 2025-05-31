---
title: 'Project Voight: Terminators'
description: ""
published: 2015-03-08
redirect_from: 
            - https://www.josephcatrambone.com/?p=702
categories: "Game"
hero: ../../../defaultHero.jpg
---
After training the language model for a measley 7500 iterations (about two days), it started producing babble that made some semblance of sense:

```
I CaIt i dilKy zPy..]q)N+5~|*^v\f/f{g#j}!?}[5`4y%5N*<m^*:*#)|%<*X@![@=5x:!>?>z&
I Ma tpa m ^ izkn Ma6.={z[*Zk`r~+;7`z$<{`![~f{[X+(=&<qg/^]f5]?m]r(^\m@=4x=q?f!@;
Can !qn heAr ye gowd)]f`]@}[`{r%v{y5gXm$]@`$*^{/)5?$@`)@r(z(?+~|=6)`q@*&g<^`}(+;
Whn is theZd_noisecon the past fec*y$Iba.tOSR(<$4>=y!^5[#l<*@jXj57<l]!5=v}7|k&
```

Unfortunately, a lot of what would be sensical is drowned out by junk at the end. The vectorizer just empty-fills the matrix after the end of the sentence. I went back and added one more 'character' to the vectorizer. It's "ascii-129", which isn't a real character, but it just means I needed to add one more bit per character. When vectorizing, the end of sentence (end of the array) marks ascii 129. When unvectorizing, the sentence stops being generated at 129. Simple enough. I would use the '\\0' character, as it's supposed to be, but there are a number of non-printable characters between ascii-32 and ascii-0.  
After another 6000 iterations, here are some samples:

```
Broden dind,00 a&amp;k aThes a}e .widiAB thefdey#)+}g/^X&gt;!/{yQ
We're Bettin, J.X: cJoJe to ZeTrUf[ISion`C):+6+]f{y%{^@k?g6=|/K[[$lX!/r|g;54]+
Ki l aBl hAman6.8q&gt;f$z\+;@&gt;
```
