When hearing about Rust the first time, terms like 'double free', 'data race', and 'dangling pointer' are often mentioned.
But without an understanding of these errors, this aspect of Rust is hard to appreciate.
Since Rust aims to appeal not only to systems programmers(where these kinds of errors are well known), I have collected a few simple examples that explain the error and show how Rust handles them.

## Let's start simple: Null pointer dereferencing

When Tony Hoare finished implementing ALGOL 60 in 1965, he didn't know that he would be apologizing for one of it's features in 2009: at QCon in London, he called null references a 'billion dollar mistake'. Languages since have picked up variations of the concept. For example, Java has no pointers accessible to the programmer, yet it has a `NullPointerException`.

Let's look at a simple C example of a null pointer dereference:

```c
#include <stdio.h>

int main() {
    // Reading and writing to a NULL pointer results in undefined behaviour.
    int *ptr = NULL;
    *ptr     = 10;
    printf("%d", *ptr);
}
```

We create a pointer to an int with the value of `NULL`. When we try to write to that location, a SIGSEGV happens.
How does this look in Rust? Simply put, safe Rust has no null references. There is no way to express this in safe Rust.
But there is always `unsafe`:

```rust
use std::ptr;

fn main() {
    let pointer: *mut u32 = ptr::null_mut();
    unsafe {
        pointer.write(10);
    }
    unsafe {
        println!("{}", pointer.read());
    }
}
```

Of course, this way we do get a SIGSEGV. Had to bend over backwards and do it explicitly!

## Free of 'use-after-free'

Something more interesting: unlike many other languages, we have to request and release our memory when working with C.
Those actions are hidden when using Java, Python, or Haskell. This is a difficult problem, because we have to ensure that we:

* free all allocated memory
* never use memory that we have free'd before
* ensure that memory is free'd exactly once in all code paths of our program

If we fail to do this, our program might crash, or worse, it might corrupt data or open up to attackers.
The `free` function in C deallocates a block of memory, but it does not invalidate our pointer - we can still use it:

```c
#include <stdio.h>
#include <stdlib.h>

#define BUF_SIZE 16

int main() {
    puts("Enter your name!");
    char *buffer = malloc(BUF_SIZE);
    
    fgets(buffer, BUF_SIZE, stdin);
    printf("buffer: %s\n", buffer);
    free(buffer);

    fgets(buffer, BUF_SIZE, stdin);
    printf("buffer after free: %s\n", buffer);
}
```

We allocate a 16-byte block, which we then pass to the `fgets` function to read user input. After printing that input, we free the buffer.
Nothing in the C language stops us from using that pointer again!

Let's see what this would look like in Rust:

```rust
use std::io::BufRead;

fn main() {
    println!("Enter your name!");
    let stdin = std::io::stdin();
    let buffer = stdin.lock().lines().next().unwrap().unwrap();

    std::mem::drop(buffer);

    println!("buffer: {}", buffer);
}
```

All the Rust compiler has to say is:

```
8  |     std::mem::drop(buffer);
   |                    ------ value moved here
9  |
10 |     println!("buffer: {}", buffer);
   |                            ^^^^^^ value used here after move
```

It is important to note that the `drop` function is not often used in Rust. That is because an object can be destructed when it goes out of scope of it's owner. But the function exists, and it is the most beautiful function ever:

```rust
pub fn drop<T>(_x: T) { }
```

That's all they wrote. The function takes ownership of _x of any type, and then it lets it go out of scope, resulting in deterministic deconstruction.

# More examples

A few more examples can be found [here](https://github.com/barafael/errare-humanum-est/tree/master/examples).
# A solution for C and C++: great linters

The `cppcheck` and `clang-tidy` have heuristics for many of the problems listed here. Often, their explanation of the problem is very good, as well.
