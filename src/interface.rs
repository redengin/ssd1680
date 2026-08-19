/// use standard Display Errors
use display_interface::DisplayError;
/// use standard Display SPI interface
use display_interface_spi::SPIInterface;


pub struct Ssd1680Interface<SPI, DC, BUSY, RESET> {
    /// SPI device
    spi_interface: SPIInterface<SPI, DC>,
    /// Low for busy, Wait until display is ready!
    busy: BUSY,
    /// Pin for Reseting
    reset: RESET,
}

// provide embedded_hal abstractions
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::delay::DelayNs;

#[cfg(not(feature = "async"))]
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

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
        delay.delay_ms(10);
        self.reset.set_low().unwrap();
        delay.delay_ms(10);
        self.reset.set_high().unwrap();
    }
}

#[cfg(not(feature = "async"))]
use display_interface::WriteOnlyDataCommand;
#[cfg(feature = "async")]
use display_interface::AsyncWriteOnlyDataCommand;

use display_interface::DataFormat;

#[maybe_async_cfg::maybe(
    idents(
        AsyncWriteOnlyDataCommand(sync = "WriteOnlyDataCommand",  async = "AsyncWriteOnlyDataCommand")
    ),
    // sync(keep_self),
    // async(keep_self, feature = "async")
)]
impl<SPI, DC, BUSY, RESET> AsyncWriteOnlyDataCommand for Ssd1680Interface<SPI, DC, BUSY, RESET>
where
    SPI: SpiDevice,
    DC: OutputPin,
    BUSY: InputPin,
    RESET: OutputPin,
{
    /// Send a batch of commands to display
    async fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError>
    {
        self.spi_interface.send_commands(cmd).await
    }

    /// Send pixel data to display
    async fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError>
    {
        self.spi_interface.send_data(buf).await
    }
}

