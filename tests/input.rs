extern crate switch_hal;

use switch_hal::mock::{Pin, State};
use switch_hal::{InputSwitch, Switch};

mod active_high_switch {
    use super::*;

    use switch_hal::ActiveHigh;

    mod is_active {
        use super::*;

        #[test]
        fn true_when_pin_high() {
            let pin = Pin::with_state(State::High);

            let button = Switch::<_, ActiveHigh>::new(pin);
            assert_eq!(true, button.is_active().unwrap());
        }

        #[test]
        fn false_when_pin_low() {
            let pin = Pin::with_state(State::Low);

            let button = Switch::<_, ActiveHigh>::new(pin);
            assert_eq!(false, button.is_active().unwrap());
        }

        #[test]
        fn propogates_errors_from_pin() {
            let pin = Pin::new();
            let button = Switch::<_, ActiveHigh>::new(pin);
            button.is_active().expect_err("Expected unitialized error");
        }
    }
}

mod active_low_switch {
    use super::*;

    use switch_hal::ActiveLow;

    mod is_active {
        use super::*;

        #[test]
        fn false_when_pin_high() {
            let pin = Pin::with_state(State::High);

            let button = Switch::<_, ActiveLow>::new(pin);
            assert_eq!(false, button.is_active().unwrap());
        }

        #[test]
        fn true_when_pin_low() {
            let pin = Pin::with_state(State::Low);

            let button = Switch::<_, ActiveLow>::new(pin);
            assert_eq!(true, button.is_active().unwrap());
        }

        #[test]
        fn propogates_errors_from_pin() {
            let pin = Pin::new();
            let button = Switch::<_, ActiveLow>::new(pin);
            button.is_active().expect_err("Expected unitialized error");
        }
    }
}
