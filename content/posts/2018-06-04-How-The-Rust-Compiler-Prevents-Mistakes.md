+++
title = "How the Rust Compiler prevents mistakes"
date = 2018-06-04
+++

When learning about Rust for the first time, one is confronted with words like 'double free', 'data race', and 'dangling pointer'.
Without an understanding of these problems, the safety aspects of Rust are perhaps difficult to appreciate.
However, Rust aims to appeal not only to systems programmers(where these kinds of problems are well-known), but developers from any background! It is perfectly fine to write high-level Rust code without knowing what a data race is (that is the freedom Rust grants) but understanding the underlying issue makes the compiler error messages more understandable.

To explain some of the problems Rust solves, I have collected examples to show and explain specific problems and see what `rustc`("the Rust complainer") has to say about them.

## Let's start simple: Dereferencing Null Pointers

When Tony Hoare finished implementing ALGOL 60 in 1965, he couldn't know that he would be apologizing for one of its concepts 44 years later: at '09 QCon in London, he called null pointers a 'billion dollar mistake.'

Languages since have picked up variations of the concept. For example, Java has no pointers accessible to the programmer, yet it has a `NullPointerException`.

Let's look at a simple C example of a null pointer dereference:

```c
#include <stdio.h>

int main() {
    // Reading and writing to a NULL pointer results in undefined behavior.
    int *ptr = NULL;
    *ptr     = 10;
    printf("%d", *ptr);
}
```

We create a pointer to an int with the value of `NULL`. When we try to write to that location, a SIGSEGV happens to happen on my machine and setup, but dereferencing a NULL pointer is undefined behavior, meaning that anything could happen.

How does this look in Rust? Simply put, safe Rust has no null references. There is no way to express this in safe Rust.

There is always `unsafe`:

```rust
use std::ptr;

fn main() {
    let pointer: *mut u32 = ptr::null_mut();
    unsafe {
        *pointer = 10;
    }
    unsafe {
        println!("{}", *pointer);
    }
}
```

With this code, I happen to get a SIGSEGV, too.

## And now for something more interesting: Free of 'use-after-free'

Unlike many other languages, we have to explicitly request and release our memory from/to the operating system when working with C.
Those actions still happen when using Java, Python, or Haskell, but they happen automatically in the background. This is safe (because it is automatic), and it is very comfortable for the programmer. However, such garbage collection has a cost, which is out of scope for this article but frequently discussed.

Managing memory manually (like it is done in most systems programming languages) is a difficult problem because we have to ensure that:

* all allocated memory is `free()`'d (no memory leaks)
* memory that was `free()`'d before is never reused (no use-after-free)
* memory is `free()`'d exactly once in all code paths of the program (no double free)

If we fail to do this, our program might crash, or worse, it might corrupt data anywhere or present opportunities to attackers.

The `free` function in C deallocates previously allocated blocks of memory, but it does not invalidate our pointer to that memory - we can still use it:

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

    // reusing dangling pointer into heap
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

    drop(buffer);

    println!("buffer: {}", buffer);
}
```

All the Rust compiler has to say is:

```
error[E0382]: use of moved value: `buffer`
  --> use_after_free.rs:10:28
   |
8  |     drop(buffer);
   |                    ------ value moved here
9  |
10 |     println!("buffer: {}", buffer);
   |                            ^^^^^^ value used here after move
   |
   = note: move occurs because `buffer` has type `std::string::String`, which does not implement the `Copy` trait
```

The error message uses some Rust-specific language (move? Copy trait?), but it is pretty clear: ownership of the buffer is moved into `drop()`, after which the buffer is not usable.

It is important to note that the `drop` function is used rarely in Rust. That is because every object is destructed automatically at the point where its owner's scope ends (which is known at compile time).

Regardless, the function exists, and it is one of the most beautiful functions in the standard library:

```rust
pub fn drop<T>(_x: T) { }
```

That's all they wrote: take ownership of an `_x` of an unconstrained type T, and go out of scope, resulting in the deterministic deconstruction of `_x`.

## But what about `std::move`?

Modern C++ introduced moving ownership. The move constructor invalidates the old owner in some agreed-upon way, even if the object is const. After the move, the old pointer may still be used (but that is not a good idea!).

```cpp
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
    string origin = "This is a string.";

    vector<std::string> vec;

    // Copy origin and append to vec
    vec.push_back(origin);

    cout << "After pushing origin copy onto vec, origin is unchanged: \"" << origin << "\"\n";

    // Move origin into vec, invalidating origin variable (at runtime)
    vec.push_back(std::move(origin));

    cout << "After move into vec, origin is invalidated: \"" << origin << "\"\n";

    cout << "Contents of vec: \"" << vec[0] << "\", \"" << vec[1] << "\"\n";
}
```
Note that a moved-from object (that is not trivially destructible) still has to be destroyed: http://www.drdobbs.com/cpp/why-moving-an-object-does-not-destroy-th/231601451

Rust has moves and ownership baked deep into the language:

```rust
fn main() {
    let mut origin: String = "This is a string".into();

    let moved = origin;

    println!("origin after move: {}", origin);
    println!("moved  after move: {}", moved);

    // Binding that was moved out from can be reassigned
    origin = "Hello, I'm back!".into();
    println!("origin after reassignment: {}", origin);
}
```

The Rust compiler comments:

```
error[E0382]: use of moved value: `origin`
 --> use_after_move_var.rs:6:39
  |
4 |     let moved = origin;
  |         ----- value moved here
5 |
6 |     println!("origin after move: {}", origin);
  |                                       ^^^^^^ value used here after move
  |
  = note: move occurs because `origin` has type `std::string::String`, which does not implement the `Copy` trait
```

Types implementing the `Copy`-trait are similar to 'primitive datatypes' in java. They are passed by value (or 'by copy').
But copying is a quick operation only for a few basic types! For any other type, the explicit `clone` method can be used, if the type implements the `Clone` trait. This is unlike C++, where an implicit copy of an object often occurs, without any special syntax.

# The Many Kinds Of Dangling Pointers

### Dangling Pointer Into Heap
In C-like languages, we can use the addresses of objects on the heap or stack directly. This is powerful, but it means we have to be cautious about the memory at the other end of a pointer:

```c
#include "stdlib.h"
#include "stdio.h"

#define BUFFER_SIZE 16

int main() {
    int *array = malloc(BUFFER_SIZE * sizeof(int));
    for (size_t index = 0; index < BUFFER_SIZE; index++) {
        array[index] = index;
    }
    // Take a pointer into the heap-allocated array
    int *ptr = &array[6];

    // at this point, ptr becomes a dangling pointer
    free(array);

    printf("%d's array has been set free!\n", *ptr);
    *ptr = 3;
    printf("array[6] was modified to %d after being free'd!\n", *ptr);
}
```

This program allocates a buffer on the heap, creates a pointer into that buffer, then frees the buffer. However, the pointer still exists! That is a dangling pointer into the heap. Let's reconstruct this in Rust:

```rust
fn main() {
    let mut array = Vec::new();
    for index in 0..10 {
        array.push(index as u32);
    }

    // Take a pointer into the heap-allocated array
    let reference = &mut array[6];

    // at this point, reference would become a dangling pointer
    drop(array);

    println!("{}'s array has been set free!", *reference);
    *reference = 3;
    println!("array[6] was modified to {} after being free'd!", *reference);
}
```

The compiler interjects! Error message:

```
error[E0505]: cannot move out of `array` because it is borrowed
  --> dangling_pointer_heap.rs:11:20
   |
8  |     let reference = &mut array[6];
   |                          ----- borrow of `array` occurs here
...
11 |     drop(array);
   |                    ^^^^^ move out of `array` occurs here
```

The message is clear. How dare we move the ownership of the variable array to `drop` if we still have borrowed it to `reference`?

# Capturing Closures

Closures are anonymous functions which can capture variables from their originating scope.
Therefore, if we create a string variable `some_string` and a closure `some_closure` in one scope, we can use `some_string` from `some_closure`.
Closures would be pointless if we could not pass them around different scopes! That is dangerous, though:

```cpp
#include <iostream>
#include <functional>

using namespace std;

function<int(int)> get_lambda_with_local_reference(int index) {
    int local_arr[] = { 1, 2, 3, 4, 5 };
    return [&](int value) { return value + local_arr[index]; };
}

int main() {
    // This function returns a lambda which internally keeps a pointer to a local array.
    // Of course, when using the returned lambda, the array does not exist anymore.
    function<int(int)> function = get_lambda_with_local_reference(2);
    cout << "lambda uses stack-local reference:" << function(6) << endl;
}
```

The function `get_lambda_with_local_reference` will return a `function<int(int)>`, which is a lambda that takes and returns an int.
That function is defined in the last line of `get_lambda_with_local_reference` as returning the sum of its argument and the element at index `index` of an array defined in the same (stack frame) scope.
When we return the lambda, this array goes out of scope. When we call it, the lambda dereferences some random value from the stack (at best).

Same story in Rust:

```rust
// This function returns a closure with a pointer to a stack-local array.
fn get_lambda_with_local_reference(index: usize) -> impl Fn(i32) -> i32 {
    let local_arr = [1, 2, 3];
    |value| value + local_arr[index]
}

// Moving the stack-local array into the returned closure is fine.
fn get_lambda_with_moved_reference(index: usize) -> impl Fn(i32) -> i32 {
    let local_arr = [1, 2, 3];
    move |value| value + local_arr[index]
}

fn main() {
    let function = get_lambda_with_local_reference(2);
    println!("lambda uses stack-local reference: {}", function(6));
}
```

The Rust Complainer says NO:

```
error[E0373]: closure may outlive the current function, but it borrows `local_arr`, which is owned by the current function
 --> dangling_pointer_closure.rs:5:5
  |
5 |     |value| value + local_arr[index]
  |     ^^^^^^^         --------- `local_arr` is borrowed here
  |     |
  |     may outlive borrowed value `local_arr`
help: to force the closure to take ownership of `local_arr` (and any other referenced variables), use the `move` keyword
  |
5 |     move |value| value + local_arr[index]
  |     ^^^^^^^^^^^^
```

`rustc` also suggests how to fix the problem, as seen in the function `get_lambda_with_moved_reference`.

### More dangling pointers

You can find more examples of less interesting dangling pointers [here](https://github.com/barafael/errare-humanum-est/tree/master/examples).

# Crossing Boundaries or: yet another 'Billion Dollar Mistake'

For performance reasons, the creators of the C language used raw pointers to memory blocks as array types. After creating an array, it's size has to be tracked manually, often by something like `#define BUF_SIZE 256`. Array access by index, like `arr[115]`, happens without checking bounds. One could check manually.
Similarly, strings (which are just char pointers) always end with a `\0`-byte. The performance benefits come with a price - it is incredibly easy to make a mistake:

```c
#define BUFFER_SIZE 15

/* Compile with -fno-stack-protector for real fun */
int main() {
    int buffer[BUFFER_SIZE];
    for (int index = 0; index <= BUFFER_SIZE; index++) {
        buffer[index] = index;
        // or equivalently, but more explicit:
        *(buffer + index) = index;
    }
}
```

Just one character too much - the '=' in the for loop exit condition causes our index to reach one element past the buffer boundaries.
This is easy to catch. But there are other possible buffer overruns which are even in the C library:

```c
#include <stdio.h>
#include <string.h>

#define BUFFER_SIZE 16

/* Compile with -fno-stack-protector for full effect */
int main() {
    // gets is a hazardous function, and gcc even warns when using it.
    // Here, gets overwrites a part of the stack when a long text is entered on stdin,
    // possibly corrupting a variable that comes after the input buffer on the stack.
    char buffer[BUFFER_SIZE];
    int password = 0;

    printf("Enter password:\n");
    gets(buffer);

    if (strcmp(buffer, "pass123") == 0) {
        printf("Correct password\n");
        password = 1;
    } else {
        printf("Wrong password\n");
    }
    if (password) {
        printf("Privileged access granted!!!\n");
    }
}
```
Writing a string into the buffer that is larger than the buffer can corrupt the password flag and grant us privileged access!
And yes, you should not use `gets`, as the compiler may tell us here. So, let's use fgets, but hide a mistake in our code:

```c
#include <stdio.h>
#include <string.h>

#define BUFFER_SIZE 16

/* Compile with -fno-stack-protector */
int main() {
    // fgets is somewhat better than gets. But one can still use it wrong.
    char buffer[BUFFER_SIZE - 5];
    int password = 0;

    printf("Enter password:\n");
    fgets(buffer, BUFFER_SIZE, stdin);

    if (strcmp(buffer, "pass123") == 0) {
        printf("Correct password\n");
        password = 1;
    } else {
        printf("Wrong password\n");
    }
    if (password) {
        printf("Privileged access granted!!!\n");
    }
}
```

The fundamental problem is that the array size is unknown. There may be a performance advantage to not having those runtime index bounds checks, but modern LLVM is good at optimizing those away. Either way, bounds checks should be an opt-out feature for critical loops, not an opt-in by manually coding them in my opinion.

Obligatory Rust example:

```rust
use std::io::{self, BufRead};

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for index in 0..10 {
        println!("{}", array[index]);
    }
}
```

This code panics with 'index out of bounds' at runtime. Rust cannot catch that kind of bug at compile time (it is hard to catch in the general case)!

# The Real Fun Stuff: Access to Shared Data

Data races can happen if these 3 conditions are met: (1) multiple parts of a program have access to the same memory (sharing), (2) at least one of them writes to the shared data (mutation), and (3) there is no mechanism in place to ensure proper order of transactions (synchronization).

To wrap everything in mutexes and semaphores is one viable option, but Rust offers a safer and faster option: ensuring the first 2 conditions are never true at the same time. This is what "Sharing XOR mutation" means: either many processes read, or at most one writes. It turns out the borrow checker that ensures memory errors never happen also prevents many issues arising from shared access to resources because there is always a clear owner and it is known if and how data is shared.

Here is what a multithreaded C++ program could look like:

```cpp
#include <iostream>
#include <thread>

struct Account {
    int balance{ 100 };
};

void transferMoney(int amount, Account &from, Account &to) {
    using namespace std::chrono_literals;
    if (from.balance >= amount) {
        from.balance -= amount;
        std::this_thread::sleep_for(1ns);
        to.balance += amount;
    }
}

void printSum(Account &a1, Account &a2) {
    std::cout << (a1.balance + a2.balance) << std::endl;
}

int main() {
    Account     account1;
    Account     account2;
    std::thread thr1(transferMoney, 50, std::ref(account1), std::ref(account2));
    std::thread thr2(transferMoney, 130, std::ref(account2), std::ref(account1));

    thr1.join();
    thr2.join();

    std::cout << "account1.balance: " << account1.balance << std::endl;
    std::cout << "account2.balance: " << account2.balance << std::endl;

    std::cout << std::endl;
}
```

We give the data race some time to occur. What happens is not deterministic, but the second transaction is sometimes just swallowed by the void.
We have met all 3 of the above conditions!

To fix the problem, we might use an `atomic` type for `balance`, however, not even cppcheck or clang-tidy warn us here.

Translating the same to Rust:

```rust
#[derive(Debug)]
struct Account {
    balance: u32,
}

impl Account {
    fn transfer_money_to(&mut self, amount: u32, mut to: Account) {
        if self.balance >= amount {
            self.balance -= amount;
            std::thread::sleep(std::time::Duration::from_millis(1));
            to.balance += amount;
        }
    }

    fn new(initial: u32) -> Self {
        Account {
            balance: initial,
        }
    }
}

fn main() {
    let mut account1 = Account::new(100);
    let mut account2 = Account::new(100);

    let child1 = std::thread::spawn(|| {
        account1.transfer_money_to(50, account2)
    });

    let child2 = std::thread::spawn(|| {
        account2.transfer_money_to(130, account1)
    });

    child1.join();
    child2.join();

    println!("{:?}\n{:?}", account1, account2);
}
```

The error message here is quite long because there are so many mistakes from the Rust compiler's point of view.

# More examples

A few more examples can be found [here](https://github.com/barafael/errare-humanum-est/tree/master/examples).

# A solution for C and C++: great linters

The `cppcheck` and `clang-tidy` have heuristics for many of the problems listed here. Often, their explanation of the problem is excellent, as well.
