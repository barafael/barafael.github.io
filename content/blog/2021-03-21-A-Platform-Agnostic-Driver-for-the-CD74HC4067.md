+++
title = "A Platform-Agnostic Driver for the CD74HC4067"
date = 2021-03-21
+++

To read 16 analog values, one might use 16 pins with ADCs.
But depending on the required update rates and the available pins, this might be impossible.

Instead, a simple 74HC4067 could be used.
It connects 1 of 16 analog IO lines to 1 signal line, depending on the 4 select pins.
The only thing to look out for is to disable the chip with the disable signal before changing the select pins to avoid activating an unwanted line during switching.

The [74HC4067 driver](https://github.com/barafael/cd74hc4067-rs) described in this post requires just a couple embedded-hal GPIO pins.
It ensures that the multiplexer is disabled during the changing of the channel and makes no assumptions about what happens with the I/O pin.

## Why make such a simple driver?

It is a simple driver for very simple hardware, yes. But any repetition saved is code and bugs saved!

Additionally, this code serves as a good demonstration of testing using embedded-hal-mock and measuring test coverage using tarpaulin. And it serves as an example of type-state programming, it's advantages, and also ergonomics pitfalls.

## Example usage

This code will activate random inputs. Working code for this on `stm32f0` target can be found [here](https://github.com/barafael/cd74hc4067-rs/tree/main/stm32f0-example).

```rust
let mut delay = Delay::new(/* snip */);

let gpioa = dp.GPIOA.split(/* snip */);

let (pin_0, pin_1, pin_2, pin_3, pin_enable) = disable_interrupts(|cs| {
    (
        gpioa.pa0.into_push_pull_output(cs).downgrade(),
        gpioa.pa1.into_push_pull_output(cs).downgrade(),
        gpioa.pa4.into_push_pull_output(cs).downgrade(),
        gpioa.pa8.into_push_pull_output(cs).downgrade(),
        gpioa.pa7.into_push_pull_output(cs).downgrade(),
    )
});

let mut on_for = |duration: u32,
                  pin: u8,
                  mut hc: Cd74hc4067<
    Pin<Output<PushPull>>,
    Pin<Output<PushPull>>,
    DisabledState,
>| {
    hc.set_channel_active(pin as u8).debugless_unwrap();
    let enabled = hc.enable().debugless_unwrap();

    delay.delay_ms(duration);

    enabled.disable().debugless_unwrap()
};

let mut disabled =
    cd74hc4067::Cd74hc4067::new(pin_0, pin_1, pin_2, pin_3, pin_enable).debugless_unwrap();

let mut rng = RNG::<WyRand, u8>::new(0xDEADBEEF);

let delay_time_ms: u32 = 2000;
loop {
    let generated: u8 = rng.generate_range(0, 15);
    disabled = on_for(delay_time_ms, generated, disabled);
}
```

## Advantages of Typestate Programming

* The chip starts out in disabled mode
* The select pins can only be changed while the chip is disabled
* Once the chip is enabled, it must be disabled before anything can be done to it

## Cumbersome usage due to Typestate Programming

The driver implementation relies on typestate programming for implementing the state transitions.
There are only 2 states, and the driver toggles between them as required.
In the `EnabledState`, the multiplexer is active, and the active output channel cannot be changed.
In the `DisabledState`, the multiplexer is not active, and the output channel can be selected.

This makes the usage a little bit cumbersome, but it ensures that the driver can only be used safely while remaining efficient.

## Line Coverage and PhantomData

While writing unit tests while keeping an eye on the line coverage, I noticed that some lines would not be covered - and for a really good reason!
PhantomData markers are used in the code to disambiguate disabled and enabled states.
These markers are compile-time-only constructs - hence, line coverage cannot ever include them.
Neat.

See here: [coverage.pdf](https://github.com/barafael/cd74hc4067-rs/blob/main/cd74hc4067/coverage.pdf).

