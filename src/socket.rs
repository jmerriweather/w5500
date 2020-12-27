use crate::bus::ActiveBus;
use crate::register::socketn;
use embedded_nal::Ipv4Addr;

pub struct Socket {
    index: u8,
}

impl Socket {
    pub fn new(index: u8) -> Self {
        /*
         * Socket 0 is at address    0x01
         * Socket 0 TX is at address 0x02
         * Socket 0 RX is at address 0x03
         * skip                      0x04
         * Socket 1 is at address    0x05
         * Socket 1 TX is at address 0x06
         * Socket 1 RX is at address 0x07
         * ...
         */
        Socket { index }
    }

    pub const fn index(&self) -> u8 {
        self.index
    }

    pub const fn register(&self) -> u8 {
        (self.index * 4) + 1
    }
    pub const fn tx_buffer(&self) -> u8 {
        (self.index * 4) + 2
    }
    pub const fn rx_buffer(&self) -> u8 {
        (self.index * 4) + 3
    }

    pub fn set_mode<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        mode: socketn::Protocol,
    ) -> Result<(), SpiBus::Error> {
        let mode = [mode as u8];
        bus.write_frame(self.register(), socketn::MODE, &mode)?;
        Ok(())
    }

    pub fn reset_interrupt<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        code: socketn::Interrupt,
    ) -> Result<(), SpiBus::Error> {
        let data = [code as u8];
        bus.write_frame(self.register(), socketn::INTERRUPT, &data)?;
        Ok(())
    }

    pub fn has_interrupt<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        code: socketn::Interrupt,
    ) -> Result<bool, SpiBus::Error> {
        let mut data = [0u8];
        bus.read_frame(self.register(), socketn::INTERRUPT, &mut data)?;
        Ok(data[0] & code as u8 != 0)
    }

    pub fn set_source_port<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        port: u16,
    ) -> Result<(), SpiBus::Error> {
        let data = port.to_be_bytes();
        bus.write_frame(self.register(), socketn::SOURCE_PORT, &data)?;
        Ok(())
    }

    pub fn set_destination_ip<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        ip: Ipv4Addr,
    ) -> Result<(), SpiBus::Error> {
        let data = ip.octets();
        bus.write_frame(self.register(), socketn::DESTINATION_IP, &data)?;
        Ok(())
    }

    pub fn set_destination_port<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        port: u16,
    ) -> Result<(), SpiBus::Error> {
        let data = port.to_be_bytes();
        bus.write_frame(self.register(), socketn::DESTINATION_PORT, &data)?;
        Ok(())
    }

    pub fn get_tx_read_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
    ) -> Result<u16, SpiBus::Error> {
        let mut data = [0u8; 2];
        bus.read_frame(self.register(), socketn::TX_DATA_READ_POINTER, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    pub fn set_tx_read_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        pointer: u16,
    ) -> Result<(), SpiBus::Error> {
        let data = pointer.to_be_bytes();
        bus.write_frame(self.register(), socketn::TX_DATA_READ_POINTER, &data)?;
        Ok(())
    }

    pub fn get_tx_write_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
    ) -> Result<u16, SpiBus::Error> {
        let mut data = [0u8; 2];
        bus.read_frame(self.register(), socketn::TX_DATA_WRITE_POINTER, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    pub fn set_tx_write_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        pointer: u16,
    ) -> Result<(), SpiBus::Error> {
        let data = pointer.to_be_bytes();
        bus.write_frame(self.register(), socketn::TX_DATA_WRITE_POINTER, &data)?;
        Ok(())
    }

    pub fn get_rx_read_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
    ) -> Result<u16, SpiBus::Error> {
        let mut data = [0u8; 2];
        bus.read_frame(self.register(), socketn::RX_DATA_READ_POINTER, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    pub fn set_rx_read_pointer<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        pointer: u16,
    ) -> Result<(), SpiBus::Error> {
        let data = pointer.to_be_bytes();
        bus.write_frame(self.register(), socketn::RX_DATA_READ_POINTER, &data)?;
        Ok(())
    }

    pub fn set_interrupt_mask<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        mask: u8,
    ) -> Result<(), SpiBus::Error> {
        let data = [mask];
        bus.write_frame(self.register(), socketn::INTERRUPT_MASK, &data)?;
        Ok(())
    }

    pub fn command<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
        command: socketn::Command,
    ) -> Result<(), SpiBus::Error> {
        let data = [command as u8];
        bus.write_frame(self.register(), socketn::COMMAND, &data)?;
        Ok(())
    }

    pub fn get_receive_size<SpiBus: ActiveBus>(
        &self,
        bus: &mut SpiBus,
    ) -> Result<u16, SpiBus::Error> {
        loop {
            // Section 4.2 of datasheet, Sn_TX_FSR address docs indicate that read must be repeated until two sequential reads are stable
            let mut sample_0 = [0u8; 2];
            bus.read_frame(self.register(), socketn::RECEIVED_SIZE, &mut sample_0)?;
            let mut sample_1 = [0u8; 2];
            bus.read_frame(self.register(), socketn::RECEIVED_SIZE, &mut sample_1)?;
            if sample_0 == sample_1 && sample_0[0] >= 8 {
                break Ok(u16::from_be_bytes(sample_0));
            }
        }
    }
}
