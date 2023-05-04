use fugit::RateExtU32;
use hal::{
    gpio::{Alternate, Pin},
    spi::{NoMiso, NoSck, Spi, TransferModeNormal},
};
use stm32f4xx_hal::{self as hal};
use ws2812_spi::Ws2812;

#[cfg(feature = "spi")]
mod ws_hw {
    use super::*;
    pub type LedSpi = hal::pac::SPI1;
    pub type DataPin = Pin<'B', 5_u8, Alternate<5_u8>>;
}

#[cfg(all(not(feature = "spi"), feature = "spi_alt"))]
mod ws_hw {
    use super::*;
    pub type LedSpi = hal::pac::SPI2;
    pub type DataPin = Pin<'B', 15_u8, Alternate<5_u8>>;
}

use ws_hw::*;
pub type WsSpi = Spi<LedSpi>;

pub type WS = Ws2812<WsSpi>;
use hal::rcc::Clocks;

pub fn spi(spi: LedSpi, data_pin: DataPin, clocks: &Clocks) -> WsSpi {
    WsSpi::new(
        spi,
        (NoSck::new(), NoMiso::new(), data_pin),
        ws2812_spi::MODE,
        2400.kHz(),
        &clocks,
    )
}
