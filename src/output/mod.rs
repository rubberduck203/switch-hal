use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

/// Represents a logical output switch, such as a LED "switch" or transitor
pub trait OutputSwitch {
    type Error;

    /// Turns the switch on
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::output::{OutputSwitch, Switch, ActiveHigh};
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
    /// use switch_hal::output::{OutputSwitch, Switch, ActiveHigh};
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
    /// use switch_hal::output::{ToggleableOutputSwitch, OutputSwitch, Switch, ActiveHigh};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// led.toggle().ok();
    /// ```
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

use core::marker::PhantomData;

/// Zero sized struct for signaling to [Switch](Switch) that it is active high
pub struct ActiveHigh;
/// Zero sized struct for signaling to [Switch](Switch) that it is active low
pub struct ActiveLow;

/// Concrete implementation of [OutputSwitch](OutputSwitch)
pub struct Switch<T, Activeness>
where
    T: OutputPin,
{
    pin: T,
    active: PhantomData<Activeness>,
}

impl<T: OutputPin, Activeness> Switch<T, Activeness> {
    /// Constructs a new [Switch](Switch) from a concrete implementation of an [OutputPin](embedded_hal::digital::v2::OutputPin).
    /// 
    /// # Examples
    ///
    /// Active High
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::output::{OutputSwitch, Switch, ActiveHigh};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// ```
    ///
    /// ActiveLow
    ///
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::output::{OutputSwitch, Switch, ActiveLow};
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
    /// use switch_hal::output::{Switch, ActiveHigh};
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
    pub fn new(pin: T) -> Self {
        Switch {
            pin: pin,
            active: PhantomData::<Activeness>,
        }
    }

    /// Consumes the [Switch](Switch) and returns the underlying [OutputPin](embedded_hal::digital::v2::OutputPin).
    /// 
    /// This is useful fore retrieving the underlying pin to use it for a different purpose.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use switch_hal::mock;
    /// use switch_hal::output::{OutputSwitch, Switch, ActiveHigh};
    /// # let pin = mock::Pin::new();
    /// let mut led = Switch::<_, ActiveHigh>::new(pin);
    /// led.on().ok();
    /// let mut pin = led.into_pin();
    /// // do something else with the pin
    /// ```
    pub fn into_pin(self) -> T {
        self.pin
    }
}

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveLow> {
    type Error = <T as OutputPin>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: OutputPin + ToggleableOutputPin, Activeness> ToggleableOutputSwitch for Switch<T, Activeness> {
    type Error = <T as ToggleableOutputPin>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}
