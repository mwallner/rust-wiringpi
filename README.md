#WiringPi Bindings for Rust

An API wrapper for [WiringPi](http://wiringpi.com/) to make it accessible
using Rust. It implements the most important functions and provides a bit of
type system convenience.

[Online documentation](http://ogeon.github.io/docs/rust-wiringpi/master/wiringpi/index.html)

##Example: Flashing Light

```Rust
extern crate wiringpi;

use std::time::Duration;

use wiringpi::pin::{High, Low};
use wiringpi::time::delay;

fn main() {
	//Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup().expect("WiringPi setup failed");

    //Use WiringPi pin 0 as output
    let pin = pi.output_pin(0);

    loop {
    	//Set pin 0 to high and wait one second
        pin.digital_write(High);
        delay(Duration::seconds(1));

    	//Set pin 0 to low and wait one second
        pin.digital_write(Low);
        delay(Duration::seconds(1));
    }
}

```

##Cross Compiling Using Cargo

This project can be cross compiled using Cargo.
[Follow these instructions](https://github.com/Ogeon/rust-on-raspberry-pi)
And use `./cross64 build` or `./cross32 build`, depending on your system,
to check if everything builds as expected.
