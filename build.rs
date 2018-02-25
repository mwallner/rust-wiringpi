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
