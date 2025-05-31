+++
title = "Assembly and CPU Design"
date = "2017-03-11"

[taxonomies]
tags=["Game,","Programming"]
+++

I revived a prototype platformer I started in college called Terminus. For me, it was mostly an excuse to fiddle with chatbots as a method of driving the plot forward, but I found an extra interesting twist: an in-game assembly language that allows the user to solve programming problems.Here's a general overview of the CPU:

- 256 bytes of memory.
- 1 byte instructions with up to three operators.
- No registers, but every instruction allows dereferencing and writing to an arbitrary memory output.

Here's some sample code which puts calculates if a value on \_BUS0 is prime or not, writing 1 for primes, 2 for composites, and 0 for not yet finished.

```
;; Prime directive.
;; Assorted one-byte primes are coming in on BUS0.
;; Write 1 to BUS1 if they're prime numbers or 2 if they're composite.
;; Keep at 0 while processing.
;; 0 and 1 are not prime.
JMP _START ; Set aside some protected memory at the start of the program.
BUS0: 0x00
BUS1: 0x00
TARGET: 0x00
DIVISOR: 0x00
TEMP: 0x00
;; Begin.
START:
; Save the values from _BUS0 just in case it changes.
; Init.
MOV *BUS0 TARGET
MOV 2 DIVISOR
MOV 0 BUS1
; Main loop.
LOOP:
MOD *TARGET *DIVISOR TEMP
JE *TEMP 0 DIVISOR_FOUND
; Otherwise this doesn't divide evenly
ADD 1 *DIVISOR DIVISOR ; Increase our number by 1.
JE *DIVISOR *TARGET NO_DIVISOR_FOUND ; If we're at the target, no divisor.
JE 0 0 LOOP
DIVISOR_FOUND:
MOV 1 BUS1
JE 0 0 START
NO_DIVISOR_FOUND:
MOV 2 BUS1
JE 0 0 START
```

It's a nice change of pace from ML work to do some CPU design.  The choice to leave off a register actually turned out to simplify the code, too.  Here are some screenshots:[![](./img/wp-content-uploads-2017-03-terminus_0-300x169.png)](http://www.josephcatrambone.com/wp-content/uploads/2017/03/terminus_0.png) [![](./img/wp-content-uploads-2017-03-terminus_1-1-300x169.png)](http://www.josephcatrambone.com/wp-content/uploads/2017/03/terminus_1-1.png)
