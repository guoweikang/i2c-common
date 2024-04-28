//! I2c common module
//!
//! Compatible with different hardware platforms
//! Include:
//! timing: I2C timing config

#![no_std]

#[macro_use]
extern crate derive_builder;

/// i2c operation mode
pub enum I2cMode {
    /// Master Mode.      
    ///
    ///A master in an I2C system and programmed only as a Master
    Master = 0,
    /// Slave Mode
    ///
    ///A slave in an I2C system and programmed only as a Slave
    Slave = 1,
}

/// i2c Speed mode
pub enum I2cSpeedMode {
    /// Standard Speed Mode.
    StandMode = 0,
    /// Fast Speed Mode.
    FastMode,
    /// Fast Plus Mode.
    FastPlusMode,
    /// TURBO Mode.
    TurboMode,
    /// High Speed.
    HighSpeedMode,
    /// ULTRA_FAST.
    UltraFastMode,
}

/// i2c timing
mod timing;
/// i2c func
mod functionality;

pub use crate::{
    timing::*,functionality::*,
};
