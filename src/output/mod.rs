use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

pub trait OutputSwitch {
    type Error;

    fn on(&mut self) -> Result<(), Self::Error>;
    fn off(&mut self) -> Result<(), Self::Error>;
}

pub trait ToggleableOutputSwitch {
    type Error;

    fn toggle(&mut self) -> Result<(), Self::Error>;
}

use core::marker::PhantomData;

pub struct ActiveHigh;
pub struct ActiveLow;

pub struct Switch<T, Activeness>
where
    T: OutputPin,
{
    pin: T,
    active: PhantomData<Activeness>,
}

impl<T: OutputPin, Activeness> Switch<T, Activeness> {
    pub fn new(pin: T) -> Self {
        Switch {
            pin: pin,
            active: PhantomData::<Activeness>,
        }
    }

    pub fn into_pin(self) -> T {
        self.pin
    }
}

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

impl<T: OutputPin + ToggleableOutputPin, Activeness> ToggleableOutputSwitch for Switch<T, Activeness> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}
