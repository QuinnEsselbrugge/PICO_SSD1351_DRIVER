use embedded_hal::{digital::OutputPin, spi::SpiBus};
use rp_pico::{
    hal::{
        self,
        gpio::{bank0::*, FunctionSioOutput, FunctionSpi, Pin, PullDown, PullNone},
    },
    pac::SPI0,
};

use alloc::{vec, vec::Vec};

use rp_pico::hal::spi;

use self::hal::pac;
use self::hal::pac::RESETS;

use fugit::{HertzU32, RateExtU32};

pub struct SPIDisplayInterface {
    pub spi: hal::Spi<
        spi::Enabled,
        SPI0,
        (
            Pin<Gpio3, FunctionSpi, PullNone>,
            Pin<Gpio2, FunctionSpi, PullNone>,
        ),
    >,
    pub pins: SPIDriverPins,
}

// if you are the inventor of "DynPinID" and replaced "DynPin" count your fucking days im coming for you
pub struct SPIPins {
    pub spi_sclk: Pin<Gpio2, FunctionSpi, PullNone>,
    pub spi_mosi: Pin<Gpio3, FunctionSpi, PullNone>,
}

pub struct SPIDriverPins {
    pub cs: Pin<Gpio6, FunctionSioOutput, PullDown>,
    pub dc: Pin<Gpio7, FunctionSioOutput, PullDown>,
    pub rst: Pin<Gpio8, FunctionSioOutput, PullDown>,
}

impl SPIDisplayInterface {
    pub fn new(
        spi_pins: SPIPins,
        spi_driver_pins: SPIDriverPins,
        device: pac::SPI0,
        resets: &mut RESETS,
        freq: &mut HertzU32,
    ) -> SPIDisplayInterface {
        let spi = spi::Spi::<_, _, _, 8>::new(device, (spi_pins.spi_mosi, spi_pins.spi_sclk));

        let spi = spi.init(
            resets,
            *freq,
            50.MHz(),                       // max value SDD1351
            &embedded_hal::spi::MODE_3, // Data sampled on the rising edge and shifted out on the falling edge
        );

        SPIDisplayInterface {
            spi: spi,
            pins: spi_driver_pins,
        }
    }

    pub fn reset_interface(&mut self, delay: &mut cortex_m::delay::Delay) {
        let _ = &mut self.pins.rst.set_high().unwrap();

        delay.delay_ms(100);

        let _ = &mut self.pins.rst.set_low().unwrap();

        delay.delay_ms(100);

        let _ = &mut self.pins.rst.set_high().unwrap();

        delay.delay_ms(100);
    }

    pub fn transfer_command_vec(&mut self, command: u8, vals: Vec<u8>) {
        self.set_command(&[command]);

        for val in vals {
            self.transfer(&[val])
        }
    }

    pub fn transfer(&mut self, val: &[u8]) {
        let _ = &mut self.pins.dc.set_high().unwrap();
        let _ = &mut self.pins.cs.set_low().unwrap();
        let _ = &mut self.spi.write(val);
        let _ = &mut self.pins.cs.set_high().unwrap();
    }

    pub fn set_command(&mut self, val: &[u8]) {
        let _ = &mut self.pins.dc.set_low().unwrap();
        let _ = &mut self.pins.cs.set_low().unwrap();
        let _ = &mut self.spi.write(val);
        let _ = &mut self.pins.cs.set_high().unwrap();
    }
}
