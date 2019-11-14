use core::ops;
use register::{mmio::*, register_bitfields, register_structs};

register_bitfields! {
    u32,

    GPFSEL1 [
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD0 = 0b100
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD0 = 0b100
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x000 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x004 => SYSTMR_HI: ReadOnly<u32>),
        (0x008 => @END),
    }
}

pub struct DeviceDriver {
    base_addr: usize,
}

impl ops::Deref for DeviceDriver {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl DeviceDriver {
    pub fn new(base_addr: usize) -> Self {
        DeviceDriver { base_addr }
    }

    /// Returns a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    fn do_something(&self) -> u32 {
        self.GPFSEL1.set(0x1337);
        self.SYSTMR_HI.get()
    }
}

pub fn main2() {
    let dev = DeviceDriver::new(0x1337_0000);

    println!("{}", dev.do_something());
}
