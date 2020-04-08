#![cfg_attr(feature = "std", allow(unreachable_code, unused_variables))]

use core::{
    fmt::{self, Write},
    slice,
};

const ADDRESS_BASE: usize = 0xE000_0000;

/// ITM stimulus port handler.
#[derive(Clone, Copy)]
pub struct Port {
    address: usize,
}

pub trait Value: Copy {
    fn port_write(address: usize, value: Self);
}

impl Port {
    /// Creates a new ITM stimulus port handler.
    ///
    /// # Panics
    ///
    /// If `port` is out of bounds.
    #[inline]
    pub fn new(address: usize) -> Self {
        assert!(address < 0x20);
        Self { address: ADDRESS_BASE + (address << 2) }
    }

    /// Writes `bytes` to the stimulus port.
    #[inline]
    pub fn write_bytes(self, bytes: &[u8]) {
        fn write_slice<T: Value>(port: Port, slice: &[T]) {
            for item in slice {
                port.write(*item);
            }
        }
        let mut end = bytes.len();
        if end < 4 {
            return write_slice(self, bytes);
        }
        let mut start = bytes.as_ptr() as usize;
        let mut rem = start & 0b11;
        end += start;
        if rem != 0 {
            rem = 0b100 - rem;
            write_slice(self, unsafe { slice::from_raw_parts(start as *const u8, rem) });
            start += rem;
        }
        rem = end & 0b11;
        end -= rem;
        write_slice(self, unsafe { slice::from_raw_parts(start as *const u32, end - start >> 2) });
        write_slice(self, unsafe { slice::from_raw_parts(end as *const u8, rem) });
    }

    /// Writes `value` of type `u8`, `u16` or `u32` to the stimulus port.
    ///
    /// This method can be chained.
    #[inline]
    pub fn write<T: Value>(self, value: T) -> Self {
        T::port_write(self.address, value);
        self
    }
}

impl Write for Port {
    #[inline]
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_bytes(string.as_bytes());
        Ok(())
    }
}

impl Value for u8 {
    fn port_write(address: usize, value: Self) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe {
            asm!("
            0:
                ldrexb r0, [$1]
                cmp r0, #0
                itt ne
                strexbne r0, $0, [$1]
                cmpne r0, #1
                beq 0b
            "   :
                : "r"(value), "r"(address as *mut Self)
                : "r0", "cc"
                : "volatile"
            );
        }
    }
}

impl Value for u16 {
    fn port_write(address: usize, value: Self) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe {
            asm!("
            0:
                ldrexh r0, [$1]
                cmp r0, #0
                itt ne
                strexhne r0, $0, [$1]
                cmpne r0, #1
                beq 0b
            "   :
                : "r"(value), "r"(address as *mut Self)
                : "r0", "cc"
                : "volatile"
            );
        }
    }
}

impl Value for u32 {
    fn port_write(address: usize, value: Self) {
        #[cfg(feature = "std")]
        return unimplemented!();
        unsafe {
            asm!("
            0:
                ldrex r0, [$1]
                cmp r0, #0
                itt ne
                strexne r0, $0, [$1]
                cmpne r0, #1
                beq 0b
            "   :
                : "r"(value), "r"(address as *mut Self)
                : "r0", "cc"
                : "volatile"
            );
        }
    }
}
