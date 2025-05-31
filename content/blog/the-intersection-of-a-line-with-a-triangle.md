---
title: 'The Intersection of a Line with a Triangle'
description: ""
published: 2017-11-23
redirect_from: 
            - https://www.josephcatrambone.com/?p=967
categories: "Game, Math, Programming"
hero: ../../../defaultHero.jpg
---
Shout out to [MathJax/ASCIIMath](http://asciimath.org/) for being awesome.

### Motivation and Problem Statement

We're launching a ball with a great deal of force at a wall. We can describe our wall with four points in 3D space: \`a = \[x, y, z]\`, \`b = \[x, y, z]\`, and so on for \`c\` and \`d\`. Our ball travels along a straight line path called \`bbL\`. It's origin is \`p_0\` and it's moving towards \`p_1\`.There is some redundancy. I picked four points for our wall because it makes intuitive sense, but we're going to be applying this method on only three of those four points, the triangle \`Delta abc\`. If you feel the compulsion to run this on a square, you can easily extend the approach to two triangles.Let's begin with describing our plane. There are a few different formulations of the plane in 3D. For our purposes, the 'normal + offset' configuration is the easiest. We'll figure out the normal (think a straight line pointing out of the wall) from our three points.

### A quick review of dot and cross

I'm going to assume that the idea of cross-product and dot product are familiar, but here's a very quick review in the interest of completeness.\`a = (x, y, z)\` \`b = (x, y, z)\`\`a o. b = a_x \* b_x + a_y \* b_y + a_z \* b_z\`\`a ox b = \[\[a_y\*b_z - a_z\*b_y], \[a_x\*b_z - a_z\*b_x], \[a_x\*b_y - a_y\*b_x]]\`Note that the dot product is a scalar and the cross product is a vector.One other thing to realize: the dot product of orthogonal vectors is zero. The cross product of two vectors produces a vector that's orthogonal to both. If that's not clear, don't worry.

### The Normal

Let's get back to the wall. We've got \`a\`, \`b\`, and \`c\` and we want to figure out the normal. If these three points make up an infinite plane, then the normal will jut out of it straight towards us. Recall (or look at the notes above) that the cross product of two vectors makes an orthogonal vector. We can convert our three points to two vectors by picking one to be the start. Let's say our two new vectors are \`r = b-a\` and \`s = c-a\`. That means our normal, \`bbN\` is just \`r ox s\`! And since we picked \`a\` as our origin, our formula for the plane is \`(P - a) o. bbN = 0\` for some point \`P\`. Put differently, if we have some point \`P\` and it's on the plane, when we plug it into that formula along with \`a\` and \`bbN\` we'll get zero.

### Enter: The Line

We mentioned before that our line \`bbL\` has a start point of \`p_0\` and an end of \`p_1\`. This means if a point \`P\` is on the line, then there's some value \`t\` where \`P = p_0 + t\*(p_1 - p_0)\`. Now comes the fun part. We want to figure out where this line intersects with our plane (if it does). To do that, we'll plug in the formula for a point on our line into the formula for a point on our plane.\`(P - a) o. bbN = 0\` // Point on plane. \`(((p_0 + t\*(p_1 - p_0)) - a) o. bbN = 0\` // Replace \`P\` with the formula. \`(((p_0 + t\*(p_1 - p_0)) o. bbN - a o. bbN = 0\` // Distribute the dot. \`(((p_0 + t\*(p_1 - p_0)) o. bbN = a o. bbN\` // Add \`a o. bbN\` to both sides, effectively moving it to the right. \`p_0 o. bbN + t\*(p_1 - p_0) o. bbN = a o. bbN\` // Distribute again. \`t\*(p_1 - p_0) o. bbN = a o. bbN - p_0 o. bbN\` // Subtract p_0 o. bbN from both sides. \`t\*(p_1 - p_0) o. bbN = (a - p_0) o. bbN\` // Pull out the dot product. \`t = ((a - p_0) o. bbN) / ((p_1 - p_0) o. bbN)\` // Divide by \`(p_1 - p_0) o. bbN\` on both sides.Our final answer, then, is\`t = (bbN o. (a - p_0))/(bbN o. (p_1 - p_0))\`If the denominator is zero, there's no solution. This can happen if the plane and line segment are perpendicular. Otherwise, we can plug t back into our line equation to get some point on the plane!

### Inside the Triangle

We have a point \`P\` that's on the plane and the line, but is it inside the triangle defined by \`Delta\`\`abc\`? There's a fairly easy way to check for that. If you've got a triangle, as we have, then any point in that triangle can be described as some combination of \`a + u\*(b-a) + v\*(c-a)\`, where \`u\` and \`v\` are in the interval \`\[0,1]\`. If \`u\` or \`v\` is less than zero, it means they're outside the triangle. If they're greater than one, it means they're outside the triangle. If their sum is greater than one, it means they're outside, too. So we just have to find some \`u\` and \`v\` for \`P = a + u\*(b-a) + v\*(c-a)\`.

### Systems of Equations

It might not seem possible. We have two unknowns and only one equation. However, there's something we've overlooked. \`P = a + u\*(b-a) + v\*(c-a)\` actually has three equations. We've been using a shorthand for our points, but \`u\` and \`v\` are scalars. Really, we should be looking for a solution for this:\`P = a + u\*(b-a) + v\*(c-a)\`\`P - a = u\*(b-a) + v\*(c-a)\`\`\[\`BAM! Two unknowns, three equations. You might also recognize this to be of the form \`bbAx=b\`. You'd be correct. If there were three unknowns and three equations, we could have been fancy and used Cramer's Rule. It's not a hard thing to solve, however.\`bbbAx = b\` \`bbbA^TbbbAx = bbbA^Tb\` // Start by making \`bbbA\` a square matrix. \`(bbbA^TbbbA)^-1 bbbA^TbbbA x = (bbbA^TbbbA)^-1 bbbA^T b\` // Since it's square, it probably has an inverse. \`bbbI x = (bbbA^TbbbA)^-1 bbbA^T b\` // Cancel the inverse. \`x = (bbbA^TbbbA)^-1 bbbA^T b\` // Simplify.And now we've got x in terms that we know (or can calculate)!

### Inverse of a Square Matrix

\`(bbbA^TbbbA)^-1\` looks like a mess, but it's not as bad as it seems. I'm going to multiply it out and simplify it again.\`bbbA^T bbbA = \[\`Whew.

### Closing: Just Show Me The Code

The moment for which you've been waiting. Here's an EMScript6 implementation of the Point and Triangle objects.A Gist is available on GitHub at <https://gist.github.com/JosephCatrambone/578c22f6e507dc52420752013a45b92b.js> or you can play with this interactively on JSFiddle:

<!-- Uncomment this to fetch the code from GitHub and run it here.
<h3>A Small Demo</h3><p></p>Embedding the above code on the page:<p></p><div style="border: black 1px">
<canvas id="canvas" width="400" height="300"></canvas>
</div><p></p><script type="application/javascript">
// Grab and run the gist:
function asyncHTTPRequest()
{
	var gistURL = "https://gist.githubusercontent.com/JosephCatrambone/578c22f6e507dc52420752013a45b92b/raw/ecd66916fe50875a583e7a8b9caa9ee171ec328c/TriangleLineIntersection.js";
	var xmlHttp = new XMLHttpRequest();
	xmlHttp.onreadystatechange = function() {
		if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
			eval(xmlHttp.responseText);
			window.requestAnimationFrame(draw);
		}
	}
	xmlHttp.open("GET", gistURL, true); // true for asynchronous
	xmlHttp.send(null);
}
//asyncHTTPRequest();
</script>
-->
