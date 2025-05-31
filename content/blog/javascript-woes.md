+++
title = "Javascript Woes"
date = "2012-07-25"

[taxonomies]
tags=["Rant"]
+++

See if you can guess what the following Javascript code does.`for(var i=0; i < 5; i++) { var b = document.createElement("input"); b.type = "button"; b.onclick = function() { alert(i); } document.appendChild(b); }`If you guessed, "It will create five buttons which, when clicked, pop up an alert with the numbers 0, 1, 2, 3, and 4," then you're mistaken.  Javascript passes only the reference of 'i' to the b.onclick.  Instead, you'll see '5' displayed five times.  This has plagued me for the past two days, and I've yet to find an elegant solution.
