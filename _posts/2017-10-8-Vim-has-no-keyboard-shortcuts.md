In (this StackOverflow
post)[https://stackoverflow.com/questions/1218390/what-is-your-most-productive-shortcut-with-vim],
a question about the 'Most productive Vim keyboard shortcut' sparks a long
discussion about Vim and productivity. In (this
answer)[https://stackoverflow.com/a/1220118], titled _'Your problem with Vim is
that you don't grok vi.'_ which is easily one of the most influential pieces of
Vim documentation and intro material on the internet, user Jim Dennis takes the
question apart and shows how Vim is not about shortcuts but about a language
for manipulating text. If you read and understood this answer, you probably
don't need to read this post here, since it is just my personal take on the
Vim philosophy.

## 'Shortcuts' and Vim

A shortcut in any other program is like a certain event that causes the program to do a certain action. Like ```F5``` in a browser or ```CTRL+D``` in most terminals.

In Sublime Text, there is a shortcut to delete a line: ```CTRL+UP+K```. This cannot be remembered easily, involves ```CTRL```, is generally hard to do and impossible with just one hand (yes, I know there is also ```CTRL+X```). In Vim, the same action happens on ```dd```. This seems arbitrary too, but actually it fits well into the rest of the vim language, where 'd' means to delete something and a double tap means to apply an action on an entire line. Moreover, this command can be combined with others as in a real programming language and has the side effect of storing the line in some register. This is not a shortcut, but an action from the language that makes up the way you manipulate text in vim. 

## Actions and Macros

The fact that actions in vim can be randomly made up on the fly indefinitely, and combined with each other, is already very powerful. But there are even more benefits: since a text edit is just a sequence of characters constituting the action, the use of a macro is simply recording and playing back such a sequence of characters. Because vim is awesome, it stores a macro in some register (bearing the name you gave to the macro) so you can easily access, save for later, or modify the actions a macro does. You can also call a macro from itself for multiline edits - to vim it is simply a long chain of characters that it must interpret. Macros can be nested - you can call a macro b from within macro a since macro invocation is only a sequence of characters. Finally, it is easy to parameterise macros with register contents.

For example, you might have a macro 'a' that finds a word matching a certain pattern and yanks it to register 'w', and then calls a macro 'b' which finds another word and substitutes the contents of the 'w' register. This could be done with regex back references, but to do it with macros in vim feels much cleaner to me - it is like programming with functions.

## Why all the ramble about Macros?

Because all the other text editors which use shortcuts cannot really do this and that is precisely the benefit of not having shortcuts, duh!

