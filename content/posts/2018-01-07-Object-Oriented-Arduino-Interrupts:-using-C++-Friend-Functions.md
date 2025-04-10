+++
title = "Object-Oriented Arduino interrupts: using C++ Friend Functions"
date = 2018-01-07
+++

Most Arduino users won't ever need interrupts or even C++/OOP for that matter.
But for writing a nice object-oriented library or advancing your programming
skills, both are very interesting subjects. But they clash somewhat: an
interrupt is basically a global function that takes no arguments (not even
C++-```this```) and returns no value, either. It is not called by the main
program like regular functions, but instead runs when it is triggered by an
external signal, like a pin logic voltage level change. This idea has no place
in an OOP (let alone functional!) world - what object does the function belong
to? It is global and cannot see or modify anything but static data, and the
fact that it does not take arguments makes it impossible to call this function
with method syntax. If this is confusing, don't worry: in C++, a function knows
which object it belongs to by the implicit, invisible argument ```this```. If
it is still confusing, don't worry - it doesn't really matter. It just means
that interrupts and OOP do not mix well.

Unless...

## C++ Friend Functions

A Friend Function is a function that can be at global scope (i.e. not belonging
to a class) but still retain private access to all members of a specified class
as if it was a member of that class. This class has to declare the function as
a friend. Now, this function can be registered as interrupt!

There is one tiny ugly thing that remains: to access private members of a class instance, this instance has to be known by the interrupt. This can be done by having a global pointer to a class instance which is valid for the lifetime of the instance. If you attach your interrupts and set this pointer in your constructor you should be fine. Remember to detach the interrupts and set the pointer to ```NULL``` or ```nullptr``` in the destructor, if this applies to your program.

## Example Code: Simple library to interface an Arduino or Teensy device to an RC Receiver

Here is the code:

<script src="https://gist.github.com/barafael/7434737e6c1846a9107b87213ea14a09.js"></script>

For a possibly more up-to-date version: [github repo.](https://github.com/barafael/RC-Receiver-Interface/)
