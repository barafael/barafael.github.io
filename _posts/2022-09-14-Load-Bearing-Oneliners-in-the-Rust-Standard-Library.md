The Rust language tries to be as minimal as possible.
Many crucial features and design decisions aren't compiler magic, instead they live in `core` or other parts of the standard library.
Here are some:

# Shared References are `Copy` (look don't touch)

The `core::marker::Copy` trait marks a type which can be safely copied by simply copying its bits.
Think `bool`, `f64`, `Duration`, or `SocketAddr`.

A shared reference can obviously be copied. Owning a shared reference to a `T`, i.e., a `&T`, allows us to copy it.

Here's the implementation of `core::marker::Copy` for shared references:

```rust
/// Shared references can be copied, but mutable references *cannot*!
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Copy for &T {}
```
(lives at: https://doc.rust-lang.org/src/core/marker.rs.html#839)

This means, any shared reference to objects which aren't necessarily even `Sized` can be copied.
That's why we even can call a method taking `&self` - copy the pointer to `self`, pass it to the method.

# Drop-Dead Gorgeous

```rust
/// --- Doc comment omitted ---
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "mem_drop")]
pub fn drop<T>(_x: T) {}
```

The function `drop` is passed a `T` without any trait bounds.
What can it do with a `T`?
It cannot `Clone` it, cannot do `Add` or `Neg`, cannot even `Display`/`Debug`.
And it isn't returning a value of `T`!
There isn't much besides `{}` that would be a valid function body:

* Doing `unsafe` things to look at `T` (perilous, `T` might not be `Sized`)
* Looping forever
* Producing a value of type `Never` or `core::marker::Infallible` (`panic` and friends, or `std::process::exit`)
* Taking a shared reference to `_x`, converting it to a usize, and storing it in a global `AtomicUsize`:

```rust
static R: AtomicUsize = AtomicUsize::new(0);

pub fn drop<T>(_x: T) {
    let r = &_x;
    R.store(r as *const T as usize, Ordering::SeqCst);
}
```
Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5e603039ddafeac0fdd599d78a59a13e
