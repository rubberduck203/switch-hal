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

pub struct ActiveHighOutputSwitch<T>
where
    T: OutputPin,
{
    pin: T,
}

impl<T: OutputPin> ActiveHighOutputSwitch<T> {
    pub fn new(pin: T) -> Self {
        ActiveHighOutputSwitch { pin: pin }
    }

    pub fn into_pin(self) -> T {
        self.pin
    }
}

impl<T: OutputPin> OutputSwitch for ActiveHighOutputSwitch<T> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin + ToggleableOutputPin> ToggleableOutputSwitch for ActiveHighOutputSwitch<T> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}
