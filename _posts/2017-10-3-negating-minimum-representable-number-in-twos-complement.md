Usually, we expect that negating a number yields a number which is of the same absolute value, but with a flipped sign. However, there exists a curious behaviour in many programming languages where this property
does not hold for one special case. You might have come across this if you (like me) tried to use a constant Integer.MIN_VALUE (or somesuch) to simulate something like negative infinity. For example, this would be useful
in a min-max algorithm with alpha-beta pruning (don't worry if you have never heard of this, though).

## What's the issue?!

If you negate the lowest representable int in (for example) java, you could expect it to become
nteger.MAX_VALUE. This however, is literally far from the truth! Instead, you get... Integer.MIN_VALUE again. How the flip did that happen?
