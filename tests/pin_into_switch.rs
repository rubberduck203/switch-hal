use switch_hal::mock::{Pin, State};
use switch_hal::IntoSwitch;

mod output_pin {
    use super::*;

    use embedded_hal::digital::v2::InputPin;
    use switch_hal::OutputSwitch;

    #[test]
    fn active_high() {
        let pin = Pin::new();
        let mut switch = pin.into_active_high_switch();
        switch.on().unwrap();

        let pin = switch.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn active_low() {
        let pin = Pin::new();
        let mut switch = pin.into_active_low_switch();
        switch.on().unwrap();

        let pin = switch.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }
}

mod input_pin {
    use super::*;
    use switch_hal::InputSwitch;

    #[test]
    fn active_high() {
        let pin = Pin::with_state(State::High);
        let switch = pin.into_active_high_switch();
        assert_eq!(true, switch.is_active().unwrap());
    }

    #[test]
    fn active_low() {
        let pin = Pin::with_state(State::Low);
        let switch = pin.into_active_low_switch();
        assert_eq!(true, switch.is_active().unwrap());
    }
}
