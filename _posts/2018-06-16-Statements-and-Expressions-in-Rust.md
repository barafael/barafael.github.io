In this post, we will explore how Rust distinguishes between expressions and statements.
Even though this distinction seems very theoretical, it has wide-ranging implications on the Rust language and (in my opinion) is a beautiful detail of Rust.

## Terms and Conditions

What do I mean by expressions and statements?

Arithmetic terms like `(a + 3) / 2`, or a condition `(a & !b) | (!a & b)` are expressions. Evaluating them yields some (non-empty-)value like `3` or `true`. This is what functional programmers mean when they say their whole program is basically one large mathematical term which is evaluated. It is important to note that evaluating an expression in this mode of thinking has no effect on the outside world, or 'context'.

To interact with the world, we need statements. A statement could be ```motor.drive(100)```, where ```drive(int)``` is a void function which changes the context such that something in the real world reflects this effect (like writing to an output port).

## What's in a '{}'-Block?

Languages like C and Java use curly braces to denote scope. In Rust, curly braces mark a scope too, but they can also evaluate to a value. This value can be empty '()', though. Let's see some examples:

```c
// Standard C if statement
if (some_condition) {
    some_action();
}
```

```rust
// Slightly different syntax for Rust
if some_condition {
    some_action();
}
```

The syntax appears very similar, but this is a false lookalike: we can evaluate expressions in blocks.

```rust
let a = if some_condition {
    some_action()
} else {
    3
};
```

This is a very structured solution to the frequent problem of conditionally assigning/binding values to variables.

It should be noted that a semicolon suppresses the value of an expression. This is why we do not have semicola after `3` or `some_action()`, but after the last curly brace.

### Retiring the Ternary Operator

This C shorthand for an if-else-block as expression:
```c
int val = 5;
int result = val < 10 ? 10 : val;
```

does not require special syntax in Rust.

```rust
let val = 5;
let result = if val < 10 { 10 } else { val };
```

Note that in the Rust snippet, each branch of the if statement evaluates to a number.

## The Point of 'No Return'

Using the `return` keyword is discouraged sometimes in the Rust community. It makes no difference for the program, but omitting it fits better with the 'Everything is an Expression' mode of thinking. This becomes apparent when looking at the following examples, which may look scary for some C programmers.

```c
// C function that simply returns some value
int some_function(int x) {
    return x + 3;
}
```

The exit point of a C function often is a 'return', reminding us that in the background, the stack is restored and control is given to the caller.

```rust
// In Rust, the 'return' is unnecessary, since the function body is an expression.
fn some_function(x: u32) -> u32 {
    x + 3
}

// A void function in Rust
// Note: function actually returns '()', which is value of expression 'x + 3;'
fn some_void_func(x: u32) {
    x + 3; // < note the semicolon
}

// Note: the 'return' keyword can still be used for early returns
fn early_return(num: u8) -> u8 {
    if num == 3 {
        return 0
    }
    ...
}
```

## Unary incr/decrement Confusion

The standard way to increment a variable in C:

```c
int x = 10;
// Both expression (evaluates to 10) and statement (increments x).
x++;
// Same here, except expression evaluates to 12 now.
++x;
```

In Rust, this works differently:

```rust
let mut x = 10;
//x++ // not possible
x += 1;
```

This prevents such C chaos:

```c
int c = 0;
// What does this evaluate to?
int d = ++c + c++;
// and this?
int e = ++c + c++ + c;
```

## Switch-Case is no Match for Patterns

Pattern matching in Rust takes advantage of expression blocks quite beautifully:
```rust
let choice = 17;
let is_sparta = true;

let result = match choice {
    0 => "zero for u",
    1...9 => "choice not great",
    i if i % 2 == 1 => "larger than or equal 11, and odd",
    42 => "u r n4rd",
    300 => {
        if is_sparta {
            "This is Sparta!"
        } else {
            "This is not Sparta!"
        }
    }
    _ => "larger than or equal 10, and even",
}; // < semicolon needed, because this expression binds to 'result'
```

Every match arm must evaluate to the same type (&str here). The match arm can be any expression in a block that eventually evaluates to a &str.
The patterns must be exhaustive!
This is like switch/case on steroids.

## Let's if let

To destructure enums using `if let` is useful for ergonomic error handling with algebraic error types(Option, Result):

```rust
    let number: Option<i32> = Some(8);

    if let Some(8) = number {
        println!("Option contained an eight!");
    }

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Option contained None!");
    };
```

This works with any pattern, on enums, structs, and tuples.
