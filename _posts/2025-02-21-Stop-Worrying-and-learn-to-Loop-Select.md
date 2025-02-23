---
layout: post
toc: true
---

I can't seem to break from employing the `loop-select!` pattern.
But what should merely be an intra-task concurrency primitive combining pattern matching and futures has sparked heated discussions.
While I begrudgingly acknowledge there are some footguns, here I'll advocate that actor-style programming _likes_ the `loop-select!` (and so should you).

> **_Note:_** In my previous post, I have looked at actor-style programming from the _outside_. I have described my view of the preferrable public API of an actor, the influence of channels on the system architecture, and how to structure actors and full applications for "natural actor shutdown". These relate to this article, wherein I peek into the inner workings of what an actor can look like in tokio/rust.

## Hang on - what's `select!`, again?

The [`tokio::select!`](https://docs.rs/tokio/latest/tokio/macro.select.html) macro facilitates running several processes concurrently **within one task**.
These processes can have completely different objectives, but because they run concurrently,
they may share or even mutate the local variables among each other without any locks.

While the [docs on the `tokio::select!` macro](https://docs.rs/tokio/latest/tokio/macro.select.html) have improved massively in the last few releases,
here is another attempt at a simplified description:

1. Start several processes, and
2. oversee their progress.
3. Whenever one finishes, determine if its output matches our expectations for it, then
      * 3a: if it did not, ignore the process, and go back to 2;
      * 3b: else, stop.

These processes, as you might imagine, are futures (1). Not tasks! Futures.

The select macro will [(informedly) poll](https://doc.rust-lang.org/std/future/trait.Future.html) each future under its supervision (2).
When a future is signalled to be woken, the macro will poll it (2).
This is not special, it is simply how futures are run.

Finally, when a future returns [`Poll::Ready(T)`](https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready),
the resulting value is matched on a user-supplied pattern (3).
If the pattern does not match (3a), the macro disregards that process but continues monitoring the remaining ones.
Else, all other processes (futures) are canceled (3b) and the macro evaluates to the value yielded by the future.

### In terms of Syntax

```rust

```

# Example 1: Explicit shutdown requires `loop-select!`

Imagine an actor which periodically sends a message into the world.
This actor shall stop running only when you tell it to, via a cancellation token.
No need to resort to aborting a task using [`JoinHandle::abort`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort)!

The core of the `Beacon` actor could look like this:

```rust
pub async fn event_loop<S>(mut self, token: CancellationToken, mut sink: S) -> Self
where
    S: Sink<u8> + Unpin,
    S::Error: std::fmt::Debug,
{
    let mut interval = interval(Duration::from_secs(1));
    loop {
        // First: get the next message.
        let message = select! {
            _cancellation = token.cancelled() => {
                break;
            }
            _ = interval.tick() => {
                self.0
            }
        };
        // Second: handle it.
        if let Err(error) = sink.send(message).await {
            tracing::warn!(?error, "Sink closed");
            break;
        }
        self.0 += 1;
    }
    self
}
```

Because the cancellation token and the interval are monitored concurrently, triggering the cancellation token will simply break the loop.
If it gets cancelled while the "business logic" is running, no problem - it will simply yield immediately on the next loop iteration.

> **_Note_**: This test pauses time. That's an incredibly neat feature of tests in tokio - they run in stop-time (faster than light, sometimes)!. It works by skipping over any operations which are explicitly "sleeping" (such as [`Interval::tick`](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html#method.tick)) **if they are the only remaining operations right now**.

> **_Note_**: The trait bounds allow us to abstract over the kind of I/O resource we are actually using. It could be a TCP socket, websocket, webtransport socket, (wrapped) channel sender, or any other [implementor of the `futures::Sink` trait](https://docs.rs/futures/latest/futures/sink/trait.Sink.html#implementors). One frequent pattern is to wrap an I/O resource implementing the [`AsyncWrite`] trait by some type which then implements the `Sink` trait. The [`codec`](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html) module of [`tokio-util`](https://docs.rs/tokio-util/) simplifies this pattern.

[Full example](https://github.com/barafael/barafael.github.io/tree/master/_projects/explicit-actor-shutdown)

# Example 2: `loop-select!` enables Implicit Shutdown

I don't think explicit shutdown is an anti-pattern, however usually I prefer implicit shutdown.
Imagine an actor similar to the previous one, which collects messages from a channel and sinks them into a TCP socket.
Additionally, it shall sink a specific (unchanging) message into the socket once per second.

The core of the `Forwarder` actor could look like this:

```rust
pub async fn event_loop<S>(self, mut rx: mpsc::Receiver<u8>, mut sink: S) -> Self
where
    S: Sink<u8> + Unpin,
    S::Error: std::fmt::Debug,
{
    let mut interval = interval(Duration::from_secs(1));
    loop {
        // First: get the next message.
        let message = select! {
            message = rx.recv() => {
                let Some(message) = message else {
                    break;
                };
                message
            }
            _ = interval.tick() => {
                PING
            }
        };
        // Second: handle it.
        if let Err(error) = sink.send(message).await {
            tracing::warn!(?error, "Sink closed");
            break;
        }
    }
    self
}
```

I have [previously argued](https://barafael.github.io/More-Actors-with-Tokio/) that "natural actor shutdown" is "best actor shutdown".
Here, we exit whenever the channel is closed naturally or in the unhappy case where our socket (sink) was closed remotely,
in which case we also have no more work to do.
The other actors sending us messages may react to this, as they will observe the forwarders absence eventually (certainly when they try to send it something).

[Full example](https://github.com/barafael/barafael.github.io/tree/master/_projects/implicit-actor-shutdown)

# Footguns hiding in the dark

You may have noticed I have papered over the details.

## Issue #1: Cancellation Safety

I stated that the futures in the competing concurrent branches of `select!` are simply dropped when another arm "wins".
If a future already internally commits some transaction (memory, network, database, whatever) before becoming `Poll::Ready`, and is then dropped,
the transaction will of course not be reverted.
Or worse, if part of the transaction has happened, and the future is then dropped, the system may remain in an inconsistent state.
Most futures in tokio are marked cancellation safe in the docs. In the ecosystem, that annotation is less pervasive.
I haven't personally had problems with Cancellation Safety Violations (or am unaware), but it is something to keep in mind.

## Issue #2: A hidden panic

In the examples above, we used `select!` to produce a value of type `T`.
And in the sequence, I have outlined how an arm will get disabled when it resolves but the pattern does not match.
So what happens if all arms become disabled? A `panic!`.

> **_Note_**: That's because a `panic!` is one way to create a value of type `T`, by creating a value of type `!` (the never type). The never type is the only subtype in Rust, and it's a subtype of every other type. There is even an `impl From<!> for T` somewhere, among other fun shenanigans.

Here's a simple example:

```rust
    #[tokio::test]
    #[should_panic(expected = "all branches are disabled and there is no else branch")]
    async fn select_nothing() {
        let _nonono = tokio::select! {
            Some(n) = async { None } => { n },
        };
    }
```

To fix it in this case, you can just add a `break` as a `select!` arm:

```rust
let zero = tokio::select! {
    Some(n) = async { None } => { n },
    else => 0,
};
```

I like to `match` or `let-else` on the `mpsc`-arm of the `select` (there usually is one) and break there.
In other cases, one may want to break from the `loop-select` when [`read`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read)ing an [`AsyncRead`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html) returns `0` (no more bytes to be had).

[Full example](https://github.com/barafael/barafael.github.io/tree/master/_projects/select-examples)

## Issue #3: Easy to create an infinite busy loop

Imagine having an actor to collect messages from three mpsc channels (for sake of example, it's not terribly important).
One could (I did) write such an event loop:

```rust
pub async fn event_loop(
    mut self,
    mut rx1: mpsc::Receiver<u8>,
    mut rx2: mpsc::Receiver<u8>,
    mut rx3: mpsc::Receiver<u8>,
) -> Self {
    loop {
        tracing::info!("waiting for message");
        // First: get the next message.
        let message = select! {
            message = rx1.recv() => {
                message
            }
            message = rx2.recv() => {
                message
            }
            message = rx3.recv() => {
                message
            }
        };
        // Second: handle it.
        tracing::info!(value = ?message, "got message");
        self.0 += 1;
    }
}
```

Once all channel handles have been dropped, every pattern always matches (even though it will always be `None`).
Then, we get our useless tracing output, and finally, we start from top.
I decided to include this example because busy loops like this are especially bad in an `async` world.
The only solace offered here is that the loop is `async-busy`, that is, at least it does not permanently hog the CPU.
It is busy in the sense that it can always run, but at least `tokio` won't let it suffocate the other tasks.

[Full example](https://github.com/barafael/barafael.github.io/tree/master/_projects/infinite-busy-loop-via-select)

## Issue #4: Tooling

`Rustfmt` simply does not touch anything within a `select`, and Rust Analyzer also struggles with it.
That's not surprising, as the syntax is obviously invalid Rust to it.

I like to take it as an incentive to create functions for all event handlers.
Here's my recommended approach: in your `select`, only bother with message/event collection and shutdown.
The `select`, at the start of the event loop, shall only produce a value of a type `Event` or similar.
Then, after collecting the event, a method `on_event` can be called.

```rust
pub async fn event_loop(mut self, mut rx: mpsc::Receiver<Message>) -> Self {
    loop {
        // Collect some event.
        let event = select! {
            message = rx.recv() => {
                let Some(message) = message else {
                    break;
                };
                Event::Message(message)
            }
            Some((id, status)) = self.tasks.next() => {
                Event::TaskStatus(id, status)
            }
        };

        // Process the event.
        if let Err(error) = self.on_event(event) {
            warn!(?error, "Error processing event");
        }
    }

    // Await completion of remaining tasks.
    while let Some((id, status)) = self.tasks.next().await {
        if let Err(error) = self.on_task_status(id, status) {
            warn!(?error, "Error processing task status");
        }
    }

    self
}
```

[Full example](https://github.com/barafael/barafael.github.io/tree/master/_projects/nice-loop-select)
