
use switch_hal::mock::{Pin, State};

mod output_pin {
    use super::*;

    use embedded_hal::digital::v2::InputPin;
    use switch_hal::{OutputSwitch, OutputPinExt};

    #[test]
    fn into_active_high() {
        let pin = Pin::new();
        let mut switch = pin.into_active_high_output();
        switch.on().unwrap();

        let pin = switch.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn into_active_low() {
        let pin = Pin::new();
        let mut switch = pin.into_active_low_output();
        switch.on().unwrap();

        let pin = switch.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }
}

mod input_pin {
    use super::*;
    use switch_hal::{InputSwitch, InputPinExt};

    #[test]
    fn into_active_high() {
        let pin = Pin::with_state(State::High);
        let switch = pin.into_active_high_input();
        assert_eq!(true, switch.is_active().unwrap());
    }

    #[test]
    fn into_active_low() {
        let pin = Pin::with_state(State::Low);
        let switch = pin.into_active_low_input();
        assert_eq!(true, switch.is_active().unwrap());
    }
}