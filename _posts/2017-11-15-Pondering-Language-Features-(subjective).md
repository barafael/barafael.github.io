This is an attempt to define some language features of a sane and simple, yet powerful programming language.
As such it is COMPLETELY subjective, because I have no formal education about this.
But I thought long and hard about it, and if anything, it will be fun for me to read this in a while when I might know more about the topic.

# Some General Elements of C, C++ and Rust

I want this language to be essentially like C, but with more modern concepts and abstractions.
Here are some properties that I think it might have:

* No garbage collector
* Static Typing (unlike C)
* No implicit conversion (unlike C)
* No inheritance
* Compile-time generics with java-like bounds
* Complete avoidance of NULL/nullptr

* Only numeric types with explicit size and signedness (like uint64\_t)

* A struct type that can be initialised with a designated initializer

* A Checked Tagged Union Type
    * As a result: Option type with null pointer optimization

* Rust style algebraic error management

* C-Style for loop and iterable interface for collections

* Standard library with collections
* Strings not null-terminated, but stored length
* Standard Library smart pointers
    * What types? Heap, Refcount, Mutex, ...

* Optional Array bounds checking at Runtime

* Unicode Support

* Module system
* Central repo and library manager for such modules

# Data and Program Structure

Data can be structured in Product Types (structures). Functions on structs should be declared like members, and take a pointer to their instance.

<pre>
data SomeThing {
    SomeType a;
    SomeOtherType b;
}

methods SomeThing {
    doOperation: (uint16\_t arg) -> Option<uint64\_t>
}

main() -> MainResult {
    SomeThing thing = { .a = ..., .b = ... };
    result = thing.doOperation(3);
    # equivalent:
    result = SomeThing::doOperation(thing, 3);
}
</pre>

* Constructors and destructors for structures
* Explicit Interfaces (no duck-typing)

## Operator overloading
An overloadable operator is just defined by a trait.

# Syntactic Properties

I like the basic syntactic elements of C, but here are some complements
* keywords `true` and `false`
* `:=` for assignment, `=` for equality (Pascal-style)
* No unary pre/post increment/decrement (like Rust)
* If clauses/loops always require curly braces, making the parenthesis around the condition obsolete (like Rust)


# Comfortable Niceties

Tuple support
Pattern matching
Type inference for local variables
Native bitfields

# Open Questions

* Preprocessor/Macro System
* Types which constrain a numeric variable to a certain interval
    * Resolved at runtime or compile time

## How to avoid NULL
## How to handle Pointers

# Conclusion
You might notice that I essentially took some features o Rust and stuffed them into a C-like language. But I made a very strong omission: the borrow checker and lifetimes. This would make the language in question less safe, but might be worth it for simplicity's sake. In hindsight the question this post answers is: is the main source of language complexity in Rust the Ownership system? To which I think the answer is yes - everything else in Rust is rather simple. Now, the Ownership system is also one of the strongest properties of Rust.

Even without the borrow checker/ownership system, this still is a rather safe language, and way easier to use.
