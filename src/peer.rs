
use errors::*;
use sleep_register::{HyperRegister, SleepDirRegister};
use protocol::DatConnection;
use bitfield::Bitfield;

pub struct DatPeer {
    registers: Vec<SleepDirRegister>,
    connection: DatConnection,
    have_log: Vec<Vec<Bitfield>>,
}

impl DatPeer {

    pub fn new(connection: DatConnection, registers: Vec<SleepDirRegister>) -> DatPeer {
        DatPeer {
            registers,
            connection,
            have_log: vec![],
        }
    }

    pub fn has(self, register: u64, index: u64) -> Result<bool> {
        for bitfield in self.have_log[register as usize].iter() {
            if bitfield.get(index)? {
                return Ok(true)
            }
        }
        Ok(false)
    }
}
