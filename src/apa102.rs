use apa102_spi::Apa102;
use hal::{
    gpio::{Alternate, PushPull},
    pac::{SPI1, SPI2},
    spi::{NoMiso, Spi, TransferModeNormal},
};
use stm32f4xx_hal::{self as hal, gpio::Pin};

#[cfg(feature = "spi")]
pub type ApaSpi = Spi<
    SPI1,
    (
        Pin<'B', 3_u8, Alternate<5_u8>>,
        NoMiso,
        Pin<'B', 5_u8, Alternate<5_u8>>,
    ),
    TransferModeNormal,
>;

#[cfg(all(not(feature = "spi"), feature = "spi_alt"))]
pub type ApaSpi = Spi<
    SPI2,
    (
        Pin<'B', 13_u8, Alternate<5_u8>>,
        NoMiso,
        Pin<'B', 15_u8, Alternate<5_u8>>,
    ),
    TransferModeNormal,
>;

pub type APA = Apa102<ApaSpi>;
