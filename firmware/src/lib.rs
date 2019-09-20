//! In this crate one implements all firmware features that are not hardware dependent. If there is
//! a hardware dependence that wants to be transfered, use a trait and implement the trait for said
//! hardware in the binary crate, this way the firmware crate can be kept independent of the binary
//! crate.

#![no_std]

pub fn init() {
    // ...
}

pub fn idle() {
    // ...
}

//
// Testing requirements
//
#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;
