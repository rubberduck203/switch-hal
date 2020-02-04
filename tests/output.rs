extern crate switch_hal;

use embedded_hal_mock::pin::{Mock, State, Transaction};

mod active_high_switch {
    use super::*;
    use switch_hal::output::{ActiveHighOutputSwitch, OutputSwitch};

    #[test]
    fn when_on_pin_is_high() {
        let expectations = [Transaction::set(State::High)];
        let pin = Mock::new(&expectations);

        let mut led = ActiveHighOutputSwitch::new(pin);
        led.on().unwrap();

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }

    #[test]
    fn when_off_pin_is_low() {
        let expectations = [Transaction::set(State::Low)];
        let pin = Mock::new(&expectations);

        let mut led = ActiveHighOutputSwitch::new(pin);
        led.off().unwrap();

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }

    #[ignore] //https://github.com/dbrgn/embedded-hal-mock/issues/30
    fn is_toggleable() {
        use switch_hal::output::ToggleableOutputSwitch;

        let expectations = [
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::High),
        ];
        let pin = Mock::new(&expectations);

        let mut led = ActiveHighOutputSwitch::new(pin);
        led.off().unwrap();

        for i in 0..3 {
            //TODO: mock_hal doesn't implement toggleable, send a patch upstream
            //led.toggle().unwrap();
        }

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }
}

mod active_low_switch {
    use super::*;
    use switch_hal::output::{ActiveLowOutputSwitch, OutputSwitch};

    #[test]
    fn when_on_pin_is_low() {
        let expectations = [Transaction::set(State::Low)];
        let pin = Mock::new(&expectations);

        let mut led = ActiveLowOutputSwitch::new(pin);
        led.on().unwrap();

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }

    #[test]
    fn when_off_pin_is_high() {
        let expectations = [Transaction::set(State::High)];
        let pin = Mock::new(&expectations);

        let mut led = ActiveLowOutputSwitch::new(pin);
        led.off().unwrap();

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }

    #[ignore] //https://github.com/dbrgn/embedded-hal-mock/issues/30
    fn is_toggleable() {
        use switch_hal::output::ToggleableOutputSwitch;

        let expectations = [
            Transaction::set(State::Low),
            Transaction::set(State::High),
            Transaction::set(State::Low),
            Transaction::set(State::High),
        ];
        let pin = Mock::new(&expectations);

        let mut led = ActiveLowOutputSwitch::new(pin);
        led.off().unwrap();

        for i in 0..3 {
            //TODO: mock_hal doesn't implement toggleable, send a patch upstream
            //led.toggle().unwrap();
        }

        // retrieve the pin so we can assert the expectations
        let mut pin = led.into_pin();
        pin.done();
    }
}
