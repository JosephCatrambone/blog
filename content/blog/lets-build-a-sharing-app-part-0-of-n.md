---
title: 'Let''s Build a Sharing App: Part 0 of n'
description: ""
published: 2014-09-04
redirect_from: 
            - https://www.josephcatrambone.com/?p=573
categories: "Programming"
hero: ../../../defaultHero.jpg
---
Welcome to Let's Build. In this series, I'm going to walk through the creation of an application from concept to finished product. We'll be building a file sharing application in D. Why D? D is a time-tested, compiled, C-like language, and I have no idea how to use it. Learning for everyone! In the course of this project you'll likely see a lot of Python-isms written in D by someone with formal training in C and Java. It should make for some deliciously haphazard semantic salad. This series will target people with a basic familiarity of ADA-Class languages who know something of flow control and variables. We brake for nobody. It should illuminate the thought process of a professional (but not very good) software developer, and the mechanisms/brute-force by which a goal is achieved.

### What are we building?

In short, I'd like an application which can securely share medium to large files with groups of arbitrary size. Revision history isn't important to me, so it will be less like git and more like BitTorrent. Perhaps Napster is a reasonable descriptor, but I don't like the piracy connotations. Let's get to designing.

What features do we want our application to have? What do we want to be able to do?

- Host a server effortlessly, connect effortlessly.
- Invite people.
- Search for files owned by many peers.
- Send files peer-to-peer in a secure fashion, piecewise or in whole. MAYBE fall back to centralized distribution if we can't get things right.
- Broadcast messages from user-to-user or from user-to-all.

That seems to touch nicely the core of the matter. If we can accomplish this, embellishing things on the client side shouldn't be too hard. That might include ignoring someone in chat, or doing private messages (which is really just sending a specific kind of data peer-to-peer). Most of these things could be handed by a pretty simple command-line or curses interface. This would also extend nicely to a GUI layer set on top. We'll have to expand each of these elements later on and make sure they fit together, but in the meanwhile, let's drill down and lay out a simple command-parsing interface.

<!--h3>Setting the Cornerstone</h3>
<p>
More or less all of the operations involves sending or receiving data.  As a server, you need to listen for connections.  As a client, you need to send and receive data.  Since security is one of the essential pieces, we need to figure out how we're going to send and receive data.  We can start by defining how we identify a different endpoint.
<blockquote>An endpoint is defined by the tuple of its public key and IP address.</blockquote>
Our send and receive stubs, then, are something on the lines of, <code>connect(pubKey, IP, PORT) : connection</code>, <code>send(connection, data) : status</code>, <code>listen(PORT) : connection</code>, <code>receive(connection, data) : status</code>.  Each of these should probably also have a timeout of some sort so we can bail if needed, and maybe the return value should be success/fail and have everything passed as a reference.  I dunno!  Let's see if we can make an app which does this much.
</p-->

### Ohai World

```

import std.stdio;
import std.file;
import std.string;

void main() {
	string command = "";
	bool run = true;
	while(run) {
		command = chomp(readln());
		switch(command) {
			case "quit":
				run = false;
				break;
			default:
				writeln("Command '", command, "' not recognized.");
				break;
		}
	}
}
</code>
```

Let's compile and test this app.

```
$ dmd ./main.d
$ ./main
> derp
Command 'derp' not recognized.
> quit
Shutting down.
$
```

Hooray! We can read whole word commands. That's a little bit limited, though. It would be nice if we could issue commands of the form, "exec \[something]". Let's add the 'say' command and split up our arguments.

```

	// Replace `command = chomp(readln());` with...
	string line = chomp(readln());
	string[] args = split(line);
	command = args[0];
</code>
```

So, if we are prompted and enter "This is my command to you!", we have access to the variables line (This is my command to you!), args (\[This, is, ..., you!]), and command (This). Let's add a function 'say', which takes everything after the first argument (the command), and writes it to the screen.

```

	// Rest of case statements up here.
	case "say":
		writeln(args[1..$].join(" ")); // Write words 1 through the end, joined by a single space.
		break;
	default:
	// Rest of code down here
</code>
```

Compile and test.

```
$ dmd main.d
$ ./main
> say Hello.  My name is bob.
Hello. My name is bob.
> quit
Shutting down.
```

Smashing. I think that's enough for a first foray into D. I'm going to have a sandwich and, when we get back, I'll do a simple socket connection.
