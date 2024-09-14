extern crate cc;
extern crate glob;

#[cfg(not(feature = "orangepi"))]
const TARGET: &'static str = "wiringPi";
#[cfg(feature = "orangepi")]
const TARGET: &'static str = "WiringOP";

fn main() {
    if cfg!(feature = "development") {
        return;
    }

    // only build wiringpi/wiringop for arm/armv7 platforms

    let target = std::env::var("TARGET").unwrap();
    if !(target.starts_with("arm-")
        || target.starts_with("armv7-")
        || target.starts_with("aarch64"))
    {
        println!("cargo:rustc-cfg=feature=\"development\"");
        return;
    }

    cc::Build::new()
        .files(
            glob::glob(&format!("{}/wiringPi/*.c", TARGET))
                .unwrap()
                .filter_map(|x| x.ok()),
        )
        .include(format!("{}/", TARGET))
        .include(format!("{}/wiringPi/", TARGET))
        .static_flag(true)
        .compile("wiringpi");
}
