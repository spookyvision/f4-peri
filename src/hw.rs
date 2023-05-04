use hal::{
    gpio::{
        gpioa::{PA5, PA6, PA7},
        gpiob::{PB0, PB1, PB13, PB14, PB15},
        Output, PushPull,
    },
    pac::{Peripherals, SPI1, SPI2},
    spi::{Spi, TransferModeNormal},
};
use stm32f4xx_hal as hal;
use wifinina::{commands::socket::ConnectedSocket, WifiNina};

pub type HalSpiError = hal::spi::Error;

use numtoa::NumToA;
use utils::str::Uuid;

pub struct Uid<'a>(&'a stm32f4xx_hal::signature::Uid);

impl<'a> Uuid for Uid<'a> {
    type T = heapless::String<32>;
    type Error = ();
    fn uuid(&self) -> Result<Self::T, Self::Error> {
        let mut buf = [0u8; 4];
        let br = &mut buf;
        let mut s = heapless::String::<32>::new();
        // e.g.  "48"+"68"+"16"

        let inner = &self.0;
        s.push_str(inner.x().numtoa_str(10, br))?;
        s.push_str(inner.y().numtoa_str(10, br))?;
        s.push_str(inner.waf_num().numtoa_str(10, br))?;
        Ok(s)
    }
}

impl<'a> Uid<'a> {
    pub fn get() -> Self {
        Self(stm32f4xx_hal::signature::Uid::get())
    }
}

#[cfg(feature = "spi")]
pub mod spi {
    use super::*;
    pub type CsPin = PA5<Output<PushPull>>;
    pub type BusyPin = PA6<hal::gpio::Input>;
    pub type WifiPhysical<D> = WifiNina<CsPin, BusyPin, SpiPhysical, D>;
    pub type SocketPhysical<'a, D> =
        ConnectedSocket<'a, CsPin, BusyPin, SpiPhysical, HalSpiError, D>;

    pub type SpiPhysical = Spi<SPI2>;
    pub type Pins = (
        PB13<hal::gpio::Input>,
        PB14<hal::gpio::Input>,
        PB15<hal::gpio::Input>,
    );
}

#[cfg(all(not(feature = "spi"), feature = "spi_alt"))]
pub mod spi {
    use super::*;
    pub type CsPin = PB0<Output<PushPull>>;
    pub type BusyPin = PB1<hal::gpio::Input>;
    pub type WifiPhysical<D> = WifiNina<CsPin, BusyPin, SpiPhysical, D>;
    pub type SocketPhysical<'a, D> =
        ConnectedSocket<'a, CsPin, BusyPin, SpiPhysical, HalSpiError, D>;

    pub type SpiPhysical = Spi<SPI1>;
    pub type Pins = (
        PA5<hal::gpio::Input>,
        PA6<hal::gpio::Input>,
        PA7<hal::gpio::Input>,
    );
}
