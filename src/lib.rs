#![doc(html_root_url = "http://ogeon.github.io/docs/rust-wiringpi/master/")]

extern crate libc;

use std::marker::PhantomData;

use pin::{Pin, Pwm, GpioClock, RequiresRoot};

macro_rules! impl_pins {
    ($($name:ident),+) => (
        $(
            #[derive(Clone, Copy)]
            pub struct $name;

            impl Pin for $name {}
        )+
    )
}

macro_rules! impl_pwm {
    ($($name:ident: $pwm:expr),+) => (
        $(
            impl Pwm for $name {
                #[inline]
                fn pwm_pin() -> PwmPin<$name> {
                    PwmPin::new($pwm)
                }
            }
        )+
    )
}

macro_rules! impl_clock {
    ($($name:ident: $pwm:expr),+) => (
        $(
            impl GpioClock for $name {
                #[inline]
                fn clock_pin() -> ClockPin<$name> {
                    ClockPin::new($pwm)
                }
            }
        )+
    )
}

macro_rules! require_root {
    ($($name:ident),+) => (
        $(
            impl RequiresRoot for $name {}
        )+
    )
}

mod bindings;

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

    const INPUT: libc::c_int = 0;
    const OUTPUT: libc::c_int = 1;
    const PWM_OUTPUT: libc::c_int = 2;
    const GPIO_CLOCK: libc::c_int = 3;
    //const SOFT_PWM_OUTPUT: libc::c_int = 4;
    //const SOFT_TONE_OUTPUT: libc::c_int = 5;
    //const PWM_TONE_OUTPUT: libc::c_int = 6;

    ///This returns the BCM_GPIO pin number of the supplied **wiringPi** pin.
    ///
    ///It takes the board revision into account.
    pub fn wpi_to_gpio_number(wpi_number: u16) -> u16 {
        unsafe {
            bindings::wpiPinToGpio(wpi_number as libc::c_int) as u16
        }
    }

    ///This returns the BCM_GPIO pin number of the supplied physical pin on
    ///the P1 connector.
    pub fn phys_to_gpio_number(phys_number: u16) -> u16 {
        unsafe {
            bindings::physPinToGpio(phys_number as libc::c_int) as u16
        }
    }

    impl_pins!(WiringPi, Gpio, Phys, Sys);
    impl_pwm!(WiringPi: 1, Gpio: 18, Phys: 12);
    impl_clock!(WiringPi: 7, Gpio: 4, Phys: 7);
    require_root!(WiringPi, Gpio, Phys);

    pub trait Pin {}

    pub trait Pwm: RequiresRoot + Sized {
        fn pwm_pin() -> PwmPin<Self>;
    }

    pub trait GpioClock: RequiresRoot + Sized {
        fn clock_pin() -> ClockPin<Self>;
    }

    pub trait RequiresRoot: Pin {}

    #[derive(Clone, Copy, PartialEq, Eq)]
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
                bindings::pinMode(pin, INPUT);
            }

            InputPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &InputPin(number, _) = self;
            number
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

        ///This returns the value read on the supplied analog input pin. You
        ///will need to register additional analog modules to enable this
        ///function for devices such as the Gertboard, quick2Wire analog
        ///board, etc.
        pub fn analog_read(&self) -> u16 {
            unsafe {
                bindings::analogRead(self.number()) as u16
            }
        }
    }

    impl<P: Pin + RequiresRoot> InputPin<P> {
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

        pub fn into_output(self) -> OutputPin<P> {
            let InputPin(number, _) = self;
            OutputPin::new(number)
        }

        pub fn into_soft_pwm(self) -> SoftPwmPin<P> {
            let InputPin(number, _) = self;
            SoftPwmPin::new(number)
        }
    }

    impl<P: Pin + Pwm> InputPin<P> {
        pub fn into_pwm(self) -> PwmPin<P> {
            let InputPin(number, _) = self;
            PwmPin::new(number)
        }
    }

    impl<P: Pin + GpioClock> InputPin<P> {
        pub fn into_clock(self) -> ClockPin<P> {
            let InputPin(number, _) = self;
            ClockPin::new(number)
        }
    }

    /// A pin with software controlled PWM output.
    ///
    /// Due to limitations of the chip only one pin is able to do
    /// hardware-controlled PWM output. The `SoftPwmPin`s on the
    /// other hand allow for all GPIOs to output PWM signals.
    ///
    /// The pulse width of the signal will be 100μs with a value range
    /// of [0,100] \(where `0` is a constant low and `100` is a
    /// constant high) resulting in a frequenzy of 100 Hz.
    ///
    /// **Important**: In order to use software PWM pins *wiringPi*
    /// has to be setup in GPIO mode via `setup_gpio()`.
    pub struct SoftPwmPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin + RequiresRoot> SoftPwmPin<P> {
        /// Configures the given `pin` to output a software controlled PWM
        /// signal.
        pub fn new(pin: libc::c_int) -> SoftPwmPin<P> {
            unsafe {
                bindings::softPwmCreate(pin, 0, 100);
            }

            SoftPwmPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &SoftPwmPin(number, _) = self;
            number
        }

        /// Sets the duty cycle.
        ///
        /// `value` has to be in the interval [0,100].
        pub fn pwm_write(&self, value: libc::c_int) {
            unsafe {
                bindings::softPwmWrite(self.number(), value);
            }
        }

        /// Stops the software handling of this pin.
        ///
        /// _Note_: In order to control this pin via software PWM again
        /// it will need to be recreated using `new()`.
        pub fn pwm_stop(self) {
            unsafe {
                bindings::softPwmStop(self.number());
            }
        }

        pub fn into_input(self) -> InputPin<P> {
            let SoftPwmPin(number, _) = self;
            self.pwm_stop();
            InputPin::new(number)
        }

        pub fn into_output(self) -> OutputPin<P> {
            let SoftPwmPin(number, _) = self;
            self.pwm_stop();
            OutputPin::new(number)
        }

    }

    impl<P: Pin + Pwm> SoftPwmPin<P> {
        pub fn into_pwm(self) -> PwmPin<P> {
            let SoftPwmPin(number, _) = self;
            self.pwm_stop();
            PwmPin::new(number)
        }
    }

    impl<P: Pin + GpioClock> SoftPwmPin<P> {
        pub fn into_clock(self) -> ClockPin<P> {
            let SoftPwmPin(number, _) = self;
            self.pwm_stop();
            ClockPin::new(number)
        }
    }

    pub struct OutputPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin> OutputPin<P> {
        pub fn new(pin: libc::c_int) -> OutputPin<P> {
            unsafe {
                bindings::pinMode(pin, OUTPUT);
            }

            OutputPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &OutputPin(number, _) = self;
            number
        }

        ///Writes the value `High` or `Low` (1 or 0) to the given pin which must have been previously set as an output.
        pub fn digital_write(&self, value: Value) {
            unsafe {
                bindings::digitalWrite(self.number(), value as libc::c_int);
            }
        }

        ///This writes the given value to the supplied analog pin. You will
        ///need to register additional analog modules to enable this function
        ///for devices such as the Gertboard.
        pub fn analog_write(&self, value: u16) {
            unsafe {
                bindings::analogWrite(self.number(), value as libc::c_int);
            }
        }

    }

    impl<P: Pin + RequiresRoot> OutputPin<P> {
        pub fn into_soft_pwm(self) -> SoftPwmPin<P> {
            let OutputPin(number, _) = self;
            SoftPwmPin::new(number)
        }
    }

    impl<P: Pin + RequiresRoot> OutputPin<P> {
        pub fn into_input(self) -> InputPin<P> {
            let OutputPin(number, _) = self;
            InputPin::new(number)
        }
    }

    impl<P: Pin + Pwm> OutputPin<P> {
        pub fn into_pwm(self) -> PwmPin<P> {
            let OutputPin(number, _) = self;
            PwmPin::new(number)
        }
    }

    impl<P: Pin + GpioClock> OutputPin<P> {
        pub fn into_clock(self) -> ClockPin<P> {
            let OutputPin(number, _) = self;
            ClockPin::new(number)
        }
    }

    ///To understand more about the PWM system, you’ll need to read the Broadcom ARM peripherals manual.
    pub struct PwmPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin + Pwm> PwmPin<P> {
        pub fn new(pin: libc::c_int) -> PwmPin<P> {
            unsafe {
                bindings::pinMode(pin, PWM_OUTPUT);
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

        pub fn into_soft_pwm(self) -> SoftPwmPin<P> {
            let PwmPin(number, _) = self;
            SoftPwmPin::new(number)
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

    pub struct ClockPin<Pin>(libc::c_int, PhantomData<Pin>);

    impl<P: Pin + GpioClock> ClockPin<P> {
        pub fn new(pin: libc::c_int) -> ClockPin<P> {
            unsafe {
                bindings::pinMode(pin, GPIO_CLOCK);
            }

            ClockPin(pin, PhantomData)
        }

        #[inline]
        pub fn number(&self) -> libc::c_int {
            let &ClockPin(number, _) = self;
            number
        }

        pub fn into_input(self) -> InputPin<P> {
            let ClockPin(number, _) = self;
            InputPin::new(number)
        }

        pub fn into_output(self) -> OutputPin<P> {
            let ClockPin(number, _) = self;
            OutputPin::new(number)
        }

        pub fn into_soft_pwm(self) -> SoftPwmPin<P> {
            let ClockPin(number, _) = self;
            SoftPwmPin::new(number)
        }

        ///Set the freuency on a GPIO clock pin.
        pub fn frequency(&self, freq: u16) {
            unsafe {
                bindings::gpioClockSet(self.number(), freq as libc::c_int);
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

    ///This writes the 8-bit byte supplied to the first 8 GPIO pins. It’s the
    ///fastest way to set all 8 bits at once to a particular value, although
    ///it still takes two write operations to the Pi’s GPIO hardware.
    pub fn digital_write_byte(&self, byte: u8) {
        unsafe {
            bindings::digitalWriteByte(byte as libc::c_int);
        }
    }
}

impl<P: Pwm + Pin> WiringPi<P> {
    pub fn pwm_pin(&self) -> pin::PwmPin<P> {
        Pwm::pwm_pin()
    }
}

impl<P: GpioClock + Pin> WiringPi<P> {
    pub fn clock_pin(&self) -> pin::ClockPin<P> {
        GpioClock::clock_pin()
    }
}

impl<P: Pin + RequiresRoot> WiringPi<P> {
    pub fn soft_pwm_pin(&self, pin: u16) -> pin::SoftPwmPin<P> {
        let pin = pin as libc::c_int;
        pin::SoftPwmPin::new(pin)
    }
}
