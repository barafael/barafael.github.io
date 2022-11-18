---
layout: post
---

Radio communication is not easy, right? Well, unless it's complexities are hidden from us. That's why the nice people of [Ebyte](https://www.cdebyte.com/) have built the [E32 Modules](https://www.ebyte.com/en/product-view-news.html?id=108). These modules allow for easy serial-based LoRa communication. Under the hood, they use the SX1278 modules which give us lots of power. Wouldn't it be nice to have a Rust driver for those easy to use modules?

# Rust Driver

Pretty bog-standard implementation of a driver that uses `embedded-hal`. Provided with a couple GPIOs and a serial port, this driver will expose the functions `read` and `write` from `embedded-hal`.

For checking if you wired everything up correctly, you can get the model data as a nice rusty struct. For configuration, there are some nice data structures for all the different parameters and options the chip provides (different baud rates for UART and radio transmission, error correction, transmission power, of course channel and address, and some more). You can just inspect the settings of the chip, or store new ones temporarily or persistently.

The implementation is scrutinized with mocking, proptests, and mutation tests. Also, it includes my first load-bearing usage of declarative macros :)

[Ebyte E32 Rust Driver on GitHub](https://github.com/barafael/ebyte-e32-rs)

## Demo Project (bare metal, STM32F411 Black Pill)
[To be found here](https://github.com/barafael/ebyte-e32-demo)

## CLI and graphical user interfaces for testing

[Project here](https://github.com/barafael/ebyte-e32-ui)

The parameter types inside the driver optionally implement the `clap::ValueEnum` derive macro behind the feature gate `"value_enum"` (which also requires `std`). This means, the enums can be used as part of a CLI! And, using the super powers of [klask](https://github.com/MichalGniadek/klask), this CLI can be turned into a GUI. See [this project](https://github.com/barafael/ebyte-e32-ui) for how that goes! (I should probably blog about it).

Sneak peek:
![Image of GUI]({{ site.baseurl }}/images/ebyte-e32-gui.png)

## Blocking Operation

All operations are blocking, of course... Should be changed when `embedded-hal` becomes `async` or somehow supports non-blocking operation better than with `nb` (technically, you could do non-blocking implementation now - `nb::Result` gives you a `WouldBlock` which you could work with).

## Module Graph

![Image of Module Graph]({{ site.baseurl }}/images/ebyte-e32-rs-mods.png)

## Reduced Dependency Graph

![Image of Dependency Graph]({{ site.baseurl }}/images/ebyte-e32-rs-deps.png)

## Prior Work

* [Renzo Mischiantis Ebyte E32 Library](https://www.mischianti.org/2019/10/21/lora-e32-device-for-arduino-esp32-or-esp8266-library-part-2/)

* [Same library on GitHub](https://github.com/xreef/LoRa_E32_Series_Library)

Thanks for your inspiration!
