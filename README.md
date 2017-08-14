#WiringPi Bindings for Rust

An API wrapper for [WiringPi](http://wiringpi.com/) to make it accessible
using Rust. It implements the most important functions and provides a bit of
type system convenience.

[Online documentation](http://ogeon.github.io/docs/rust-wiringpi/master/wiringpi/index.html)

Add the following lines to your `Cargo.io` to use `rust-wiringpi`:

```
[dependencies]
wiringpi = "0.2"
```

##Example: Flashing Light

```Rust
extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup();

    //Use WiringPi pin 0 as output
    let pin = pi.output_pin(0);

    let interval = time::Duration::from_millis(1000);

    loop {
        //Set pin 0 to high and wait one second
        pin.digital_write(High);
        thread::sleep(interval);

        //Set pin 0 to low and wait one second
        pin.digital_write(Low);
        thread::sleep(interval);
    }
}
```

##Cross Compiling Using Cargo

This project can be cross compiled using Cargo.
[Follow these instructions](https://github.com/Ogeon/rust-on-raspberry-pi)
And use `./cross64 build` or `./cross32 build`, depending on your system,
to check if everything builds as expected.

##Orange Pi support

`rust-wiringpi` can also wrap the WiringOP library for the Orange Pi SBC boards.
This can be enabled with the `orangepi` feature:

```
[dependencies.wiringpi]
verson = "0.2"
features = ["orangepi"]
```

##Development Mode

In development mode, `rust-wiringpi` is compiled as a rust-native library excluding the original WiringPi.
And binding functions are replaced by dummy functions that output simple logs to stdout.
With this mode, you can build and debug your project on platforms that does not support WiringPi.

```
# build
$ cargo build --features wiringpi/development

# run
$ cargo run --features wiringpi/development

[wiringpi] `wiringPiSetup` called
[wiringpi] `pinMode` called with: 0, 1
[wiringpi] `digitalWrite` called with: 0, 1
[wiringpi] `digitalWrite` called with: 0, 0
...
```
