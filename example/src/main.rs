#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

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

    // CONFIGURATION - using HELTEC WIRELESS PAPER board
    // ------------------------------------------------------------------
    let sck_pin = peripherals.GPIO3;
    let mosi_pin = peripherals.GPIO2;
    let cs_pin = peripherals.GPIO4;
    let dc_pin = peripherals.GPIO5;
    let reset_pin = peripherals.GPIO6;
    let busy_pin = peripherals.GPIO7;
    // ------------------------------------------------------------------

    // create the SPI device on the SoC
    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(
        esp_hal::spi::master::Spi::new(peripherals.SPI3, esp_hal::spi::master::Config::default())
            .unwrap()
            .with_sck(sck_pin)
            .with_mosi(mosi_pin),
        esp_hal::gpio::Output::new(
            cs_pin,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
    )
    .unwrap();


    // provide driver for spi_device

    use display_interface_spi::SPIInterface;
    let dc = esp_hal::gpio::Output::new(
        dc_pin,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    let spi_interface = SPIInterface::new(spi_device, dc);

    // create the driver object
    let busy = esp_hal::gpio::Input::new(busy_pin, esp_hal::gpio::InputConfig::default());
    let reset = esp_hal::gpio::Output::new(
        reset_pin,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    let screen_interface = epd_ssd1680::interface::Ssd1680Interface::new(
        spi_interface,
        busy,
        reset,
        &mut esp_hal::delay::Delay::new(),
    );
    let ssd1680 = epd_ssd1680::Ssd1680::new(
        screen_interface,
        embedded_graphics::geometry::Size::new(122, 250),
        epd_ssd1680::DisplayRotation::Rotate0,
    );

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
