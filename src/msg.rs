//! I2C msg abstraction


/// msg flags
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum I2cMsgFlags {
    /// read data (from slave to master). Guaranteed to be 0x0001!
    I2cMasterRead,
    /// this is a 10 bit chip address
    /// Only if I2C_FUNC_10BIT_ADDR is set
    I2cMasterTen,

    /// message length will be first received byte
    /// Only if I2C_FUNC_SMBUS_READ_BLOCK_DATA is set
    I2cMasterRecvLen,
    /// Linux kernel 
    I2cMasterDmaSafe,
    /// in a read message, master ACK/NACK bit is skipped
    I2cMasterNoReadAck,
    /// treat NACK from client as ACK
    I2cMasterIgnNak,
    /// toggles the Rd/Wr bit
    I2cMasterRevDir,
    /// force a STOP condition after the message
    I2cMasterStop,
}


/// an I2C transaction segment beginning with START 
#[allow(dead_code)]
pub struct I2cMsg{
    ///  Slave address, either 7 or 10 bits. When this is a 10 bit address,
    ///  I2C_M_TEN must be set in @flags and the adapter must support I2C_FUNC_10BIT_ADDR
    addr: u16,
    /// Number of data bytes in @buf being read from or written to the I2C
    /// slave address. For read transactions where %I2C_M_RECV_LEN is set, the
    /// caller guarantees that this buffer can hold up to %I2C_SMBUS_BLOCK_MAX
    /// bytes in addition to the initial length byte sent by the slave (plus,
    /// if used, the SMBus PEC); and this value will be incremented by the number
    /// of block data bytes received.
    len: u16,
    /// The buffer into which data is read, or from which it's written
    buf: *mut u8,
    /// msg flags:
    flags: I2cMsgFlags,
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


