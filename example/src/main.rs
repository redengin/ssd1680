#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::time::{Duration, Instant};
use esp_hal::{main, peripherals};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3 -o unstable-hal -o alloc

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::default());
    let peripherals = esp_hal::init(config);

    // create the heap space (reclaiming bootloader RAM)
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    // FOLLOWING IS CONFIGURED FOR HELTEC WIRELESS PAPER board
    // ------------------------------------------------------------------
    let sck = peripherals.GPIO3;
    let mosi = peripherals.GPIO2;
    let cs = peripherals.GPIO4;
    // ------------------------------------------------------------------
    let spi_bus =
        esp_hal::spi::master::Spi::new(peripherals.SPI3, esp_hal::spi::master::Config::default())
            .unwrap()
            .with_sck(sck)
            .with_mosi(mosi);
    let cs_pin = esp_hal::gpio::Output::new(
        cs,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi_bus, cs_pin).unwrap();
    use display_interface_spi::SPIInterface;
    let spi_interface = SPIInterface::new(
        spi_device,
        esp_hal::gpio::Output::new(
            peripherals.GPIO5,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
    );

    // create the driver object
    use epd_ssd1680::interface::DisplayInterface;
    let busy = esp_hal::gpio::Input::new(peripherals.GPIO7, esp_hal::gpio::InputConfig::default());
    let reset = esp_hal::gpio::Output::new(
        peripherals.GPIO6,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    let interface = DisplayInterface::new(spi_interface, busy, reset);


    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
