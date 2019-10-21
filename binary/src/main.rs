//! In here all hardware dependent code is kept, and to run the independent parts the firmware crate
//! is called.

#![no_main]
#![no_std]

pub use firmware as _;
use nrf52832_hal as _;
use nrf52832_hal::target;
// use panic_halt as _;
use panic_semihosting as _;
use rtfm::app;

#[app(device = nrf52832_hal::target, peripherals = true)]
const APP: () = {
    struct Resources {
        spi: target::SPIM0,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        init::LateResources {
            spi: cx.device.SPIM0,
        }
    }
};
