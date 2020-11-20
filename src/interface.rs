use core::cell::RefCell;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::FullDuplex;
use embedded_nal::Ipv4Addr;

use crate::bus::{ActiveBus, ActiveFourWire, FourWire};
use crate::device::Device;
use crate::network::{Manual, Network};
use crate::uninitialized_device::{InitializeError, UninitializedDevice};
use crate::{MacAddress, Mode};

pub struct Interface<SpiBus: ActiveBus, NetworkImpl: Network> {
    pub device: RefCell<Device<SpiBus, NetworkImpl>>,
}

impl<SpiBus: ActiveBus, NetworkImpl: Network> Interface<SpiBus, NetworkImpl> {
    fn new(device: Device<SpiBus, NetworkImpl>) -> Self {
        Self {
            device: RefCell::new(device),
        }
    }

    pub fn release(self) -> Device<SpiBus, NetworkImpl> {
        self.device.into_inner()
    }
}

impl<Spi: FullDuplex<u8>, ChipSelect: OutputPin>
    Interface<ActiveFourWire<Spi, ChipSelect>, Manual>
{
    pub fn setup(
        spi: Spi,
        cs: ChipSelect,
        mac: MacAddress,
        ip: Ipv4Addr,
    ) -> Result<Self, InitializeError<<ActiveFourWire<Spi, ChipSelect> as ActiveBus>::Error>> {
        Ok(UninitializedDevice::new(FourWire::new(cs).activate(spi))
            .initialize_manual(mac, ip, Mode::default())?
            .into_interface())
    }
}

impl<SpiBus: ActiveBus, NetworkImpl: Network> From<Device<SpiBus, NetworkImpl>>
    for Interface<SpiBus, NetworkImpl>
{
    fn from(device: Device<SpiBus, NetworkImpl>) -> Interface<SpiBus, NetworkImpl> {
        Interface::<SpiBus, NetworkImpl>::new(device)
    }
}
