//! Driver for ssd1680 EPD display chip
//!
//! # Example
//!
//!
//!
//!
//!

// choose std or no_std environment
#![cfg_attr(not(feature = "std"), no_std)]

// /// provide logging primitives (and derive helpers)
// use defmt_or_log::*;

/// provide hardware interface
pub mod interface;

#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
pub struct Ssd1680<DI> {
    spi_interface: DI,
    size: embedded_graphics::geometry::Size,
    rotation: DisplayRotation,
}
impl<DI> Ssd1680<DI> {
    pub fn new(
        spi_interface: DI,
        size: embedded_graphics::geometry::Size,
        rotation: DisplayRotation,
    ) -> Self {
        Self {
            spi_interface, 
            size,
            rotation,
        }
    }
}



/// Display rotation.
// #[derive(Copy, Clone, Debug)]
pub enum DisplayRotation {
    /// No rotation, normal display
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}