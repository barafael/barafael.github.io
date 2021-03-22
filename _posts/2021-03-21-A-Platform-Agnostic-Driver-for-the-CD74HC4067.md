---
layout: post
---

# CD74HC4067 16ch Multiplexer Embedded-HAL Driver Crate

To read 16 analog values, one might use 16 pins with ADCs.
But depending on the required update rates and the available pins, this might not even be possible.
Instead, a 74HC4067 could be used: 4 select pins determine which of the 16 I/Os is active.
This 16-channel analog or digital bidirectional multiplexer is fairly simple to control.
However, care must be taken when changing the active output, because during the logic high/low transitions of the 4 select pins, some unwanted patterns will be generated.
The [74HC4067 driver](https://github.com/barafael/cd74hc4067-rs) described in this post requires just a couple embedded-hal GPIO pins.
It ensures that the multiplexer is disabled during the changing of the channel.
It makes no assumptions about what to do with the I/O pin.

## Why do this?

It is a simple driver for very simple hardware, yes. But any repetition saved is code saved!

Additionally, this code serves as a good demonstration of testing using embedded-hal-mock and measuring test coverage using tarpaulin. And it serves as an example of type-state programming, it's advantages, and also ergonomics pitfalls.

## Example of usage

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

See here: [coverage.pdf](https://github.com/barafael/cd74hc4067/blob/main/coverage.pdf).

