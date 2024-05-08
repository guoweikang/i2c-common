//! I2c common module
//!
//! Compatible with different hardware platforms
//! Include:
//! timing: I2C timing config

#![no_std]

#[macro_use]
extern crate derive_builder;

/// i2c operation mode
#[derive(Debug)]
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


/// i2c timing
mod timing;
/// i2c func
mod functionality;
/// i2c msg
pub mod msg;

pub use crate::{
    timing::*,functionality::*
};
