use embedded_hal::digital::{Error, ErrorType, InputPin, OutputPin, StatefulOutputPin};

/// Inverted input/output pin
///
/// If wrapping an output pin, whenever setting this pin to a high or low level,
/// the wrapped pin will be set to the opposite level.
///
/// Likewise, if wrapping an input pin, whenever reading this pin it will read
/// the wrapped input pin and return the opposite level.
#[derive(Debug, Clone, Copy)]
pub struct InvertedPin<P> {
    pin: P,
}

impl<P> InvertedPin<P> {
    /// Create new instance
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Destroy instance and return the wrapped pin
    pub fn destroy(self) -> P {
        self.pin
    }
}

impl<P, E> ErrorType for InvertedPin<P>
where
    P: ErrorType<Error = E>,
    E: Error,
{
    type Error = E;
}

impl<P, E> OutputPin for InvertedPin<P>
where
    P: OutputPin<Error = E>,
    E: Error,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<P, E> InputPin for InvertedPin<P>
where
    P: InputPin<Error = E>,
    E: Error,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<P, E> StatefulOutputPin for InvertedPin<P>
where
    P: StatefulOutputPin<Error = E>,
    E: Error,
{
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }
}
