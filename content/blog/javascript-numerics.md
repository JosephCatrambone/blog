+++
title = "Javascript Numerics"
date = "2013-06-12"

[taxonomies]
tags=["Rant"]
+++

I ran into an interesting issue very recently. I needed a way to check if a string was a number. Piece of cake.  Skip to the bottom for the solution.>> parseInt("10") > 10So far so good.>> parseInt("hurr"); > NaNExcellent. With a little Underscore JS action...>> !\_.isNaN(parseInt("w") > falseCool.>> parseInt("w10") > NaNStupendous.>> parseInt("10a") > 10Wait what?>> parseInt("10b") > 10Uhh. Must be interpreting the 'a' or 'b' as hex?>> parseInt("10w") > 10Fuck.Well what about Underscore's internal \_isNumber?>> \_.isNumber("10k"); > falseHAH! Great. Let's check the base case...>> \_.isNumber("10"); > falseDamnit.Well, after a great deal of deliberation, I threw together the following effective, if gross method:\_.reduce(x, function(memo, val) {return memo && !\_.isNaN(parseInt(val))}, true);Or, wrapped more nicely:isNum = function(x) {return \_.reduce(x, function(memo, val) {return memo && !\_.isNaN(parseInt(val))}, true)}>> isNum("10a") > false>> isNum("10") > true>> isNum(10) > trueAddendum: This method may plausibly be faster, considering Underscore defers to the native every when available, and allows for early-out:isNum = function(x){ return \_.every(x, function(y){ return !\_.isNaN(parseInt(y)); }); }
