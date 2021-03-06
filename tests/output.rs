extern crate switch_hal;

use switch_hal::mock;

mod active_high_switch {
    use super::*;
    use embedded_hal::digital::v2::InputPin;
    use switch_hal::{ActiveHigh, OutputSwitch, Switch};

    #[test]
    fn when_on_pin_is_high() {
        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.on().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn when_off_pin_is_low() {
        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }

    #[test]
    fn is_toggleable() {
        use switch_hal::ToggleableOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        led.toggle().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn not_on_when_low() {
        use switch_hal::StatefulOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        assert_eq!(false, led.is_on().unwrap());

        let pin = led.into_pin();
        assert_eq!(false, pin.is_high().unwrap());
    }

    #[test]
    fn is_on_when_high() {
        use switch_hal::StatefulOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.on().unwrap();

        assert_eq!(true, led.is_on().unwrap());

        let pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }
}

mod active_low_switch {
    use super::*;
    use embedded_hal::digital::v2::InputPin;
    use switch_hal::{ActiveLow, OutputSwitch, Switch};

    #[test]
    fn when_on_pin_is_low() {
        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.on().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }

    #[test]
    fn when_off_pin_is_high() {
        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn is_toggleable() {
        use switch_hal::ToggleableOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        led.toggle().unwrap();

        let pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }

    #[test]
    fn not_on_when_high() {
        use switch_hal::StatefulOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        assert_eq!(false, led.is_on().unwrap());

        let pin = led.into_pin();
        assert_eq!(false, pin.is_low().unwrap());
    }

    #[test]
    fn is_on_when_low() {
        use switch_hal::StatefulOutputSwitch;

        let pin = mock::Pin::new();

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.on().unwrap();

        assert_eq!(true, led.is_on().unwrap());

        let pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }
}
