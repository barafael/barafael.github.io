Usually, we expect that negating a number yields a number which is of the same
absolute value, but with a flipped sign. However, there is a curious
behaviour in many programming languages where this property does not hold for
one special case. You might have come across this if you (like me) tried to (mis)use
a constant Integer.MIN\_VALUE (or somesuch) to simulate something like negative
infinity. For example, this could be useful in a min-max algorithm with
alpha-beta pruning.

## What's the issue?!

If you negate the lowest representable int in (for example) java, you could expect it to become
Integer.MAX\_VALUE. This however, is literally far from the truth! Instead, you get... Integer.MIN\_VALUE again. How the flip did that happen?

In order to explain this behaviour, we have to dig a bit deeper into the way signed integer numbers are represented in memory:

### Two's Complement

If you already know how Two's Complement works, just skip this. If not, here is a short explanation:

In order to represent both positive and negative numbers, we need to somehow
account for the sign of the number we are representing.  This can be done in
multiple ways, but Two's Complement has emerged as the most successful
approach. Basically, numbers are still just binary strings of bits, but the
Most Significant Bit (MSB) (the bit representing the largest power of 2) counts
negatively. For example, if we have the 8-bit binary number ´´´0001 0011´´´, 19
in decimal, everything is normal because the MSB is zero.  The number ´´´1001
0011´´´ however would be interpreted as (-128) + 16 + 2 + 1, -109 in decimal.

Determining the sign of a number becomes very simple: if the MSB is set, the
number is negative. Zero is represented as 0000 0000, negative one as 1111 1111
(the other 7 bits are needed to offset the large 'negative weight'), and the
largest positive number is 0111 1111.

### Negating a Number in Two's Complement and Ranges of Representable Numbers

The lowest representable number in Two's Complement is 2^(Place of MSB), while
the largest number is only 2^(Place of MSB) - 1. That is because the positive
numbers also need to include zero.

Negating a number in Two's Complement is easy: start at the right (Least
Significant Bit LSB) and copy all zeroes and the first one. Then, invert the
rest.

### Bringing it together

We have seen how negative numbers can be stored. Now we have everything that is needed to understand the behaviour shown above where ´´´-a = a´´´ (with 'a' being some negative number).


