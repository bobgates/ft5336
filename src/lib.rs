//! A platform agnostic driver to interface with the FT5336 touch controller
//!
//! This driver was build using ['embedded-hal'] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use embedded_hal as hal;
use hal::blocking::i2c;

use core::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}
