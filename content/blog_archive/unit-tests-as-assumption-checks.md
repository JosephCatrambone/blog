+++
title = "Unit Tests as Assumption Checks"
date = "2018-01-05"

[taxonomies]
tags=["Programming"]
+++

Testing is good. Testing is right. Testing helps us find and fix mistakes before they become so rooted in our applications that new behaviors start to depend upon them. We should not, however, test for the sake of testing. There's an oft neglected balance between allowing one's application to grow and being well tested. If you have 10k unit tests that cover 100% of your 10 functions, there is a bit of a problem when you get a ticket which asks you to change your assumptions. Testing is, then, a double edged sword: it allows you to move faster knowing the accuracy of your solution, but it holds you back if you build multiple tests which are variations on a theme. The two ways I've found which are most ideal for adding a unit test are as follows:

> Good tests affirm assumptions about behavior.

Any time you find yourself unsure about the documentation of the libraries on which you depend is a good time to add a unit test. I was adding SVD functionality to Koma and found that the U, S, and V values returned by EJML and JBlas weren't obviously clear. Ideally, the matrix A should be the product of U \* S \* V^T, but is the value of V returned by these functions V or V^T? If you have to open up an interactive shell to try something, make it a unit test!
