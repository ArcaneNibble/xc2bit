//! Top-level bitstream functions

use bittwiddler_core::prelude::{BitArray as BittwiddlerBitArray, Coordinate};
use bitvec::prelude::*;

use crate::{
    global_fuses::GlobalFuses,
    partdb::{XC2Device, XC2Part},
};

pub(crate) trait BitHolder {
    fn get(&self, idx: usize) -> bool;
    fn set(&mut self, idx: usize, val: bool);
}
impl BitHolder for &mut BitArray {
    fn get(&self, idx: usize) -> bool {
        self[idx]
    }
    fn set(&mut self, idx: usize, val: bool) {
        BitSlice::set(self, idx, val)
    }
}
#[cfg(feature = "alloc")]
impl BitHolder for BitBox {
    fn get(&self, idx: usize) -> bool {
        self[idx]
    }
    fn set(&mut self, idx: usize, val: bool) {
        BitSlice::set(self, idx, val)
    }
}

/// Bitstream struct
#[allow(private_bounds)]
pub struct Coolrunner2<B: BitHolder> {
    pub(crate) part: XC2Part,
    pub(crate) bits: B,
}
#[cfg(feature = "alloc")]
impl Coolrunner2<BitBox> {
    pub fn new(part: XC2Part) -> Self {
        let fuse_dims = part.device.fuse_array_dims();
        let bits = bitbox![0; fuse_dims.0 * fuse_dims.1];

        let mut ret = Self { part, bits };

        // Initialize security/done/usercode rows to all 1s
        for x in 0..fuse_dims.0 {
            ret.set(Coordinate::new(x, fuse_dims.1 - 1), true);
            ret.set(Coordinate::new(x, fuse_dims.1 - 2), true);
        }

        // done1
        ret.set(part.device.done1(), false);

        ret
    }
}
impl<B: BitHolder> BittwiddlerBitArray for Coolrunner2<B> {
    fn get(&self, c: Coordinate) -> bool {
        let (fuse_dims_w, _) = self.part.device.fuse_array_dims();
        BitHolder::get(&self.bits, c.y * fuse_dims_w + c.x)
    }
    fn set(&mut self, c: Coordinate, val: bool) {
        let (fuse_dims_w, _) = self.part.device.fuse_array_dims();
        BitHolder::set(&mut self.bits, c.y * fuse_dims_w + c.x, val)
    }
}

pub trait BuriedMacrocells {
    fn has_io_at(&self, fb: u8, mc: u8) -> bool;
}
impl BuriedMacrocells for XC2Device {
    fn has_io_at(&self, fb: u8, mc: u8) -> bool {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => true,
            XC2Device::XC2C64 | XC2Device::XC2C64A => true,
            XC2Device::XC2C128 => match fb {
                0 | 1 | 5 | 7 => !(6..10).contains(&mc),
                2 | 3 | 4 | 6 => !(7..10).contains(&mc),
                _ => unreachable!(),
            },
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => !(5..11).contains(&mc),
            XC2Device::XC2C512 => match fb {
                0 | 1 | 3 | 5 | 7 | 8 | 9 | 10 | 12 | 14 | 17 | 19 | 21 | 23 | 24 | 26 | 28
                | 30 => !(4..12).contains(&mc),
                2 | 4 | 6 | 11 | 13 | 15 | 16 | 18 | 20 | 22 | 25 | 27 | 29 | 31 => {
                    !(5..12).contains(&mc)
                }
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::partdb::XC2Device;

    #[test]
    #[cfg(feature = "alloc")]
    fn smoke_test_create_new() {
        let mut bitstream = Coolrunner2::new(XC2Part::new(XC2Device::XC2C32A, None, None).unwrap());

        assert_eq!(bitstream.bits.len(), 260 * 50);
        BittwiddlerBitArray::set(&mut bitstream, Coordinate::new(1, 1), true);
        assert!(bitstream.bits[261]);
    }
}
