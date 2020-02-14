use crate::{ActiveHigh, ActiveLow, InputSwitch, Switch};
use embedded_hal::digital::v2::InputPin;

impl<T: InputPin> InputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as InputPin>::Error;

    fn is_active(&self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<T: InputPin> InputSwitch for Switch<T, ActiveLow> {
    type Error = <T as InputPin>::Error;

    fn is_active(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}
