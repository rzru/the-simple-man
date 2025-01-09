#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use app::run;
// use app_ret::run;

mod app;
mod app_bc;
mod app_ret;
mod background;
mod tiles;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    run(gba)
}
