+++
title = "A Platform-Agnostic driver for the HC12 serial radio module"
date = 2021-03-21
+++

If only a serial port could be used without having to deal with cables!
The HC12 serial radio module enables just this super power.
It connects to a normal serial port, sending and receiving data to/from another hc12 module.

The baud rate, transmission power, channel, and some other properties can be configured over the same serial interface, using simple AT commands.
To disambiguate the normal mode from the configuration mode, a digital signal is used.

In this blog post, I introduce a simple embedded-hal driver for the hc12 module.
It requires a serial interface, a delay implementation, and a digital pin.

Type-state is used to disambiguate the modes of operation: normal mode, configuration mode, and sleeping mode.
In normal mode, the driver instance can simply be used like any other serial device (with the Read and Write methods of the embedded-hal Serial trait).

## Usage example

Full code [here](https://github.com/barafael/hc12-at-rs/tree/main/hc12-example-raspi).
This example makes a driver instance, queries a few parameters, then sends a basic string over the hc12 repeatedly.

```rust
let uart = Uart::new(9600, Parity::None, 8, 1).unwrap();

let set_pin = Gpio::new().unwrap().get(18).unwrap().into_output();

let hc12 = hc12_at::hc12::Hc12::new(uart, set_pin, linux_embedded_hal::Delay);

let mut hc12 = match hc12.into_configuration_mode().unwrap();

assert!(hc12.is_ok());

let mut buffer = [0u8; 64];
let result = hc12.get_version(&mut buffer);
println!("{:?}", std::str::from_utf8(&result).unwrap());

let params = hc12.get_parameters().unwrap();

println!("{:#?}", params);

println!("{:?}", params.get_air_baud_rate());

println!(
    "{:?}",
    params.get_air_baud_rate().get_wireless_sensitivity_dbm()
);

let mut hc12 = match hc12.into_normal_mode().unwrap();

loop {
    hc12.write_buffer(b"hello hc12\r\n").unwrap();
    thread::sleep(Duration::from_millis(500));
}
```

## Configuration

The baudrate, radio channel, radio mode, and transmission power can be configured using AT commands.
To get the current configuration, the API function get_params can be used.

Only some combinations of settings are valid though - the API should protect from misconfiguration.

To get the air baud rate and the wireless sensitivity as specified in the [datasheet](https://www.elecrow.com/download/HC-12.pdf), API functions get_air_baud_rate and get_wireless_sensitivity_dbm can be used.

