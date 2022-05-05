---
layout: post
---

Basics explained well in this blog post: [Rust Error Handling](https://www.sheshbabu.com/posts/rust-error-handling/).

The linked blog post shows how to manually create boxed errors and error enums.

# Boxed Errors

Use boxed errors if you have a very wide set of sources which could produce errors (wide as in, many crates or many modules). Why? Because boxing an error is simple, but makes it harder to handle for the layer above you.

# Error Enums

A custom error enum is the way to go if you want the user of the code you are working on to be able to check what exactly failed. You do have to create an enum variant for all the different error conditions. These could carry data, like source errors, backtraces, or even the input that caused the error (see e.g. [SendError](https://docs.rs/tokio/latest/tokio/sync/mpsc/error/struct.SendError.html)).

# Apps vs. Libs?

In many cases, applications use boxed errors and libs use error enums. But there are also counterexamples. I personally like error enums quite a bit, but this is a question of ergonomics, API design, and maybe taste.

# Gee, that's lots of typing for both approaches. It doesn't have to be! 

The [anyhow](https://docs.rs/anyhow/latest/anyhow/) is even nicer than the "Boxed-Error" pattern and not too expensive to use.

For error enums, the [thiserror](https://docs.rs/thiserror/latest/thiserror/) crate does all the manual work for you, and you get a nice way to declare your error codes that is completely invisible from outside of your crate or module.

# Example Application (with GUI!)

See [error-safari](https://github.com/barafael/error-safari) for some examples of both approaches.

# More Realistic Examples Please!

A realistic usage of anyhow: [achat](https://github.com/barafael/achat), a simple demo of different ways to implement network applications with tokio.

Some realistic usages of thiserror can be found here:

* [pils](https://github.com/barafael/pils/blob/main/src/error.rs), a simple lisp implementation inspired by [Build Your Own LISP](buildyourownlisp.com)
* (Silly code) [famous-last-words](https://github.com/barafael/famous-last-words)
* (Silly code) [Deutsche Bahn Delay Reasons](https://github.com/barafael/deutsche-bahn-delay-reasons)
