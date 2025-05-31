---
title: 'Implementation Notes on Calculus for Computational Graphs'
description: ""
published: 2016-12-31
redirect_from: 
            - https://www.josephcatrambone.com/?p=895
categories: "Math, Programming"
hero: ../../../defaultHero.jpg
---
In an earlier update, I wrote about implementing computational graphs to learn new programming languages.  I took the time recently to implement one for Kotlin, as I was having a hard time moving forward in Rust.  Here are some implementation decisions I made and how I feel about them looking back:

### Implementing operators as standalone classes.

When I build Aij (the Java compute graph), Rust Compute Graph, and the first revision of CG4j, I had a graph class which contained 'add node'/'add operator' methods which appended lambdas to a big list of operations for forward and reverse nodes.  This meant that the graph class was nice and portable.  I could drop it into new projects and be on my way.  The downside comes from serialization.  When using Java's internal serializer, Lambdas can't get converted, so saving and restoring the graph automatically isn't possible.  Another downside to this is lambdas incur a small performance overhead, from what I see, even in Rust where we incur a heap allocation unnecessarily.  The solution: define an operator or node class and subclass it.

### Returning Nodes or Integers?

Depending on whether you opt to use nodes or node indices, you might find yourself passing either a node index and a graph pointer to your constructors OR a node.  Passing around a graph is inconvenient, especially if you have to load one from disk.  It means you need to deal with the whole chicken-and-egg thing.  Passing around a node is easier by far, but it means you have to handle saving out the nodes which might exist on multiple paths and, more problematically...

### Recursively computing the forward propagation calls.

One thing I did do correctly in the Rust implementation was calculating the nodes in order.  Each node had an ID/index, and when doing forward-prop I'd just iterate from left-to-right across the node list and calculate each one.  In the Kotlin implementation, I opted to memoize and recursively compute the nodes.  That shouldn't have been an issue, in theory, since I was caching results, but the function calls aren't free, and it means getting stack-traces like this:

```
 at com.josephcatrambone.cg4j.Tensor.add_i(Tensor.kt:256)
 at com.josephcatrambone.cg4j.AddNode.forwardOperation(Node.kt:87)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:33)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:30)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:30)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:30)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:30)
 at com.josephcatrambone.cg4j.Graph.forward(Graph.kt:30)
 at com.josephcatrambone.cg4j.Graph.forward$default(Graph.kt:19)
 at com.josephcatrambone.cg4j.RNN.predict(RNN.kt:129)
 at com.josephcatrambone.cg4j.RNN.predict$default(RNN.kt:97)
 at com.josephcatrambone.cg4j.MainKt.testRNN(Main.kt:107)
```

The single advantage to this approach is that unused code paths don't get called.  If you have nodes, A, B, ... X, Y, Z, and Z = A+B, then when you compute using the non-memoized version you're actually going to be computing the values for everything between A and Z, as opposed to just A, B, and Z for the recursive version.  Depending on how much of your graph is used at any one time, this can be a big saving.  Not sure who the clear winner is here.
