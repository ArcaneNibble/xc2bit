//! Locations of various global fuses

use bittwiddler_core::prelude::Coordinate;

use crate::partdb::XC2Device;

pub trait GlobalFuses {
    fn done1(&self) -> Coordinate;
}
impl GlobalFuses for XC2Device {
    fn done1(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => Coordinate::new(9, 48),
            XC2Device::XC2C64 | XC2Device::XC2C64A => Coordinate::new(8, 96),
            XC2Device::XC2C128 => Coordinate::new(9, 80),
            XC2Device::XC2C256 => Coordinate::new(9, 96),
            XC2Device::XC2C384 => Coordinate::new(9, 120),
            XC2Device::XC2C512 => Coordinate::new(9, 160),
        }
    }
}
