#![no_std]

/// The number of inputs from the ZIA interconnect into the AND gate section of each PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const ZIA_ROWS: usize = 40;
/// The number of AND gates in each PLA. This is also the number of inputs into each OR gate in the PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const ANDTERMS_PER_FB: usize = 56;
/// The number of macrocells in each function block. This is also the number of OR gates in each PLA.
/// This is an unchangeable property of the architecture of the CPLD.
pub const MCS_PER_FB: usize = 16;

/// The number of BUFG sites for clock signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_CLK: usize = 3;
/// The number of BUFG sites for tristate signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_GTS: usize = 4;
/// The number of BUFG sites for set/reset signals in the device.
/// This is an unchangeable property of the architecture of the CPLD.
pub const NUM_BUFG_GSR: usize = 1;

pub mod bitstream;
#[cfg(feature = "std")]
pub mod crbit;
pub mod fb;
pub mod global_bits_code;
pub mod global_fuses;
pub mod io;
pub mod jed;
pub mod mc;
pub mod partdb;
pub mod zia;

pub mod spreadsheet_magic {
    include!(concat!(env!("OUT_DIR"), "/tiles-out.rs"));
}

#[cfg(feature = "alloc")]
extern crate alloc;
