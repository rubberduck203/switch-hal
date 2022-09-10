#![no_std]

mod input;
mod output;

pub mod mock;

/// Represents an input switch, such as a button or a switch
pub trait InputSwitch {
    type Error;

    /// Returns true if the switch has been activated, otherwise false
    /// i.e. if a button is currently pressed, returns true
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{InputSwitch, OutputSwitch, Switch, IntoSwitch};
    /// # let pin = mock::Pin::with_state(mock::State::High);
    /// # let mut status_led = mock::Pin::new().into_active_high_switch();
    /// let button = pin.into_active_low_switch();
    /// match button.is_active() {
    ///     Ok(true) => { status_led.on().ok(); }
    ///     Ok(false) => { status_led.off().ok(); }
    ///     Err(_) => { panic!("Failed to read button state"); }
    /// }
    /// ```
    fn is_active(&self) -> Result<bool, Self::Error>;
}

/// Represents an output switch, such as a LED "switch" or transistor
pub trait OutputSwitch {
    type Error;

    /// Turns the switch on
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{OutputSwitch, Switch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
    /// led.on().ok();
    /// ```
    fn on(&mut self) -> Result<(), Self::Error>;

    /// Turns the switch off
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{OutputSwitch, Switch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
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
    /// use switch_hal::{OutputSwitch, ToggleableOutputSwitch, Switch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
    /// led.toggle().ok();
    /// ```
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Checks current switch state
///
/// # Notes
/// This is only available if the underlying hal has implemented [StatefulOutputPin](embedded_hal::digital::v2::StatefulOutputPin)
pub trait StatefulOutputSwitch {
    type Error;

    /// Checks whether the switch is on
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{OutputSwitch, Switch, StatefulOutputSwitch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
    /// led.off().ok();
    /// assert!(!led.is_on().unwrap());
    /// ```
    fn is_on(&mut self) -> Result<bool, Self::Error>;

    /// Checks whether the switch is off
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{OutputSwitch, Switch, StatefulOutputSwitch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
    /// led.off().ok();
    /// assert!(led.is_off().unwrap());
    /// ```
    fn is_off(&mut self) -> Result<bool, Self::Error>;
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
    /// Constructs a new [Switch](struct.Switch.html) from a concrete implementation of an
    /// [InputPin](embedded_hal::digital::v2::InputPin) or [OutputPin](embedded_hal::digital::v2::OutputPin)
    ///
    /// **Prefer the [IntoSwitch](trait.IntoSwitch.html) trait over calling [new](#method.new) directly.**
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

    /// Consumes the [Switch](struct.Switch.html) and returns the underlying [InputPin](embedded_hal::digital::v2::InputPin) or [OutputPin](embedded_hal::digital::v2::OutputPin).
    ///
    /// This is useful fore retrieving the underlying pin to use it for a different purpose.
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{OutputSwitch, Switch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let mut led = pin.into_active_high_switch();
    /// led.on().ok();
    /// let mut pin = led.into_pin();
    /// // do something else with the pin
    /// ```
    pub fn into_pin(self) -> IoPin {
        self.pin
    }
}

/// Convenience functions for converting [InputPin](embedded_hal::digital::v2::InputPin)
/// and [OutputPin](embedded_hal::digital::v2::OutputPin) to a [Switch](struct.Switch.html).
///
/// The type of [Switch](struct.Switch.html) returned,
/// [InputSwitch](trait.InputSwitch.html) or [OutputSwitch](trait.OutputSwitch.html) is
/// determined by whether the `IoPin` being consumed is an [InputPin](embedded_hal::digital::v2::InputPin)
/// or [OutputPin](embedded_hal::digital::v2::OutputPin).
pub trait IntoSwitch {

    /// Consumes the `IoPin` returning a [Switch](struct.Switch.html) of the appropriate `ActiveLevel`.
    ///
    /// This method exists so other, more convenient functions, can have blanket implementations.
    /// Prefer [into_active_low_switch](#method.into_active_low_switch) and [into_active_high_switch](#method.into_active_high_switch).
    ///
    /// # Examples
    ///
    /// ## Active High
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveHigh, OutputSwitch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let led = pin.into_switch::<ActiveHigh>();
    /// ```
    ///
    /// ## Active Low
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::{ActiveLow, InputSwitch, IntoSwitch};
    /// # let pin = mock::Pin::new();
    /// let button = pin.into_switch::<ActiveLow>();
    /// ```
    fn into_switch<ActiveLevel>(self) -> Switch<Self, ActiveLevel>
    where
        Self: core::marker::Sized;

    /// Consumes the `IoPin` returning a `Switch<IoPin, ActiveLow>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::IntoSwitch;
    /// # let pin = mock::Pin::new();
    /// let led = pin.into_active_low_switch();
    /// ```
    fn into_active_low_switch(self) -> Switch<Self, ActiveLow>
    where
        Self: core::marker::Sized,
    {
        self.into_switch::<ActiveLow>()
    }

    /// Consumes the `IoPin` returning a `Switch<IoPin, ActiveHigh>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::IntoSwitch;
    /// # let pin = mock::Pin::new();
    /// let button = pin.into_active_high_switch();
    /// ```
    fn into_active_high_switch(self) -> Switch<Self, ActiveHigh>
    where
        Self: core::marker::Sized,
    {
        self.into_switch::<ActiveHigh>()
    }
}

impl<T> IntoSwitch for T {
    fn into_switch<ActiveLevel>(self) -> Switch<Self, ActiveLevel> {
        Switch::<Self, ActiveLevel>::new(self)
    }
}
