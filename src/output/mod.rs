use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

use crate::{ActiveHigh, ActiveLow, OutputSwitch, Switch, ToggleableOutputSwitch};

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveLow> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: OutputPin + ToggleableOutputPin, ActiveLevel> ToggleableOutputSwitch
    for Switch<T, ActiveLevel>
{
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}
