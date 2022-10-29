use core::fmt::Debug;

use embedded_websocket::framer::Stream;
use wifinina::Error as WifiError;

use crate::hw::{spi::SocketPhysical, HalSpiError};

#[derive(Debug)]
pub enum NetworkError<SpiError: Debug> {
    Io(WifiError<SpiError>),
    Closed,
    SocketStatusNone,
}

impl<SpiError> From<WifiError<SpiError>> for NetworkError<SpiError>
where
    SpiError: Debug,
{
    fn from(err: WifiError<SpiError>) -> NetworkError<SpiError> {
        NetworkError::Io(err)
    }
}

pub struct TcpStream<'a, D>
where
    D: embedded_hal::blocking::delay::DelayMs<u16>,
{
    socket: SocketPhysical<'a, D>,
}

impl<'a, D> TcpStream<'a, D>
where
    D: embedded_hal::blocking::delay::DelayMs<u16>,
{
    pub fn new(socket: SocketPhysical<'a, D>) -> Self {
        Self { socket }
    }
}

impl<'a, D> Stream<NetworkError<HalSpiError>> for TcpStream<'a, D>
where
    D: embedded_hal::blocking::delay::DelayMs<u16>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetworkError<HalSpiError>> {
        //rprintln!("[INF] Read: Waiting for bytes");

        loop {
            match self.socket.read(buf) {
                Ok(len) => return Ok(len),
                Err(e) => match e {
                    nb::Error::Other(e) => match e {
                        WifiError::SocketClosed => return Err(NetworkError::Closed),
                        _ => return Err(NetworkError::Io(e)),
                    },
                    nb::Error::WouldBlock => {
                        cortex_m::asm::delay(1_000_000);
                    }
                },
            };
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), NetworkError<HalSpiError>> {
        let mut start = 0;
        //rprintln!("[INF] Write: Sending {} bytes", buf.len());

        loop {
            let bytes_sent = self.socket.write(&buf[start..])?;
            start += bytes_sent;
            //rprintln!("[INF] Write: Sent {} bytes", bytes_sent);

            if start == buf.len() {
                return Ok(());
            }
        }
    }
}
