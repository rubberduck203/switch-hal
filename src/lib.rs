#![no_std]

pub mod output;
pub mod input;

pub mod mock;

/// Zero sized struct for signaling to [Switch](Switch) that it is active high
pub struct ActiveHigh;
/// Zero sized struct for signaling to [Switch](Switch) that it is active low
pub struct ActiveLow;

