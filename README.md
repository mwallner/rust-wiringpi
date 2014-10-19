#WiringPi Bindings for Rust

An API wrapper for [WiringPi](http://wiringpi.com/) to make it accessible
using Rust. It implements the most important functions and provides a bit of
type system convenience.

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

##Cross Compiling using Cargo

This guide assumes that you are using some kind of Unix or Linux like
environment. Feel free to send adaptations for other platforms and
environments.

Follow [this guide](https://github.com/npryce/rusty-pi/blob/master/doc/compile-the-compiler.asciidoc)
to compile Rust and the standard libraries. This will take a while, so go grab
something to eat or take a walk in the meantime. We'll assume that Rust is
located in `~/pi-rust` and that the Raspberry Pi toolkit is located in
`~/pi-tools`. Feel free to change this however you want, but then you will
have to tweak things.

This is enough to cross compile libraries. There is just one more thing left
before a binary can be successfully linked. Rust uses the `cc` command to link
binaries and the Raspberry Pi toolkit doesn't provide anything called `cc`, so
we'll just link `gcc` to `cc`:

```
ln -s ~/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian/bin/arm-linux-gnueabihf-gcc ~/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian/bin/cc
```

The last step is to compile. There is a script in the project root called
`cross`. It can be used as a shortcut and sandbox to avoid polluting the
environment variables. It takes a Cargo command and two optional paths:
`./cross [build, doc, ...] path/to/rust path/to/pi/toolkit`. They can be used
if Rust and the Raspberry Pi toolkit was installed somewhere else. `./cross
build`, `./cross "build --release"` or `./cross doc` will do just fine otherwise.

This script will add the `bin` directories from Rust and the Raspberry Pi
toolkit to `PATH`, add the `lib` directory from Rust to `LD_LIBRARY_PATH` and
run `cargo [command] --target=arm-unknown-linux-gnueabihf`.

That's all! You can now use Cargo manifests as usual and there's no need to
fiddle around with environment variables and stuff. Just copy `cross` to your
own project, customize it to your needs and use it instead of Cargo for
building and documentation.

Have fun!
