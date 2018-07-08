In [this
blogpost](http://fabiensanglard.net/floating_point_visually_explained/), Fabien
Sanglard explains how floating point numbers are generally displayed in modern
computers (using the ``IEEE754`` Norm). His alternative perspective using
buckets and offsets is brilliant and explains a fact that may be
overlooked when using the standard explanation: the accuracy of floating point
numbers declines with a factor of 2 by each bucket!  That is because the same
'amount' of numbers is dispersed in any bucket, but the size of the bucket
doubles every time.

I wanted to empirically show this and wrote this short C program:

<script src="https://gist.github.com/barafael/49d58525be9ff7738d6a8dc281649642.js"></script>

## One reason Unions are useful

This program uses a C union. Unions allow you to use the same memory for
multiple members, and every time you write one member, it also changes the
others. This may seem useless, but it is not. In this case, it allows you to
write to memory using the memory representation of ``float``, while reading the
same content of memory as individual bits. The union has 2 members, one
allowing to write and read a float, the other resembling the bit pattern of a
float as defined in the ``IEEE754`` Norm. After writing to the float, you can
read and manipulate the exponent and mantissa from the embedded struct.

## Printing any memory content as binary [print_any(content pointer, size)]

Casting the pointer to the memory content to a byte pointer (``uint8_t *``)
allows us to index individual bytes. To access individual bits, we need to
generate a bit mask for each of them. We simply start at ``0b1000 0000`` and
keep shifting left while using the index to access a single bit and print it.
Since the bit at position ``n`` will still be interpreted as being of value ``2^n``
we need to check if it is set and print a ``1`` if this is the case, hence the
ternary operator.

We need this function to inspect a floating point number. 

## Floating point numbers are discrete, too... [increase_float(content)]

We imagine floats as being very fine-grained. That is exactly what they are -
they do not allow for arbitrary accuracy, they are not what mathematicians
would call _dense_.

That means, we can take a float and increase it by exactly one tiny amount,
yielding the next-larger float in that bucket! That is what the function
``increase_float`` does.

We construct a ``float_parts`` union using the float as content. Then, we look
at the mantissa: if it is filled to the brim, we reset it to 0 and increase the
exponent by one. Otherwise, we simply increase the mantissa.

## Putting it together: How accuracy declines with growing exponent [print_accuracy(low, high)]

Here we keep increasing a float, and when the 'step size' changes, we log it.
We also print the factor between the last and the current step size.

This function runs way faster than I expected for some reason - it visits every
float there is between ``low`` and ``high``.

There you have it! Empirical demonstration of how inaccurate floats (and
doubles, just later) really are.
