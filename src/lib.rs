#![no_std]

mod input;
mod output;

pub mod mock;

/// Represents an input switch, such as a button or a switch
pub trait InputSwitch {
    type Error;

    /// Returns true if the swich has been activated, otherwise false
    /// i.e. if a button is currently pressed, returns true
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveLow, InputSwitch, OutputSwitch, Switch};
    /// # let pin = mock::Pin::with_state(mock::State::High);
    /// # let mut status_led = Switch::<_, ActiveLow>::new(mock::Pin::new());
    /// let button = Switch::<_, ActiveLow>::new(pin);
    /// match button.is_active() {
    ///     Ok(true) => { status_led.on().ok(); }
    ///     Ok(false) => { status_led.off().ok(); }
    ///     Err(_) => { panic!("Failed to read button state"); }
    /// }
    /// ```
    fn is_active(&self) -> Result<bool, Self::Error>;
}

/// Represents an output switch, such as a LED "switch" or transitor
pub trait OutputSwitch {
    type Error;

    /// Turns the switch on
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// led.on().ok();
    /// ```
    fn on(&mut self) -> Result<(), Self::Error>;

    /// Turns the switch off
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
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
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, ToggleableOutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// led.toggle().ok();
    /// ```
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Zero sized struct for signaling to [Switch](struct.Switch.html) that it is active high
pub struct ActiveHigh;
/// Zero sized struct for signaling to [Switch](struct.Switch.html) that it is active low
pub struct ActiveLow;

use core::marker::PhantomData;

/// Concrete implementation for [InputSwitch](trait.InputSwitch.html) and [OutputSwitch](trait.OutputSwitch.html)
///
/// # Type Params
/// - `IoPin` must be a type that implements either of the [InputPin](embedded_hal::digital::v2::InputPin) or [OutputPin](embedded_hal::digital::v2::OutputPin) traits.
/// - `ActiveLevel` indicates whether the `Switch` is [ActiveHigh](ActiveHigh) or [ActiveLow](ActiveLow).
///     `ActiveLevel` is not actually stored in the struct.
///     It's [PhantomData](core::marker::PhantomData) used to indicate which implementation to use.
pub struct Switch<IoPin, ActiveLevel> {
    pin: IoPin,
    active: PhantomData<ActiveLevel>,
}

impl<IoPin, ActiveLevel> Switch<IoPin, ActiveLevel> {
    /// Constructs a new [Switch](Switch) from a concrete implementation of an
    /// [InputPin](embedded_hal::digital::v2::InputPin) or [OutputPin](embedded_hal::digital::v2::OutputPin)
    ///
    /// # Examples
    ///
    /// Active High
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// ```
    ///
    /// ActiveLow
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveLow, OutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveLow>::new(pin);
    /// ```
    ///
    /// stm32f3xx-hal
    ///
    /// ```ignore
    /// // Example for the stm32f303
    /// use stm32f3xx_hal::gpio::gpioe;
    /// use stm32f3xx_hal::gpio::{PushPull, Output};
    /// use stm32f3xx_hal::stm32;
    ///
    /// use switch_hal::{ActiveHigh, Switch};
    ///
    /// let device_periphs = stm32::Peripherals::take().unwrap();
    /// let gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);
    ///
    /// let led = Switch::<_, ActiveHigh>::new(
    ///     gpioe
    ///     .pe9
    ///     .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
    /// )
    /// ```
    pub fn new(pin: IoPin) -> Self {
        Switch {
            pin: pin,
            active: PhantomData::<ActiveLevel>,
        }
    }

    /// Consumes the [Switch](Switch) and returns the underlying [InputPin](embedded_hal::digital::v2::InputPin) or [OutputPin](embedded_hal::digital::v2::OutputPin).
    ///
    /// This is useful fore retrieving the underlying pin to use it for a different purpose.
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, Switch};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// led.on().ok();
    /// let mut pin = led.into_pin();
    /// // do something else with the pin
    /// ```
    pub fn into_pin(self) -> IoPin {
        self.pin
    }
}

use embedded_hal::digital::v2::{InputPin, OutputPin};

pub trait OutputPinExt {
    fn into_active_high_output(self) -> Switch<Self, ActiveHigh>
        where Self: core::marker::Sized + OutputPin;

    fn into_active_low_output(self) -> Switch<Self, ActiveLow>
        where Self: core::marker::Sized + OutputPin;
}

pub trait InputPinExt {
    fn into_active_high_input(self) -> Switch<Self, ActiveHigh>
        where Self: core::marker::Sized + InputPin;

    fn into_active_low_input(self) -> Switch<Self, ActiveLow>
        where Self: core::marker::Sized + InputPin;
}

impl<T: OutputPin> OutputPinExt for T {
    fn into_active_high_output(self) -> Switch<Self, ActiveHigh> {
        Switch::<Self, ActiveHigh>::new(self)
    }

    fn into_active_low_output(self) -> Switch<Self, ActiveLow> {
        Switch::<Self, ActiveLow>::new(self)
    }
}

impl<T: InputPin> InputPinExt for T {
    fn into_active_high_input(self) -> Switch<Self, ActiveHigh> {
        Switch::<Self, ActiveHigh>::new(self)
    }

    fn into_active_low_input(self) -> Switch<Self, ActiveLow> {
        Switch::<Self, ActiveLow>::new(self)
    }
}
