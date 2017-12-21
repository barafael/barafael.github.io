The Arduino toolchain is a good starting point. So many libraries, most of good
quality, and the editor let's you write incorrect C++ so you can have a simpler
life. 

Wait what? That's right, the Arduino IDE does a lot of stuff to compile your
program that usually does not happen in standard C++ compilation. For example,
it generates function prototypes for you, so that you can write your functions
at whichever point you want. This alone is iffy for some. But it also
automatically includes header files needed  for your libraries, making the
entire process rather opaque. This would have been reason enough for me to look
into doing it differently, but really the elephant in the room that needed
changing was the poor, universally belittled Arduino IDE. The IDE simply does
not do the toolchain justice.

# Benefits from using Make

The goals are:

* being able to use any editor and just run shell commands (or press buttons)
* to compile and flash the program being able to compile my programs
* immediately after checking out, not having to set up Arduino being able to do
* syntax completion based on local files being able to follow the symbol under
* the cursor and all the IDE/ctags goodness.

The last 2 points aren't easily possible when the header files are located
somewhere in /usr/ or anywhere else, but when everything is local, any IDE
worth it's salt will be able to do this.

# The Easy Part: Using Someone Else's Makefile

In [this repo](https://github.com/apmorton/teensy-template), GitHub user
apmorton has prepared a working makefile. Only tiny adjustments are needed in
the provided makefile, like setting desired clock frequency, setting Teensy
version and enabling more compiler warning flags :)

# The Hard Part: Getting Existing Code to Compile (... and run...)

I had used numerous Arduino libraries in my existing project. Here it is
apparent how much the Arduino IDE actually did for us - after including the
libraries, there was a large wall of compiler warnings and errors. Most of them
can be fixed by:

* if it is a missing (undeclared) function:
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

# The Hardest Part: Fixing Remaining Issues when the Program compiles Fine

It is likely that after compiling with only few errors, the program still has
different behaviour than when running under Arduino IDE... In my case, example
code from I2Cdevlib was not setting an interrupt pin's pinMode and thus the
interrupts were ignored when not using the IDE. Presumably, the IDE does some
special magic to fix this up, I don't know. Setting the pinMode fixed this.

Good Luck!
