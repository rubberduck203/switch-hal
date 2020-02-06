//! Mock implementations of [InputPin](embedded_hal::digital::v2::InputPin) and [OutputPin](embedded_hal::digital::v2::OutputPin).
//!
//! WARNING: May be removed if `embedded_hal_mock` crate is improved.
//! https://github.com/dbrgn/embedded-hal-mock/issues/30
//!
//! This is part of the main crate so it is accessible to doctests.
//! Otherwise, I would have created a tests/mock/mod.rs file.
use embedded_hal::digital::v2::toggleable;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Low,
    High,
}

pub struct Pin {
    state: Option<State>,
}

impl Pin {
    pub fn new() -> Self {
        Pin { state: None }
    }

    pub fn with_state(state: State) -> Self {
        Pin { state: Some(state) }
    }
}

type MockError = &'static str;

impl InputPin for Pin {
    type Error = MockError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        match self.state {
            Some(State::High) => Ok(true),
            Some(State::Low) => Ok(false),
            None => Err("state not set"),
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        match self.is_high() {
            Ok(v) => Ok(!v),
            Err(e) => Err(e),
        }
    }
}

impl OutputPin for Pin {
    type Error = MockError;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = Some(State::Low);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = Some(State::High);
        Ok(())
    }
}

impl StatefulOutputPin for Pin {
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        self.is_low()
    }

    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_high()
    }
}

impl toggleable::Default for Pin {}

#[cfg(test)]
mod test {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn state_is_unitialized() {
            let pin = Pin::new();
            assert_eq!(None, pin.state);
            pin.is_low().expect_err("Expected unitialized pin");
        }
    }

    mod input_pin {
        use super::*;

        #[test]
        fn error_when_uninitialized() {
            let pin = Pin { state: None };
            pin.is_high().expect_err("Expected unitialized pin");
        }

        mod is_high {
            use super::*;

            #[test]
            fn returns_true_when_state_is_high() {
                let pin = Pin::with_state(State::High);
                assert_eq!(true, pin.is_high().unwrap());
            }

            #[test]
            fn returns_false_when_state_is_low() {
                let pin = Pin::with_state(State::Low);
                assert_eq!(false, pin.is_high().unwrap());
            }
        }

        mod is_low {
            use super::*;

            #[test]
            fn returns_false_when_state_is_high() {
                let pin = Pin::with_state(State::High);
                assert_eq!(false, pin.is_low().unwrap());
            }

            #[test]
            fn returns_true_when_state_is_high() {
                let pin = Pin::with_state(State::Low);
                assert_eq!(true, pin.is_low().unwrap());
            }
        }
    }

    mod output_pin {
        use super::*;

        #[test]
        fn set_low() {
            let mut pin = Pin::new();
            pin.set_low().unwrap();

            assert_eq!(true, pin.is_low().unwrap());
        }

        #[test]
        fn set_high() {
            let mut pin = Pin::new();
            pin.set_high().unwrap();

            assert_eq!(true, pin.is_high().unwrap());
        }
    }

    mod statful_output_pin {
        use super::*;

        #[test]
        fn error_when_uninitialized() {
            let pin = Pin { state: None };
            pin.is_set_high().expect_err("Expected unitialized pin");
        }

        mod is_set_low {
            use super::*;

            #[test]
            fn returns_false_when_state_is_high() {
                let pin = Pin::with_state(State::High);
                assert_eq!(false, pin.is_set_low().unwrap());
            }

            #[test]
            fn returns_true_when_state_is_high() {
                let pin = Pin::with_state(State::Low);
                assert_eq!(true, pin.is_set_low().unwrap());
            }
        }

        mod is_set_high {
            use super::*;

            #[test]
            fn returns_true_when_state_is_high() {
                let pin = Pin::with_state(State::High);
                assert_eq!(true, pin.is_set_high().unwrap());
            }

            #[test]
            fn returns_false_when_state_is_low() {
                let pin = Pin::with_state(State::Low);
                assert_eq!(false, pin.is_set_high().unwrap());
            }
        }

        mod toggleable {
            use super::*;
            use embedded_hal::digital::v2::ToggleableOutputPin;

            #[test]
            fn default_toggleable_impl() {
                let mut pin = Pin::with_state(State::Low);
                pin.toggle().unwrap();
                assert_eq!(true, pin.is_set_high().unwrap());
            }
        }
    }
}
