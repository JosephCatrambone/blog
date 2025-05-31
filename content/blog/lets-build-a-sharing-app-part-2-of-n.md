+++
title = "Let''s Build a Sharing App"
date = "2014-09-08"

[taxonomies]
tags=["Programming"]
+++

Welcome back to Let's Build. With connecting and command processing out of the way, we open today with a bit of planning and design. First, let's see how the code looked at the end of the last lesson.

```

// NOTE: I replaced the tabs with two spaces in this code bar because it was getting a little wide for the website display.  I may change it back in a later example.
import std.stdio;
import std.file;
import std.string;
import std.socket;

void main() {
  string command = "";
  bool run = true;
  while(run) {
    string line = chomp(readln());
    string[] args = split(line);
    command = args[0];
    switch(command) {
      case "quit":
        run = false;
        writeln("Shutting down.");
        break;
      case "say":
        writeln(args[1..$].join(" "));
        break;
      case "start_server":
        writeln("Opening socket on 8001");

        Socket sock = new TcpSocket();
        sock.bind(new InternetAddress(8001));
        sock.listen(1);

        Socket client = sock.accept();
        char[1024] buffer;
        auto received = client.receive(buffer);
        client.send(buffer.idup);
        client.shutdown(SocketShutdown.BOTH);
        client.close();

        writeln("Done listening!");
        break;
      case "start_client":
        writeln("Connecting to 8001");
        Socket sock = new TcpSocket();
        sock.connect(new InternetAddress(8001));
        sock.send([1, 2, 3, 4]);
        char[1024] buffer;
        sock.receive(buffer);
        sock.close();
        writeln("Done.");
        break;
      default:
        writeln("Command '", command, "' not recognized.");
        break;
    }
  }
}
</code>
```

There are a few things which bother me about the material above. First, we've got magic numbers like a hard coded port and buffer size. More notable to me, though, is we have one gigantic main function. I think that going forward it would be nicer to spin off the behaviors into individual functions. Let's do that, but without modifying our switch case. We'll keep it simple at the start, splitting off only the quit function.

```

...

void main() {
	string command = "";
	bool run = true;
	while(run) {
		string line = chomp(readln());
		string[] args = split(line);
		command = args[0];
		switch(command) {
			case "quit":
				quit(&run, line, args);
				break;
			case "say":
... // All the same
				break;
		}
	}
}

void quit(bool* run, string line, string[] args) {
	*run = false;
	writeln("Shutting down.");
}
</code>
```

There's nothing that prevents us from doing the same thing for each of the say and start functions, but that gigantic switch is kinda' gross. We can simplify this and borrow a trick from Python, which lacks the switch statement (last time I checked). Instead, we create an associative array which maps strings into function pointers. Our big branch statement becomes something like...

```

	while(run) {
		string line = chomp(readln());
		string[] args = split(line);
		command = args[0];
		functionTable[command](&run, line, args);
	}
</code>
```

Much better, no? This also gives us an intuitive way of dealing with unknown commands. Any decent associative array will also support getting a default value, which for us will be the 'command not found' message. Let's combine all these things together. Our code now is something like this:

```

import std.stdio;
import std.file;
import std.string;
import std.socket;

void main() {
	string command = "";
	bool run = true;
	void function(bool*, string, string[]) commandPointer;
	typeof(commandPointer)[string] commandTable; // Delegates can refer to non-static functions.

	// Set up the command table
	commandTable["quit"] = &quit;
	commandTable["say"] = &say;

	while(run) {
		string line = chomp(readln());
		string[] args = split(line);
		command = args[0];
		commandTable.get(command, &unrecognized)(&run, line, args);
	}
}

void unrecognized(bool* run, string line, string[] args) {
	writeln("Command not recognized.");
}

void start_server(bool* run, string line, string[] args) {
	writeln("Opening socket on 8001");

	Socket sock = new TcpSocket();
	sock.bind(new InternetAddress(8001));
	sock.listen(1);

	Socket client = sock.accept();
	char[1024] buffer;
	auto received = client.receive(buffer);
	client.send(buffer.idup);
	client.shutdown(SocketShutdown.BOTH);
	client.close();

	writeln("Done listening!");
}

void start_client(bool* run, string line, string[] args) {
	writeln("Connecting to 8001");
	Socket sock = new TcpSocket();
	sock.connect(new InternetAddress(8001));
	sock.send([1, 2, 3, 4]);
	char[1024] buffer;
	sock.receive(buffer);
	sock.close();
	writeln("Done.");
}

void say(bool* run, string line, string[] args) {
	writeln(args[1..$].join(" "));
}

void quit(bool* run, string line, string[] args) {
	*run = false;
	writeln("Shutting down.");
}
</code>
```

A thousand times better. I'd planned on getting more into the key exchange and crypto stuff, but one trick per lesson seems like enough. Join us next time and we'll interface with OpenSSL.
