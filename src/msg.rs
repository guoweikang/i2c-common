//! I2C msg abstraction

use bitflags::bitflags;

bitflags! {
    /// I2c msgs flags
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct I2cMsgFlags: u16 {
        /// read data (from slave to master). Guaranteed to be 0x0001
        const I2cMasterRead = 0x0001;
        /// Use Packet Error Checking
        const I2cClientPec = 0x0004;
        /// this is a 10 bit chip address
        /// Only if I2C_FUNC_10BIT_ADDR is set
        const I2cAddrTen =  0x0010;
        /// we are the slave
        const I2cClientSlave = 0x0020;
        /// We want to use I2C host notify
        const I2cClientHostNotify = 0x0040;
        /// for board_info; true if can wake
        const I2cClientWake = 0x0080;
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


         // Multi-bit flags
        /// Use Omnivision SCCB protocol Must match I2C_M_STOP|IGNORE_NAK
        const I2cClientSccb = Self::I2cMasterStop.bits() | Self::I2cMasterIgnNak.bits();
    }
}

/// an I2C transaction segment beginning with START
#[derive(Debug) ]
#[allow(dead_code)]
pub struct I2cMsg{
    ///  Slave address, either 7 or 10 bits. When this is a 10 bit address,
    ///  I2C_M_TEN must be set in @flags and the adapter must support I2C_FUNC_10BIT_ADDR
    addr: u16,
    /// msg flags:
    flags: I2cMsgFlags,
    /// The buffer into which data is read, or from which it's written
    buf: *mut u8,
    /// The buffer length
    buf_len: usize,
    /// record current read/write idx
    write_idx: isize,
    /// record current read/write idx
    read_idx: isize,
    /// when read msg, need record read cmd cnt 
    read_cmd_cnt: isize,
}

impl Default for I2cMsg {
    fn default() -> Self {
        Self {
            addr: 0,
            flags: I2cMsgFlags::empty(),
            buf: core::ptr::null_mut(),
            buf_len: 0,
            write_idx: 0,
            read_idx: 0,
            read_cmd_cnt: 0,
        }
    }
}

unsafe impl Send for I2cMsg {}
unsafe impl Sync for I2cMsg {}


impl I2cMsg {
    /// Create a new I2cMsg
    pub fn new(addr: u16, flags: I2cMsgFlags, buf:  *mut u8, buf_len: usize) -> Self {
         I2cMsg { addr, flags, buf, buf_len, write_idx: 0, read_idx: 0,  read_cmd_cnt: 0}
    }

    /// Get msg copy flags
    pub fn flags(&self) -> I2cMsgFlags {
        self.flags
    }

    /// remove one flag
    pub fn remove_flag(&mut self, flag: I2cMsgFlags) {
        self.flags.remove(flag);
    }

    /// Get msg addr 
    pub fn addr(&self) -> u16{
        self.addr
    }

    /// modify read_cmd_cnt only flags contains I2cMasterRecvLen
    pub  fn modify_read_cmd_cnt(&mut self, read_cmd_cnt: isize) {
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRecvLen));
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRead));
        self.read_cmd_cnt = read_cmd_cnt;
    }

    /// check wheather read cmd is send enough
    pub  fn inc_read_cmd_cnt(&mut self) {
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRead));
        self.read_cmd_cnt +=1;
    }

    /// Check whether the buffer pointer has left last one
    pub  fn left_last(&self) -> bool {
        // MasterRead means msg can be write
        if self.flags.contains(I2cMsgFlags::I2cMasterRead) {
            self.write_idx as usize == self.buf_len - 1
        } else {
            self.read_idx as usize == self.buf_len - 1
        }
    }

    /// Check whether the buffer pointer has reached the end
    pub  fn read_end(&self) -> bool {
        // MasterRead means msg can be write
        if self.flags.contains(I2cMsgFlags::I2cMasterRead) {
            self.read_cmd_cnt as usize == self.buf_len
        } else {
            self.read_idx as usize == self.buf_len
        }
    }

    /// Check whether the buffer pointer has reached the end
    pub  fn write_end(&self) -> bool {
        // MasterRead means msg can be write
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRead));
        self.write_idx as usize == self.buf_len
    }

    /// Write 1byte at the specified location
    pub  fn write_byte(&mut self, byte: u8) {
        // MasterRead means msg can be write
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRead));

        if self.write_end() {
            panic!("access buf overfllow");
        }

        unsafe{*self.buf.offset(self.write_idx) = byte};
        self.write_idx +=1;
    }

    /// Read 1byte from specified location
    pub  fn read_byte(&mut self) -> u8 {
        // MasterRead means msg can be write, don't alow read 
        debug_assert!(!self.flags.contains(I2cMsgFlags::I2cMasterRead));

        if self.read_end() {
            panic!("access buf overfllow");
        }

        let byte = unsafe{*self.buf.offset(self.read_idx)};
        self.read_idx +=1;
        byte
    }

    /// modify msg buf len
    pub fn modify_buf_len(&mut self, buf_len: usize) {
        debug_assert!(self.flags.contains(I2cMsgFlags::I2cMasterRecvLen));
        self.buf_len = buf_len;
    }
}
