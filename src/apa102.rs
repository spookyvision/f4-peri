use apa102_spi::Apa102;
use hal::{
    gpio::Alternate,
    pac::{SPI1, SPI2},
    spi::{NoMiso, Spi},
};
use stm32f4xx_hal::{self as hal, gpio::Pin};

mod spi {
    use super::*;
    pub type ApaSpi = Spi<SPI1>;
    pub type Pins = (
        Pin<'B', 3_u8, Alternate<5_u8>>,
        NoMiso,
        Pin<'B', 5_u8, Alternate<5_u8>>,
    );
}

mod spi_alt {
    use super::*;
    pub type ApaSpi = Spi<SPI2>;
    pub type Pins = (
        Pin<'B', 13_u8, Alternate<5_u8>>,
        NoMiso,
        Pin<'B', 15_u8, Alternate<5_u8>>,
    );
}
#[cfg(feature = "spi")]
pub use spi::*;
#[cfg(all(not(feature = "spi"), feature = "spi_alt"))]
pub use spi_alt::*;

pub type APA = Apa102<ApaSpi>;
