+++
title = "Let''s Build a Sharing App"
date = "2014-09-05"

[taxonomies]
tags=["Programming"]
+++

Welcome back again for hour two. Last time we made a simple read/execute/print loop. (REPL) This time we're going to add two commands, one to listen on a socket, another to send data to a listening socket. Last time we rounded off with printing. Let's review the code as we last saw it.

```

import std.stdio;
import std.file;
import std.string;

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
				break;
			case "say":
				writeln(args[1..$].join(" "));
				break;
			default:
				writeln("Command '", command, "' not recognized.");
				break;
		}
	}
}
</code>
```

Looks good. Let's start with the listening server. We want to open a TCP socket to listen for inbound connections. After we have communication, we'll echo it back to the client and close our connection.

```

import std.socket; // Add an extra import for socket stuff.
...
case "start_server":
	writeln("Opening socket on 8001");

	Socket sock = new TcpSocket();
	sock.bind(new InternetAddress(8001));
	sock.listen(1); // Wait for one connection.

	Socket client = sock.accept();
	char[1024] buffer;
	auto received = client.receive(buffer);
	client.send(buffer.idup); // char[] is mutable, but client.send requires immutable data, so we do idup to clone it.
	client.shutdown(SocketShutdown.BOTH);
	client.close();

	writeln("Done listening!");
	break;
</code>
```

Okay, let's build and test it.

```
$ dmd ./main.d
$ ./main
> start_server
Opening socket on 8001
```

Now it probably looks as though the application has deadlocked here. In some senses, it has. The process is waiting for IO. How does one supply socket data, you ask? Well, arguably the simplest way is Telnet. Telnet allows us to connect to a port and vomit raw data at it. We can open a new terminal or start a new mux/screen. We could also background the process and throw data at it, but then the output from both our programs would get tangled and it would be hard to make sure we're doing what we need to.

```
$ telnet localhost 8001
Trying ::1...
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
> foo
foo
?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????Connection closed by foreign host.
$
```

What the hell? Oh. You'll notice that in our server, we are sending a clone of the ENTIRE buffer, not just the text that was forwarded to us. Still, at the start of that exchange we saw the ~~droids~~ message we were looking for. That works well enough to me. Cutting off the extra text is left as an exercise to the reader. Let's switch back to the terminal running main.d and see what it's printing.

```
$ ./main
> start_server
Opening socket on 8001
Done listening!
>
```

Looks good! We've listened to incoming traffic. Let's cover the other side now and make something which sends a message.

```

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
</code>
```

Now compile and run. Let's run the server AND the client to see if they play well together.

```
Terminal1 $ ./main
> start_server
Opening socket on 8001

Terminal2 $ ./main
> start_client
Connecting to 8001
Done.
> quit
Shutting down.

Terminal1 $
Done listening!
> quit
Shutting down.
```

That's it. We have an application that can launch a client and a server now. This has some limitations, though. In particular, the server doesn't do anything except wait for a connection. If we were running a server it would be nice if we could do more like issue commands to shut down or to kick users. Next time, we'll do some more design work and see if we can't find some solutions.
