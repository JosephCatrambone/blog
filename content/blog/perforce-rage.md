---
title: 'Perforce Rage'
description: ""
published: 2011-12-07
redirect_from: 
            - https://www.josephcatrambone.com/?p=24
categories: "Rant"
hero: ../../../defaultHero.jpg
---
I've tried to put in words my contempt for Perforce a few times.  I believe now that were I to manifest fully the immensity of my hatred for this piece of software it would blanket the earth in a sea of ash and decay.  It would start with the sound of weeping and lamentation, like a child crying against the ambient noise of a playground.  The plague wind of silence would spread, lifting the hopeful and animated spirits from their frames and casting them into the darkness. Joy now ended, the screaming would grow from this deathly silence, filling the skeletal void with a boiling noise of apocalypse.  Tumultuous and unending, the world would burn and the fields would turn to salt in the wake of this horror incarnate.With that said, here are some of my real gripes with Perforce:

1. Setting the active client in the command line is troublesome.  I struggled today to realize that \`p4 set P4CLIENT=myclient_engbuild\` was not working because P4CLIENT should not be capitalized.  It is printed as P4CLIENT in all the documentation, but you have to use p4client if you want it to accept changes.
2. If you leave out the equals on \`p4 set client myclient_engbuild\`, you will set an environment variable called 'client', then wonder why all your commits and changes are not happening where you expected.
3. If you need to edit your changelist in the GUI client on Windows 7, good luck.  You can add files and remove them, but not add a description.  It's faster to just completely delete your change list and redo all your changes.
4. You cannot delete your changelist until you delete all of the changes on it.  The option doesn't even appear until the changelist is empty.  It is not made obvious why one cannot delete a changelist, or it is possible.  However, one can search Google and find the required information.
5. Once you get to a few million files across different projects, UI latency becomes intolerable.  On my Q6600 desktop with 4 gigs of ram, I need to wait between four and ten minutes (std dev 2 min) after EVERY MOUSE CLICK.  In defense of Perforce, I suspect it's the case that their software is fine, but my organization has a terrible codebase structure.  Is it reasonable for a lot of code to bog down a subversion server?

Having vented that, I'm sure there are reasonable solutions to all of these problems.  Unfortunately, the solutions are not obvious, and the problems are recurring.  I'd like to think I've given Perforce a fair shot, but it still falls terribly short when compared to every other piece of source control software I've used.
