//! This provides implementations of the input/output pin [`embedded-hal`] traits with inverted logic.
//!
//! For example, an `InvertedPin` can wrap an `OutputPin` and when setting it low, it will set the
//! wrapped `OutputPin` high. It works similarly for an `InputPin` as well.
//!
//! This is useful when dealing with pins that use a logic that is inverted with respect to what
//! the rest of the system expects.
//!
//! Since an `InvertedPin` implements the `OutputPin` and `InputPin` traits as well, it can be used
//! just like any other `OutputPin` or `InputPin` and serves as a drop-in replacement of the wrapped pin.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! ## Usage examples
//!
//! ### Output `InvertedPin` interaction
//!
//! ```no_run
//! # use embedded_hal::digital::OutputPin;
//! use linux_embedded_hal::SysfsPin;
//! use inverted_pin::InvertedPin;
//!
//! let pin = SysfsPin::new(25);
//! let mut inverted_pin = InvertedPin::new(pin);
//!
//! // This calls `pin.set_low()`
//! inverted_pin.set_high().unwrap();
//! ```
//!
//! ### Input `InvertedPin` interaction
//!
//! ```no_run
//! # use embedded_hal::digital::InputPin;
//! use linux_embedded_hal::SysfsPin;
//! use inverted_pin::InvertedPin;
//!
//! let pin = SysfsPin::new(25);
//! let mut inverted_pin = InvertedPin::new(pin);
//!
//! // This returns the result of calling `pin.is_low()`
//! inverted_pin.is_high().unwrap();
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod inverted;
pub use crate::inverted::InvertedPin;
