use embedded_hal::digital::OutputPin;
use inverted_pin::InvertedPin;
use linux_embedded_hal::SysfsPin;

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
    let real_pin = SysfsPin::new(25);
    let mut driver_with_real_pin = Driver::new(real_pin);
    driver_with_real_pin.do_something().unwrap();
    let real_pin = driver_with_real_pin.destroy();

    let inverted_pin = InvertedPin::new(real_pin);
    let mut driver_with_inverted_pin = Driver::new(inverted_pin);
    driver_with_inverted_pin.do_something().unwrap();
}
