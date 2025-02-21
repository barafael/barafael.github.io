---
layout: post
toc: true
---

I can't seem to break from employing the `loop-select!` pattern.
What should merely be an intra-task concurrency primitive combining pattern matching and futures has sparked heated discussions.
While I begrudgingly acknowledge there are some footguns, here I'll advocate that actor-style programming _likes_ the `loop-select!` (and so should you).

> **_Note:_** In my previous post, I have looked at actor-style programming from the _outside_. I have described my view of the preferrable public API of an actor, the influence of channels on the system architecture, and effects of actors on other actors ("natural actor shutdown"). These relate to this article, wherein we finally peek inside the inner workings of what an actor can look like in tokio/rust.

## Hang on - what's `select!` again?

While the [docs on `tokio::select!`](https://docs.rs/tokio/latest/tokio/macro.select.html) have improved massively in the last few releases, here is another high-level description:

Select is all about concurrently running several processes within the same task.
The processes can have completely different objectives, but because of _concurrency_ (not _parallelism_), they may share access to the local variables among each other without any locks.

Here's a simplified sequence:

1. Start several processes, then
2. oversee their progress.
3. If one finishes, determine if the output matches our expectations, then
      * 3a: if it did not, go to 2;
      * 3b: else, stop.

These processes, as you might imagine, are futures (1). Not tasks! Futures.
The select macro will [(informedly) poll](https://doc.rust-lang.org/std/future/trait.Future.html) each future under its supervision (2).
When a future is signalled to be woken, the macro will poll it (2).

When a future returns [`Poll::Ready(T)`](https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready),
then the resulting `T` is matched on a user-supplied pattern (3).
If the pattern does not match (3a), the macro disregards that process but simply keeps monitoring the remaining ones.
Else, all other processes (futures) are canceled (3b) and the macro evaluates to the value of type `T`.

# Example: `loop-select!` enables Implicit Shutdown

Imagine an actor which collects messages from a channel and sinks them into a TCP socket.
The heart of the actor could look like this:

# Explicit shutdown requires `loop-select!`

Imagine you have an actor which collects messages on some channel and performs some work on them.
In the best case, you can employ "natural actor shutdown" by building the actor such that [it runs until no more message can ever be sent to it](https://barafael.github.io/More-Actors-with-Tokio/).
However, you may need to shutdown the actor in some other, more explicit way. You don't need to resort to calling [`JoinHandle::abort`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort)!

which may be cancelled by triggering a [`tokio_util::sync::CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html).