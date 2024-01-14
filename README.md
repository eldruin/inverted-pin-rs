# Inverted Input/Output Pin Implementations

[![crates.io](https://img.shields.io/crates/v/inverted-pin.svg)](https://crates.io/crates/inverted-pin)
[![Docs](https://docs.rs/inverted-pin/badge.svg)](https://docs.rs/inverted-pin)
![MSRV](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![Build Status](https://github.com/eldruin/inverted-pin-rs/workflows/Build/badge.svg)](https://github.com/eldruin/inverted-pin-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/inverted-pin-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/inverted-pin-rs?branch=master)


This provides implementations of the input/output pin [`embedded-hal`] traits with inverted logic.

For example, an `InvertedPin` can wrap an `OutputPin` and when setting it low, it will set the
wrapped `OutputPin` high. It works similarly for an `InputPin` as well.

This is useful when dealing with pins that use a logic that is inverted with respect to what
the rest of the system expects.

Since an `InvertedPin` implements the `OutputPin` and `InputPin` traits as well, it can be used
just like any other `OutputPin` or `InputPin` and serves as a drop-in replacement of the wrapped pin.

## Usage

This example demonstrates how the same driver can operate with either a normal or an inverted output pin.

```rust
use embedded_hal::digital::v2::OutputPin;
use inverted_pin::InvertedPin;
use linux_embedded_hal::Pin;

struct Driver<P> {
    output: P,
}

impl<P, E> Driver<P>
where
    P: OutputPin<Error = E>,
{
    fn new(pin: P) -> Self {
        Driver { output: pin }
    }

    fn do_something(&mut self) -> Result<(), E> {
        // ...
        self.output.set_high()
    }

    fn destroy(self) -> P {
        self.output
    }
}

fn main() {
    // The same driver can operate with either a normal or an inverted pin.
    let real_pin = Pin::new(25);
    let mut driver_with_real_pin = Driver::new(real_pin);
    driver_with_real_pin.do_something().unwrap();
    let real_pin = driver_with_real_pin.destroy();

    let inverted_pin = InvertedPin::new(real_pin);
    let mut driver_with_inverted_pin = Driver::new(inverted_pin);
    driver_with_inverted_pin.do_something().unwrap();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/inverted-pin-rs/issues).

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.35 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
