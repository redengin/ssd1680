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

/// logical protocol commands
mod commands;


#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
pub struct Ssd1680<DI> {
    spi_interface: DI,
    size: embedded_graphics::geometry::Size,
    rotation: DisplayRotation,
}
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(feature = "async",
        idents(
            WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand")
        )
    )
)]
impl<DI> Ssd1680<DI>
{
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

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use embedded_graphics::geometry::OriginDimensions;
use display_interface::DisplayError;

impl <DI> OriginDimensions for Ssd1680<DI> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return match self.rotation {
            DisplayRotation::Rotate0 |
            DisplayRotation::Rotate180 => self.size,

            DisplayRotation::Rotate90 |
            DisplayRotation::Rotate270 =>
                embedded_graphics::geometry::Size::new(self.size.height, self.size.width)
        }
    }
}


impl <DI> DrawTarget for Ssd1680<DI> {
    type Color = BinaryColor;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>> {
        todo!()
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