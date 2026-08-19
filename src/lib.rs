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

/// provide logging primitives
use defmt_or_log::*;

/// provide hardware interface
pub mod interface;

/// logical protocol commands
mod commands;

#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
pub struct Ssd1680<DI> {
    interface: DI,
    size: embedded_graphics::geometry::Size,
    rotation: DisplayRotation,
}
#[maybe_async_cfg::maybe(
    sync(keep_self),
    async(
        feature = "async",
        idents(WriteOnlyDataCommand(async = "AsyncWriteOnlyDataCommand"))
    )
)]
impl<DI> Ssd1680<DI>
where DI: WriteOnlyDataCommand
 {
    pub fn new(
        interface: DI,
        size: embedded_graphics::geometry::Size,
        rotation: DisplayRotation,
    ) -> Result<Self, DisplayError> {
        let mut this = Self {
            interface,
            size,
            rotation,
        };

        this.init()?;

        Ok(this)
    }

    pub fn init(&mut self) -> Result<(), DisplayError> {
        // hardware reset
        // self.spi_interface.reset();

        // soft reset
        // Commands::SoftReset;
        // self.spi_interface.wait_until_idle();

        // configure display height
        // Commnds::DriverOutputControl{height: self.size().height};



        // TODO
        Err(DisplayError::DataFormatNotImplemented)
    }

    fn draw(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), DisplayError> {
        // FIXME
        let size = self.size;
        let Pixel(point, color) = pixel;
        if (point.x < 0)
            || (point.y < 0)
            || (point.x as u32 > size.width)
            || (point.y as u32 > size.height)
        {
            // pixel outside of screen
            return Ok(());
        }

        // TODO
        Err(DisplayError::DataFormatNotImplemented)
    }
}

use display_interface::{WriteOnlyDataCommand, AsyncWriteOnlyDataCommand, DisplayError};
use embedded_graphics::Pixel;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

impl<DI> OriginDimensions for Ssd1680<DI> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return match self.rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => self.size,

            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                embedded_graphics::geometry::Size::new(self.size.height, self.size.width)
            }
        };
    }
}

impl<DI> DrawTarget for Ssd1680<DI>
where DI: WriteOnlyDataCommand
{
    type Color = BinaryColor;

    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    {
        for p in pixels.into_iter() {
            self.draw(p)?;
        }
        Ok(())
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
