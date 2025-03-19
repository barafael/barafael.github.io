+++
title = "Programming the Teensy 3.2 using a makefile"
date = 2017-11-21
+++

The Arduino toolchain is a good starting point. So many libraries, most of them
fine quality, and the editor allows you to write incorrect C++ so you can have
a simpler life...

Wait, what? That's right, the Arduino IDE does a lot of stuff to compile your
program that usually does not happen in the standard C++ compilation procedure.
For example, the IDE generates function prototypes for you, so that you can
write your functions in whichever order you like. This alone is iffy for some.
But it also automatically includes header files needed  for your libraries,
making the entire process rather opaque. This would have been reason enough for
me to look into doing it differently, but really the elephant in the room that
needed changing was the poor, often belittled Arduino IDE. The IDE simply does
not do the toolchain justice.

## Benefits from using Make

Goals:

* Use any editor and just run shell commands (or press buttons)
* Compile and flash the program immediately after checking out from version
  control, not having to set up Arduino
* Do syntax highlighting and completion based on local files
* Symbol lookup (go to definition, find usage) and all the vim/IDE/ctags
  goodness.

## The Easy Part: Using Someone Else's Makefile

In [this repo](https://github.com/apmorton/teensy-template), GitHub user
apmorton has prepared a working makefile. Only tiny adjustments are needed in
the provided makefile, like setting desired clock frequency, setting Teensy
version and enabling more compiler warning flags :)

## The Hard Part: Getting Existing Code to Compile (... and run...)

I had used numerous Arduino libraries in my existing project. Here it became
apparent how much the Arduino IDE actually did in the backgound - after
including the libraries, there was a large wall of compiler warnings and
errors. Most of them can be fixed:

* If it is a missing (undeclared) function:
  - Looking up the header file it should be located in using google or man
    pages
  - Looking for that header file in ./teensy3 or ./tools using ripgrep or
    similar
  - Including that file, and seeing if that at least removes the error
  - When all errors are fixed, taking a hard look at all the included code, and
    whether it actually does the desired job
* If the issue are duplicate function definitions:
  - Difficult one. You need to find out which of the functions to use and then
    comment out or guard (with a preprocessor definition) the others
    definition.
* Else:
  - Tough luck, you are on your own...

## The Hardest Part: Fixing Remaining Issues When The Program Compiles Fine

It is likely that after compiling with only few errors, the program still has
different behaviour than when running under Arduino IDE... In my case, example
code from I2Cdevlib was not setting an interrupt pin's pinMode and thus the
interrupts were ignored when not using the IDE. Presumably, the IDE does some
special magic to fix this up, I don't know. Setting the pinMode fixed this.

Good Luck!

P.S. there is an even harder part - keeping the toolchain up to date. Using
platformio can fix this and get you all the benefits, but there is
something to be appreciated about just running ```make flash```...
