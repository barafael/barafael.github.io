Errors (not human errors, but literally error conditions) can happen very
quickly even in simple Arduino projects.  For example, an external library
might sometimes fail to load data from memory due to unreliable hardware (like
with the MPU6050).  In this case, if you have a serial connection, you might
see the error description printed there. But what if you have no serial
connection because you are using your board without having a computer
connected?

Using the LED and blinking in different patterns is not a great solution,
because you have to know or find out what the patterns mean, but it is often
the best solution available (and real hardware manufacturers often choose the
same method!).  In this blog post, I want to show a simple and somewhat elegant
way to do this.

Here is the code: <script
src="https://gist.github.com/barafael/d1a09af50de218b90638068cf394d7cb.js"></script>

Pretty self-explanatory! If you haven't worked with header files (like
``error_handling.h``) before: they are really simple. Before your code gets
turned into binary for the arduino, the content of the header file is placed at
it's corresponding ``#include``. Simple as that. This way, we can define our
error handling functions without cluttering our main program. The header file
must be in the same directory as the main sketch. You can rename the main
sketch, but if you rename the header file, you must also remember to change
it's ``#include`` for the new header name.

