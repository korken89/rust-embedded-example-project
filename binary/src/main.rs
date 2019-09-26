//! In here all hardware dependent code is kept, and to run the independent parts the firmware crate
//! is called.

#![no_main]
#![no_std]

pub use firmware as _;
use nrf52832_hal as _;
use nrf52832_hal::target;
use panic_semihosting as _;
use rtfm::app;

#[app(device = nrf52832_hal::target)]
const APP: () = {
    static mut SPI: target::SPIM0 = ();

    #[init]
    fn init() -> init::LateResources {
        init::LateResources { SPI: device.SPIM0 }
    }
};
