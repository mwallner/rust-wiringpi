extern crate wiringpi;

use wiringpi::pin::Value::High;
use std::time::Duration;
use std::thread;

fn main() {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    // Use pins 23 and 25 as software PWM output
    // (note that the only hardware PWM pin is 18)
    let mut alice = pi.soft_pwm_pin(23);
    let bob = pi.soft_pwm_pin(25);

    // Use a duty cycle of 0.5 on both pins
    alice.pwm_write(50);
    bob.pwm_write(50);

    thread::sleep(Duration::from_millis(2000));

    // Switch `alice` (pin 23) to output mode and turn it on
    let alice_out = alice.into_output();
    alice_out.digital_write(High);
    // Change the duty cycle of `bob` to 1
    bob.pwm_write(100);

    // Both pins now output the same signal, `alice` via software PWM,
    // `bob` via constant output

    thread::sleep(Duration::from_millis(2000));

    // Switch `alice_out` (pin 23) back to software PWM mode
    alice = alice_out.into_soft_pwm();
    alice.pwm_write(50);
    bob.pwm_write(50);

    thread::sleep(Duration::from_millis(2000));

    alice.pwm_write(0);
    bob.pwm_write(0);
}
