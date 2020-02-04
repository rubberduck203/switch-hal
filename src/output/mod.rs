use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

/// Represents a logical output switch, such as a LED "switch" or transitor
pub trait OutputSwitch {
    type Error;

    /// Turns the switch on
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use embedded_hal_mock::pin::{Mock, State, Transaction};
    /// # use switch_hal::output::{OutputSwitch, ActiveHighOutputSwitch};
    /// # let expectations = [Transaction::set(State::High)];
    /// # let pin = Mock::new(&expectations);
    /// let mut led = ActiveHighOutputSwitch::new(pin);
    /// led.on().ok();
    /// ```
    fn on(&mut self) -> Result<(), Self::Error>;

    /// Turns the switch off
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use embedded_hal_mock::pin::{Mock, State, Transaction};
    /// # use switch_hal::output::{OutputSwitch, ActiveHighOutputSwitch};
    /// # let expectations = [Transaction::set(State::Low)];
    /// # let pin = Mock::new(&expectations);
    /// let mut led = ActiveHighOutputSwitch::new(pin);
    /// led.off().ok();
    /// ```
    fn off(&mut self) -> Result<(), Self::Error>;
}

/// Toggles the switch from it's current state to it's opposite state.
/// 
/// # Notes
/// This is only available if the underlying hal has implemented [ToggleableOutputPin](embedded_hal::digital::v2::ToggleableOutputPin)
pub trait ToggleableOutputSwitch {
    type Error;

    /// Toggles the current state of the [OutputSwitch](OutputSwitch)
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// # use embedded_hal_mock::pin::{Mock, State, Transaction};
    /// # use switch_hal::output::{OutputSwitch, ToggleableOutputSwitch, ActiveHighOutputSwitch};
    /// # let pin = Mock::new(&[]);
    /// # let mut led = ActiveHighOutputSwitch::new(pin);
    /// led.toggle().ok();
    /// ```
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Represents an active high switch, such as an active high LED or N-FET.
pub struct ActiveHighOutputSwitch<T>
where
    T: OutputPin,
{
    pin: T,
}

impl<T: OutputPin> ActiveHighOutputSwitch<T> {

    /// Constructs a new [ActiveHighOutputSwitch](ActiveHighOutputSwitch) from a concrete implementation of an [OutputPin](embedded_hal::digital::v2::OutputPin).
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// // Example for the stm32f303
    /// use stm32f3xx_hal::gpio::gpioe;
    /// use stm32f3xx_hal::gpio::{PushPull, Output};
    /// use stm32f3xx_hal::stm32;
    /// 
    /// let device_periphs = stm32::Peripherals::take().unwrap();
    /// let gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);
    /// 
    /// let led = ld3: ActiveHighLed::new(
    ///     gpioe
    ///     .pe9
    ///     .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
    /// )
    /// ```
    pub fn new(pin: T) -> Self {
        ActiveHighOutputSwitch { pin: pin }
    }

    /// Consumes the [ActiveHighOutputPin](ActiveHighOutputPin) and returns the underlying [OutputPin](embedded_hal::digital::v2::OutputPin).
    /// 
    /// This is useful fore retrieving the underlying pin to use it for a different purpose.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use embedded_hal_mock::pin::{Mock, State, Transaction};
    /// # use switch_hal::output::{OutputSwitch, ActiveHighOutputSwitch};
    /// # let expectations = [Transaction::set(State::High)];
    /// # let pin = Mock::new(&expectations);
    /// let mut led = ActiveHighOutputSwitch::new(pin);
    /// led.on().ok();
    /// let mut pin = led.into_pin();
    /// // do something else with the pin
    /// ```
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

pub struct ActiveLowOutputSwitch<T>
where
    T: OutputPin,
{
    pin: T,
}

impl<T: OutputPin> ActiveLowOutputSwitch<T> {
    pub fn new(pin: T) -> Self {
        ActiveLowOutputSwitch { pin: pin }
    }

    pub fn into_pin(self) -> T {
        self.pin
    }
}

impl<T: OutputPin> OutputSwitch for ActiveLowOutputSwitch<T> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: OutputPin + ToggleableOutputPin> ToggleableOutputSwitch for ActiveLowOutputSwitch<T> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}