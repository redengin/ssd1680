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

    // FOLLOWING IS CONFIGURED FOR HELTEC WIRELESS PAPER board

    // configure the SPI SOC peripheral
    let spi = esp_hal::spi::master::Spi::new(
        peripherals.SPI3,
        // default: Mode 0  @ 1 MHz
        esp_hal::spi::master::Config::default(),
    )
    .unwrap()
    .with_cs(peripherals.GPIO4)
    .with_sck(peripherals.GPIO3)
    .with_mosi(peripherals.GPIO2)
    // .into_async()
    ;

    // create SPIInterface
    // use display_interface_spi::SPIInterface;
    // let spi_interface = SPIInterface::new(
    //     spi,
    //     esp_hal::gpio::Output::new(
    //         peripherals.GPIO5,
    //         esp_hal::gpio::Level::Low,
    //         esp_hal::gpio::OutputConfig::default(),
    //     ),
    // );

    // create the driver
    // use epd_ssd1680::interface::DisplayInterface;
    // let interface = DisplayInterface::new(
    //     spi_interface,
    //     esp_hal::gpio::Input::new(peripherals.GPIO7, esp_hal::gpio::InputConfig::default()),
    //     esp_hal::gpio::Output::new(
    //         peripherals.GPIO6,
    //         esp_hal::gpio::Level::Low,
    //         esp_hal::gpio::OutputConfig::default(),
    //     ),
    // );
    // let spi = esp_hal::spi::master::Spi::new(
    //     peripherals.SPI3,
    //     esp_hal::spi::master::Config::default()
    // );
    // let ssd1680_interface =

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
