//! In here all hardware dependent code is kept, and to run the independent parts the firmware crate
//! is called.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
pub use firmware as _;
use nrf52832_hal as _;
use panic_semihosting as _; // The pub makes the firmware visible in cargo doc

#[entry]
fn main() -> ! {
    // Some init code...
    firmware::init();

    loop {
        // Some idle code...
        firmware::idle();
    }
}
