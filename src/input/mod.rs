use crate::{ActiveHigh, ActiveLow, InputSwitch, Switch};
use embedded_hal::digital::v2::InputPin;

// Results in conflicting trait implementations

// pub trait InputSwitch<ActiveLevel> {
//     type Error;
//     fn is_active(&self) -> Result<bool, Self::Error>;
// }

// pub trait AH {}
// pub trait AL {}

// impl<T: InputPin, U: AH> InputSwitch<U> for T {
//     type Error = <T as InputPin>::Error;

//     fn is_active(&self) -> Result<bool, Self::Error> {
//         self.is_low()
//     }
// }

// impl<T: InputPin, U: AL> InputSwitch<U> for T {
//     type Error = <T as InputPin>::Error;

//     fn is_active(&self) -> Result<bool, Self::Error> {
//         self.is_high()
//     }
// }

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
