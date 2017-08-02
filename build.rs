use std::process::Command;
use std::env;

#[cfg(not(feature = "orangepi"))]
const TARGET: &'static str = "wiringpi";
#[cfg(feature = "orangepi")]
const TARGET: &'static str = "wiringop";

fn main() {
    if cfg!(feature = "development") { return; }

    let out_dir = env::var("OUT_DIR").unwrap();
    match Command::new("make").arg("-e").arg(TARGET).status() {
        Ok(status) if !status.success() => panic!("failed to build wiringPi C library (exit code {:?})", status.code()),
        Err(e) => panic!("failed to build wiringPi C library: {}", e),
        _ => {}
    }
    println!("cargo:rustc-flags=-L native={} -l static=wiringpi", out_dir);
}
