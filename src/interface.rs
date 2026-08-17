/// use standard Display Errors
// use display_interface::DisplayError;
/// use standard Display SPI interface
use display_interface_spi::SPIInterface;

// provide embedded_hal abstractions
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice as SpiDeviceAsync;


#[maybe_async_cfg::maybe(sync(keep_self), async(feature = "async"))]
pub struct Ssd1680Interface<SPI, DC, BUSY, RESET> {
    /// SPI device
    spi_interface: SPIInterface<SPI, DC>,
    /// Low for busy, Wait until display is ready!
    busy: BUSY,
    /// Pin for Reseting
    reset: RESET,
}

#[maybe_async_cfg::maybe(
    idents(SpiDevice(sync, async = "SpiDeviceAsync")),
    sync(keep_self),
    async(feature = "async")
)]
impl<SPI, DC, BUSY, RESET> Ssd1680Interface<SPI, DC, BUSY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    RESET: OutputPin,
{
    pub fn new(
        spi_interface: SPIInterface<SPI, DC>,
        busy: BUSY,
        reset: RESET,
        delay: &mut impl DelayNs,
    ) -> Self {
        let mut this = Self {
            spi_interface,
            busy,
            reset,
        };
        this.reset(delay);

        this
    }

    /// Resets the device.
    pub fn reset(&mut self, delay: &mut impl DelayNs) {
        self.reset.set_high().unwrap();
        delay.delay_ms(100);
        self.reset.set_low().unwrap();
        delay.delay_ms(100);
        self.reset.set_high().unwrap();
    }
}
