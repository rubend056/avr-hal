//! GPIO & Pin control.
//!
//! This module contains a [`Pins`] struct which represents all pins of the board.  The [`Pins`]
//! struct is most easily constructed using the [`arduino_hal::pins!()`][crate::pins] macro:
//!
//! ```no_run
//! let dp = arduino_hal::Peripherals::take().unwrap();
//! let pins = arduino_hal::pins!(dp);
//! ```
//!
//! Additionally, the [`mode`] submodule contains all valid types for the `MODE` generic parameter
//! of a pin.  The [`Pin`] type-alias represents a pin which can represent _any_ of the pins
//! dynamically (while usually each pin has its own type).
//!
//! Check the documentation for [`avr_hal_generic::port::Pin`] for a detailed explanation of GPIO
//! pins in `avr-hal`.

#[cfg(feature = "arduino-diecimila")]
mod diecimila;
#[cfg(feature = "arduino-diecimila")]
pub use diecimila::*;
#[cfg(feature = "arduino-leonardo")]
mod leonardo;
#[cfg(feature = "arduino-leonardo")]
pub use leonardo::*;
#[cfg(any(feature = "arduino-mega2560", feature = "arduino-mega1280"))]
mod mega;
#[cfg(any(feature = "arduino-mega2560", feature = "arduino-mega1280"))]
pub use mega::*;

#[cfg(feature = "samn8")]
mod samn8;
#[cfg(feature = "samn8")]
pub use samn8::*;
#[cfg(feature = "samn9")]
mod samn9;
#[cfg(feature = "samn9")]
pub use samn9::*;
#[cfg(feature = "samn_dc")]
mod samn_dc;
#[cfg(feature = "samn_dc")]
pub use samn_dc::*;
#[cfg(feature = "samn_switch")]
mod samn_switch;
#[cfg(feature = "samn_switch")]
pub use samn_switch::*;



#[cfg(any(feature = "arduino-nano", feature = "arduino-uno", feature = "nano168", feature = "sparkfun-promini-5v"))]
mod uno;
#[cfg(any(feature = "arduino-nano", feature = "arduino-uno", feature = "nano168", feature = "sparkfun-promini-5v"))]
pub use uno::*;

#[cfg(feature = "sparkfun-promicro")]
mod promicro;
#[cfg(feature = "sparkfun-promicro")]
pub use promicro::*;
#[cfg(feature = "trinket-pro")]
mod trinket_pro;
#[cfg(feature = "trinket-pro")]
pub use trinket_pro::*;
#[cfg(feature = "trinket")]
mod trinket;
#[cfg(feature = "trinket")]
pub use trinket::*;
