//! Formated print crate
#![no_std]

use embedded_hal::serial;

use core::fmt;

pub struct SerialIO<TX, RX> {
    tx: TX,
    rx: RX,
}

impl<TX, RX> SerialIO<TX, RX>
where TX: serial::Write<u8>,
    RX: serial::Read<u8>
{
    pub fn new(tx: TX, rx: RX) -> Self
    {
        Self {
            tx: tx,
            rx: rx,
        }
    }

    pub fn release(self) -> (TX, RX) {
        (self.tx, self.rx)
    }

    pub fn _fmt_write(&mut self, args: fmt::Arguments) -> Result<(), fmt::Error> {
        use core::fmt::Write;
        
        self.write_fmt(args)
    }
}

#[macro_export]
macro_rules! sprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial._fmt_write(format_args!($($arg)*)).ok()
    };
}

#[macro_export]
macro_rules! sprintln {
    ($serial:expr, $fmt:expr) => {
        $crate::sprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::sprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

impl<TX, RX> fmt::Write for SerialIO<TX, RX>
where TX: serial::Write<u8>,
    RX: serial::Read<u8>
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.tx.write(byte).ok();
        }
        Ok(())
    }
}
