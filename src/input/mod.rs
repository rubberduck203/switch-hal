
pub trait InputSwitch {
    type Error;
    fn is_active(&self) -> Result<bool, Self::Error>;
}

use core::marker::PhantomData;
use embedded_hal::digital::v2::InputPin;
use crate::{ActiveHigh,ActiveLow};

/// Concrete implementation of [InputSwitch](InputSwitch)
//FIXME: identical to OutputSwitch except for type parameter
pub struct Switch<T, ActiveLevel>
where
    T: InputPin,
{
    pin: T,
    active: PhantomData<ActiveLevel>,
}

//FIXME: identical to OutputSwitch except for type parameter
impl<T: InputPin, ActiveLevel> Switch<T, ActiveLevel> {
    pub fn new(pin: T) -> Self {
        Switch {
            pin: pin,
            active: PhantomData::<ActiveLevel>,
        }
    }

    pub fn into_pin(self) -> T {
        self.pin
    }
}

impl<T: InputPin> InputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as InputPin>::Error;

    fn is_active(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<T: InputPin> InputSwitch for Switch<T, ActiveLow> {
    type Error = <T as InputPin>::Error;

    fn is_active(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}