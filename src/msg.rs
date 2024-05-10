//! I2C msg abstraction

use bitflags::bitflags;

bitflags! {
    /// I2c msgs flags
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct I2cMsgFlags: u16 {
        /// read data (from slave to master). Guaranteed to be 0x0001
        const I2cMasterRead = 0x0001;
        /// this is a 10 bit chip address
        /// Only if I2C_FUNC_10BIT_ADDR is set
        const I2cMasterTen =  0x0010;
        /// Linux kernel
        const I2cMasterDmaSafe = 0x0200;
        /// message length will be first received byte
        /// Only if I2C_FUNC_SMBUS_READ_BLOCK_DATA is set
        const I2cMasterRecvLen = 0x0400;
        /// in a read message, master ACK/NACK bit is skipped
        const I2cMasterNoReadAck = 0x0800;
        /// treat NACK from client as ACK
        const I2cMasterIgnNak = 0x1000;
        /// toggles the Rd/Wr bit
        const I2cMasterRevDir = 0x2000;
        /// skip repeated start sequence
        const I2cMasterNoStart = 0x4000;
        /// force a STOP condition after the message
        const I2cMasterStop = 0x8000;
    }
}

/// an I2C transaction segment beginning with START
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct I2cMsg<'a> {
    ///  Slave address, either 7 or 10 bits. When this is a 10 bit address,
    ///  I2C_M_TEN must be set in @flags and the adapter must support I2C_FUNC_10BIT_ADDR
    addr: u16,
    /// msg flags:
    flags: I2cMsgFlags,
    /// The buffer into which data is read, or from which it's written
    buf: &'a [u8],
}

impl<'a> Default for I2cMsg<'a> {
    fn default() -> Self {
        Self {
            addr: 0,
            flags: I2cMsgFlags::empty(),
            buf: &[],
        }
    }
}

impl<'a> I2cMsg<'a> {
    /// Create a new I2cMsg
    pub fn new(addr: u16, flags: I2cMsgFlags, buf: &'a [u8]) -> Self {
        I2cMsg { addr, flags, buf }
    }
}

/// Represent I2C transfer method
pub trait I2cAlgorithm {
    /// Issue a set of i2c transactions to the given I2C adapter
    /// defined by the msgs array, with num messages available to transfer via
    /// the adapter specified by adap
    fn master_xfer();

    /// same as @master_xfer. Yet, only using atomic context
    /// so e.g. PMICs can be accessed very late before shutdown. Optional.
    fn master_xfer_atomic() {
        unimplemented!();
    }

    /// smbus_xfer: Issue smbus transactions to the given I2C adapter. If this
    /// is not present, then the bus layer will try and convert the SMBus calls
    /// into I2C transfers instead
    fn smbus_xfer() {
        unimplemented!();
    }

    /// same as @smbus_xfer. Yet, only using atomic context
    fn smbus_xfer_atomic() {
        unimplemented!();
    }
}
