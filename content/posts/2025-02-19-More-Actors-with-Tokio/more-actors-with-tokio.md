---
title: Unit Testing
marp: true
theme: rhea
color: "dark-gray"
size: 16:9

---

<style>
    code {
        line-height: 1.4;
    }
</style>

<!--
footer: " "
 -->

<!--
paginate: true
 -->

<!--
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# More Actors

<div style="margin-top: 30px; margin-bottom: 30px">
    <img style="width: 200px; height: auto; display: block; margin: auto;" src="images/tokio-logo/tokio-horizontal.svg">
</div>

### with Tokio

![bg right:66%](images/Routing_Diagram_for_Materials_and_for_Printing_Forms_in_a_Manufacturing_Plant/Routing_Diagram_for_Materials_and_for_Printing_Forms_in_a_Manufacturing_Plant,_1914.jpg
)

---

<!-- header: ' ' -->

<iframe width="100%" height="90%" src="https://ryhl.io/blog/actors-with-tokio/">
</iframe>

[ryhl.io/blog/actors-with-tokio](https://ryhl.io/blog/actors-with-tokio) (2021)

---

### Street Definition

<style scoped>
div.twocols {
  margin-top: 35px;
  column-count: 2;
}
div.twocols p:first-child,
div.twocols h1:first-child,
div.twocols h2:first-child,
div.twocols ul:first-child,
div.twocols ul li:first-child,
div.twocols ul li p:first-child {
  margin-top: 0 !important;
}
div.twocols p.break {
  break-before: column;
  margin-top: 0;
}
</style>

<div class="twocols">

* Autonomous unit
  isolates state/process
* Communicates with others
  via message passing (channels)
* Channels outline system topology

<p class="break"></p>

![sepia](actor-dag.drawio.svg)

</div>

Inspired by [Alan Kay on Quora](https://www.quora.com/profile/Rafael-Bachmann-2/https-www-quora-com-What-does-Alan-Kay-mean-when-he-said-OOP-to-me-means-only-messaging-local-retention-and-protection) (the real one stated [similar ideas](https://wiki.c2.com/?AlanKaysDefinitionOfObjectOriented)).

---

### An Actor should _be_ its data

Adapted from [Alices original example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1e60fb476843fb130db9034e8ead210c):

````rust
#[derive(Debug, PartialEq, Eq)]
pub struct UniqueIdService {
    next_id: u32,
}
````

No  runtime resources (sockets, channel handles, etc.) here!
They belong to the actor _event loop_ future.

---

### The Event Loop

<style scoped>
{
  font-size: 34px
}
</style>

`async fn` which takes `self` and runtime resources.

````rust
impl UniqueIdService {
    pub async fn event_loop(mut self, mut rx: mpsc::Receiver<Message>) -> Self {
        loop {
            select! {
                ...
            }
        }
    }
````

[Loop-select is a real superpower](https://barafael.github.io/posts/stop-worrying-and-learn-to-loop-select/).

---

### Why that particular signature?

Applied ownership (_move_ semantics) to show who's boss:

**_Question:_** Why return `Self`?

* Tests: can assert on the guts
* Shutdown: can act on leftovers
* Restart: can inject in fresh instance
* Distributed actors: can move data elsewhere and restart

---

### Aside: Deterministic [Unit Test](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bb316eb8bf6ab51602bfaedb2a841e70)

````rust marker:unittest_uidservice
#[tokio::test]
async fn should_increment_unique_id() {
    let actor = UniqueIdService::new();
    let (tx, rx) = mpsc::channel(3);

    let resp1 = UniqueIdService::get_unique_id(&tx).await.unwrap();
    let resp2 = UniqueIdService::get_unique_id(&tx).await.unwrap();
    let resp3 = UniqueIdService::get_unique_id(&tx).await.unwrap();

    // Important for this test:
    drop(tx);

    let service = actor.event_loop(rx).await;

    let nums = tokio::try_join!(resp1, resp2, resp3).unwrap();
    assert_eq!(nums, (0, 1, 2));
    assert_eq!(service, UniqueIdService { next_id: 3 });
}
````

---

### Architecture

* Only actors and channels here!
* Birds-eye-view: complex but manageable
* State of the running system
* Fun and useful to follow messages along channels

[Protohackers Exercise 6](https://github.com/barafael/protohackers/tree/main/speedd)

![bg fit right](images/speedd.drawio.svg)

---

![bg 80%](images/system_architecture.drawio.svg)

---

### Graceful shutdown

An actor should shut down when its primary means of communication goes away:

- Socket closes
- Channel becomes empty and there are no more senders
- Timeout occurs

When an actor exits, it drops its handles toward other actors - signaling them to exit, too.

<small>Sounds like garbage collection?</small>

---

### The Messages define the Actor

<style scoped>
table {
    width: 100%;
}
table, tbody, tr, th, td {
    background-color: rgba(0, 0, 0, 0.0) !important;
    border-width: 0px;
}
</style>

<table>
<tr>
<td>

* Type of actual actor is _erased_
* Mocking is not required

```rust
type Handle = mpsc::Sender<Message>;
```

</td>
<td style="width: 450px;" >

[Message](https://github.com/barafael/protohackers/blob/02966f7e913b545d81fb521913fa09cb05e6f550/speedd_codecs/src/client/mod.rs#L8-L14) definition:
```rust
#[derive(...)]
pub enum Message {
    Plate(PlateRecord),
    WantHeartbeat(Duration),
    IAmCamera(Camera),
    IAmDispatcher(Vec<u16>),
}
```

</td>
</tr>
</table>

<small>Sounds like a VTable/Polymorphism/type erasure?</small>

---

### Is Rust OOP or not?

<style scoped>
{
  font-size: 32px
}
</style>


> OOP to me means only messaging, local retention and protection and hiding of state-process, and extreme late-binding of all things.

> You could be dumb enough to use these ideas to simulate older, more fragile, less scalable ideas — like “procedures and data” — but who would be so bound to the past to make that enormous blunder?
>
> — <cite>Alan Kay</cite>

**Maybe Rust + actors is OOOP (original object oriented programming).**

---

### Want more?

![bg right:40% 80%](images/more-actors-with-tokio-blogpost-qrcode.svg)

Blog post: [More Actors with Tokio](https://barafael.github.io/posts/more-actors-with-tokio/)

<iframe style="margin-top:5%" width="100%" height="70%" src="https://barafael.github.io/posts/more-actors-with-tokio">
</iframe>

---

### Rust Meetup Nuremberg

[Monthly online meetings](https://www.meetup.com/de-de/rust-noris/)

![bg 80%](images/rust-nuremberg-meetup-logo.png)
![bg 60%](images/rust-nuremberg-meetup-qrcode.svg)

---

### Questions?

<iframe style="margin-top:5%" width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%7D%0A">
</iframe>

![bg right:20% grayscale](images/gorch-fock-takelage/Takelage_(Gorch_Fock_II).jpeg)
