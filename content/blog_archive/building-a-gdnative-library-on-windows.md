+++
title = "Building a GDNative Library on Windows"
date = "2017-12-20"

[taxonomies]
tags=["Game,","Programming"]
+++

Godot is a really honkin' neat engine. If you haven't tried it, I strongly recommend playing around with it. Take a look at <https://godotengine.org>.I found myself in a position where I needed to build a native library. Here's my experience doing that on Windows. I can't attest to the accuracy or repeatability of these steps, but I'm leaving them here so I can revisit them when I need to. Just remember: GDNative is a way to call into shared libraries from Godot. NativeScript is the other way -- native code that can call into Godot.

### Overview:

- Setting up Virtualenv + Scons
- Acquiring and Building Godot
- Building a Native Module

### Prerequisites and Setting Up

#### You will need:

- Microsoft Visual Studio
- Python 3 + pip (or scons installed)
- Git
- A really good reason to need to build a native library

Godot is built with Scons. The process is relatively painless compared to the dependency hell that you can get into when building other tools, but it's not without challenges. I'm going to assume that you've installed Microsoft Visual Studio and can run the following on the command line:`cl.exe`Your output should be this:`(scons) D:\Source\TerminusExperiment\CPU_v1>cl Microsoft (R) C/C++ Optimizing Compiler Version 19.00.24215.1 for x64 Copyright (C) Microsoft Corporation. All rights reserved.

usage: cl [ option... ] filename... [ /link linkoption... ]`If you don't see that, you'll probably need to search for a shortcut to "VS2015 x64 Native Tools Command Prompt". That will, in turn, include a script to call the following bat file: "%comspec% /k ""C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\vcvarsall.bat"" amd64"

#### CHECKPOINT: Visual Studio is installed.

Next is Scons. I'm not going to go into any depth about installing and setting up Python on Windows, but I've had more luck using [Chocolatey](https://chocolatey.org/) than Anaconda. Install Python 3, pip, and virtualenv.Make a new virtual environment somewhere with `python -m venv my_scons_venv` (Mine is called just 'scons' and is stored in C:\\Users\\Jo\\virtualenvs).Activate the new virtualenv. If you're on Windows, that means calling C:\\Users\\Jo\\virtualenvs\\scons\\Scripts\\activate. (This is approximately equivalent to Linux or OSX's `. ./scons/bin/activate`)Install scons in your virtual environment. `pip install scons`

#### CHECKPOINT: Scons is installed. You can build Godot.

### Downloading and building Godot with Scons

Now we'll pull the Godot source. There may be a way to make do without this, but I've not had luck.I keep my projects in D:\\Source. I opened my command prompt and did `git clone https://github.com/godotengine/godot.git`Get some coffee while the repo comes down.Change into the Godot directory. Build Godot with `scons platform=windows`.Wait.You should see your executables in "D:\\Source\\godot\\bin". Try double clicking on the tools.64.exe if it's built. Fun, eh?

#### CHECKPOINT: Godot is built from sources.

### Building The CPP Shared Library

Go back to your source folder. For me, that's "D:\\Source". Now we'll clone godot-cpp so we can build our .lib file. `git clone https://github.com/GodotNativeTools/godot-cpp`We're going to edit the SConstruct file.I set my "godot_headers_path" to `godot_headers_path = ARGUMENTS.get("headers", os.getenv("GODOT_HEADERS", "D:\\Source\\godot\\modules\\gdnative\\include"))`Note that it might be necessary to use double backslashes because Windows uses the wrong slash direction for their paths. Note that godot_headers_path points into the Godot build we cloned and into the GDNative module's include folder.Update the "godot_bin_path" to point to our executable. `godot_bin_path = ARGUMENTS.get("godotbinpath", os.getenv("GODOT_BIN_PATH", "D:\\Source\\godot\\bin\\godot.windows.tools.64.exe"))`Invoke `scons platform=windows generate-headers=yes`.There will be a short span while the lib is created. When it's all done, check your bin folder. You should see "godot_cpp_bindings.lib".

#### CHECKPOINT: You've got the godot_cpp_bindings library built

Make a new folder. I put it in my project directory, "D:\\Source\\Terminus\\CPU_v1\\". CPU_v1 will be my native module. My game involves doing some CPU emulation.Into that directory, copy D:\\Source\\godot-cpp\\include. Also make a folder called 'lib' and put "godot_cpp_bindings.lib" in it.Your directory structure should look like this:

```
D:\Source\TerminusExperiment\CPU_v1
- include
 |- core
 | |- AABB.hpp
 | \- ...
 |- AcceptDialog.hpp
 |- AnimatedSprite.hpp
 \- ...
- lib
 \- godot_cpp_bindings.lib
- src
 \- init.cpp (THIS IS YOUR CPP FILE!  Get a sample one from [x].)
```

Finally, we can build our CPP file using this command in the root of CPU_v1: `cl /Fosrc\init.obj /c src\init.cpp /TP /nologo -EHsc -D_DEBUG /MDd /I. /Iinclude /Iinclude\core /ID:\Source\godot\modules\gdnative\include`Make a good note of those trailing '/I's. The specify the include folders. If you get a message about "Missing whatever.h" then you've got one wrong./Fosrc\\init.obj specifies the output object. /c src\\init.cpp specifies our source file.

#### CHECKPOINT: We have our .obj file from our init.cpp!

Last step, we can link our two objects together. `cl /LD lib\godot_cpp_bindings.lib src\init.obj /link /DLL /OUT:init.dll`This will take our lib and our source object and will produce init.dll -- something we can use in Godot's Native library.
