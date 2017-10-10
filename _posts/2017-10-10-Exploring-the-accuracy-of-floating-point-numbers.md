In [this blogpost](http://fabiensanglard.net/floating_point_visually_explained/), Fabien Sanglard explains how floating point numbers are generally displayed in modern computers (using the IEEE754 Norm).
His alternative explanation using windows and offsets is brilliant and shines light on a fact that may be overlooked when using the standard explanation: the accuracy of floating point numbers declines with a factor of 2 by each window!
That is because the same 'amount' of numbers is dispersed in any window, but the size of the windows doubles every time.

I wanted to empirically show this and wrote this short C program:

<script src="https://gist.github.com/medium-endian/49d58525be9ff7738d6a8dc281649642.js"></script>
