#![cfg_attr(feature="nightly", feature(duration))]
#![doc(html_root_url = "http://ogeon.github.io/docs/rust-wiringpi/master/")]

extern crate libc;

use std::marker::PhantomData;

use pin::{Pin, RequiresRoot};

macro_rules! impl_pins {
    ($($name:ident),+) => (
        $(
            #[derive(Clone, Copy)]
            pub struct $name;

            impl Pin for $name {}
        )+
    )
}

macro_rules! require_root {
    ($($name:ident: $pwm:expr),+) => (
        $(
            impl RequiresRoot for $name {
                #[inline]
                fn pwm_pin() -> PwmPin<$name> {
                    PwmPin::new($pwm)
                }
            }
        )+
    )
}

mod bindings;

pub mod time {
    use bindings;
    #[cfg(feature="nightly")]
    use std::time::Duration;

    ///This causes program execution to pause for at least the provided
    ///duration in milliseconds.
    ///
    ///Due to the multi-tasking nature of Linux it could be longer. Note that
    ///the maximum delay is an unsigned 32-bit integer or approximately 49
    ///days.
    #[cfg(feature="nightly")]
    pub fn delay(duration: Duration) {
        use libc;
        
        let duration = (duration.secs() * 1000) as u32 + duration.extra_nanos() / 1_000_000;
        if duration <= 0 {
            return;
        }

        unsafe {
            bindings::delay(duration as libc::c_uint);
        }
    }

    ///This causes program execution to pause for at least the provided number
    ///of milliseconds.
    ///
    ///Due to the multi-tasking nature of Linux it could be longer. Note that
    ///the maximum delay is an unsigned 32-bit integer or approximately 49
    ///days.
    #[cfg(not(feature="nightly"))]
    pub fn delay(duration: u32) {
        unsafe {
            bindings::delay(duration);
        }
    }

    ///This causes program execution to pause for at least the provided number
    ///of microseconds.
    ///
    ///Due to the multi-tasking nature of Linux it could be longer. Note that
    ///the maximum delay is an unsigned 32-bit integer microseconds or
    ///approximately 71 minutes.
    pub fn delay_microseconds(microseconds: u32) {
        unsafe {
            bindings::delayMicroseconds(microseconds);
        }
    }
}

pub mod thread {
    use bindings;
    use libc;

    ///This attempts to shift your program (or thread in a multi-threaded
    ///program) to a higher priority and enables a real-time scheduling.
    ///
    ///The priority parameter should be from 0 (the default) to 99 (the
    ///maximum). This won’t make your program go any faster, but it will give
    ///it a bigger slice of time when other programs are running. The priority
    ///parameter works relative to others – so you can make one program
    ///priority 1 and another priority 2 and it will have the same effect as
    ///setting one to 10 and the other to 90 (as long as no other programs are
    ///running with elevated priorities)
    ///
    ///The return value is `true` for success and `false` for error. If an
    ///error is returned, the program should then consult the _errno_ global
    ///variable, as per the usual conventions.
    ///
    ///_Note_: Only programs running as root can change their priority. If
    ///called from a non-root program then nothing happens.
    pub fn priority(priority: u8) -> bool {
        unsafe {
            bindings::piHiPri(priority as libc::c_int) >= 0
        }
    }
}

pub mod pin {
    use bindings;
    use libc;
    use self::Value::{Low, High};

    use std::marker::PhantomData;

    impl_pins!(WiringPi, Gpio, Phys, Sys);
    require_root!(WiringPi: 1, Gpio: 18, Phys: 12);

    pub trait Pin {}

    pub trait RequiresRoot: Pin {
        fn pwm_pin() -> PwmPin<Self>;
    }

    #[derive(Clone, Copy)]
    pub enum Value {
        Low = 0,
        High
    }

    #[derive(Clone, Copy)]
    pub enum Pull {
        Off = 0,
        Down,
        Up
    }

    #[derive(Clone, Copy)]
    pub enum PwmMode {
        MarkSpace = 0,
        Balanced
    }

    pub struct InputPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin> InputPin<P> {
        pub fn new(pin: libc::c_int) -> InputPin<P> {
            unsafe {
                bindings::pinMode(pin, 0);
            }

            InputPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &InputPin(number, _) = self;
            number
        }

        pub fn into_output(self) -> OutputPin<P> {
            let InputPin(number, _) = self;
            OutputPin::new(number)
        }

        ///This function returns the value read at the given pin.
        ///
        ///It will be `High` or `Low` (1 or 0) depending on the logic level at the pin.
        pub fn digital_read(&self) -> Value {
            let value = unsafe {
                bindings::digitalRead(self.number())
            };

            if value == 0 {
                Low
            } else {
                High
            }
        }
    }

    impl<P: Pin + RequiresRoot> InputPin<P> {
        pub fn into_pwm(self) -> PwmPin<P> {
            let InputPin(number, _) = self;
            PwmPin::new(number)
        }

        ///This sets the pull-up or pull-down resistor mode on the given pin.
        ///
        ///Unlike the Arduino, the BCM2835 has both pull-up an down internal
        ///resistors. The parameter pud should be; `Off`, (no pull up/down),
        ///`Down` (pull to ground) or `Up` (pull to 3.3v)
        pub fn pull_up_dn_control(&self, pud: Pull) {
            unsafe {
                bindings::pullUpDnControl(self.number(), pud as libc::c_int);
            }
        }
    }

    pub struct OutputPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin> OutputPin<P> {
        pub fn new(pin: libc::c_int) -> OutputPin<P> {
            unsafe {
                bindings::pinMode(pin, 1);
            }

            OutputPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &OutputPin(number, _) = self;
            number
        }

        pub fn into_input(self) -> InputPin<P> {
            let OutputPin(number, _) = self;
            InputPin::new(number)
        }

        ///Writes the value `High` or `Low` (1 or 0) to the given pin which must have been previously set as an output.
        pub fn digital_write(&self, value: Value) {
            unsafe {
                bindings::digitalWrite(self.number(), value as libc::c_int);
            }
        }
    }

    impl<P: Pin + RequiresRoot> OutputPin<P> {
        pub fn into_pwm(self) -> PwmPin<P> {
            let OutputPin(number, _) = self;
            PwmPin::new(number)
        }
    }

    ///To understand more about the PWM system, you’ll need to read the Broadcom ARM peripherals manual.
    pub struct PwmPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin + RequiresRoot> PwmPin<P> {
        pub fn new(pin: libc::c_int) -> PwmPin<P> {
            unsafe {
                bindings::pinMode(pin, 2);
            }

            PwmPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &PwmPin(number, _) = self;
            number
        }

        pub fn into_input(self) -> InputPin<P> {
            let PwmPin(number, _) = self;
            InputPin::new(number)
        }

        pub fn into_output(self) -> OutputPin<P> {
            let PwmPin(number, _) = self;
            OutputPin::new(number)
        }

        ///Writes the value to the PWM register for the given pin.
        ///
        ///The value must be between 0 and 1024.
        pub fn write(&self, value: u16) {
            unsafe {
                bindings::pwmWrite(self.number(), value as libc::c_int);
            }
        }

        ///The PWM generator can run in 2 modes – "balanced" and "mark:space".
        ///
        ///The mark:space mode is traditional, however the default mode in the
        ///Pi is "balanced". You can switch modes by supplying the parameter:
        ///`Balanced` or `MarkSpace`.
        pub fn set_mode(&self, mode: PwmMode) {
            unsafe {
                bindings::pwmSetMode(mode as libc::c_int);
            }
        }

        ///This sets the range register in the PWM generator. The default is 1024.
        pub fn set_range(&self, value: u16) {
            unsafe {
                bindings::pwmSetRange(value as libc::c_uint);
            }
        }

        ///This sets the divisor for the PWM clock.
        pub fn set_clock(&self, value: u16) {
            unsafe {
                bindings::pwmSetClock(value as libc::c_int);
            }
        }
    }
}

///This initialises the wiringPi system and assumes that the calling program
///is going to be using the **wiringPi** pin numbering scheme.
///
///This is a simplified numbering scheme which provides a mapping from virtual
///pin numbers 0 through 16 to the real underlying Broadcom GPIO pin numbers.
///See the pins page for a table which maps the **wiringPi** pin number to the
///Broadcom GPIO pin number to the physical location on the edge connector.
///
///This function needs to be called with root privileges.
pub fn setup() -> WiringPi<pin::WiringPi> {
    unsafe { bindings::wiringPiSetup(); }
    WiringPi(PhantomData)
}

///This is identical to `setup()`, however it allows the calling programs to
///use the Broadcom GPIO pin numbers directly with no re-mapping.
///
///This function needs to be called with root privileges.
pub fn setup_gpio() -> WiringPi<pin::Gpio> {
    unsafe { bindings::wiringPiSetupGpio(); }
    WiringPi(PhantomData)
}

///This is identical to `setup()`, however it allows the calling programs to
///use the physical pin numbers _on the P1 connector only_.
///
///This function needs to be called with root privileges.
pub fn setup_phys() -> WiringPi<pin::Phys> {
    unsafe { bindings::wiringPiSetupPhys(); }
    WiringPi(PhantomData)
}

///This initialises the wiringPi system but uses the /sys/class/gpio interface
///rather than accessing the hardware directly.
///
///This can be called as a non-root user provided the GPIO pins have been
///exported before-hand using the gpio program. Pin number in this mode is the
///native Broadcom GPIO numbers.
///
///_Note_: In this mode you can only use the pins which have been exported via
///the /sys/class/gpio interface. You must export these pins before you call
///your program. You can do this in a separate shell-script, or by using the
///system() function from inside your program.
///
///Also note that some functions have no effect when using this mode as
///they’re not currently possible to action unless called with root
///privileges.
pub fn setup_sys() -> WiringPi<pin::Sys> {
    unsafe { bindings::wiringPiSetupSys(); }
    WiringPi(PhantomData)
}

///This returns the board revision of the Raspberry Pi.
///
///It will be either 1 or 2. Some of the BCM_GPIO pins changed number and
///function when moving from board revision 1 to 2, so if you are using
///BCM_GPIO pin numbers, then you need to be aware of the differences.
pub fn board_revision() -> i32 {
    unsafe {
        bindings::piBoardRev()
    }
}

///This returns the BCM_GPIO pin number of the supplied **wiringPi** pin.
///
///It takes the board revision into account.
pub fn to_gpio_number(wpi_number: u16) -> u16 {
    unsafe {
        bindings::wpiPinToGpio(wpi_number as libc::c_int) as u16
    }
}

pub struct WiringPi<Pin>(PhantomData<Pin>);

impl<P: Pin> WiringPi<P> {
    pub fn input_pin(&self, pin: u16) -> pin::InputPin<P> {
        let pin = pin as libc::c_int;
        pin::InputPin::new(pin)
    }

    pub fn output_pin(&self, pin: u16) -> pin::OutputPin<P> {
        let pin = pin as libc::c_int;
        pin::OutputPin::new(pin)
    }

    ///This returns a number representing the number if milliseconds since
    ///your program called one of the setup functions.
    ///
    ///It returns an unsigned 32-bit number which wraps after 49 days.
    pub fn millis(&self) -> u32 {
        unsafe {
            bindings::millis()
        }
    }

    ///This returns a number representing the number if microseconds since
    ///your program called one of the setup functions.
    ///
    ///It returns an unsigned 32-bit number which wraps after 71 minutes.

    pub fn micros(&self) -> u32 {
        unsafe {
            bindings::micros()
        }
    }
}

impl<P: RequiresRoot + Pin> WiringPi<P> {
    pub fn pwm_pin(&self) -> pin::PwmPin<P> {
        RequiresRoot::pwm_pin()
    }
}