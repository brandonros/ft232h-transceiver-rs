mod bitfields;
mod logger;
mod nrf24;

use std::error::Error;

use embedded_hal::spi::Polarity;
use nrf24::*;

fn main() -> Result<(), Box<dyn Error>> {
    logger::init_stdout_logger();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    return runtime.block_on(async {
        let device: libftd2xx::Ft232h = libftd2xx::Ftdi::new()?.try_into()?;
        let hal = ftdi_embedded_hal::FtHal::init_freq(device, 3_000_000)?;
        let mut ce_pin = hal.ad4()?;
        //let mut led_pin = None; // C8 + C9 can't be controlled via GPIO?
        let mut spi_device = hal.spi_device(3)?; // TODO: 0 1 2 or 3?
        spi_device.set_clock_polarity(Polarity::IdleLow);
        let nrf24_role = Nrf24Role::Receiver;
        nrf24_setup(&mut &spi_device, &mut ce_pin, &nrf24_role).await;
        match nrf24_role {
            Nrf24Role::Transmitter => {
                nrf2401_transmitter_loop(&mut &spi_device, &mut ce_pin).await;
            }
            Nrf24Role::Receiver => {
                nrf2401_receiver_loop(&mut &spi_device);
            }
        }
        Ok(())
    });
}
