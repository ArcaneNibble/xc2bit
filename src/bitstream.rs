use bittwiddler_core::prelude::Coordinate;
use bitvec::prelude::*;

use crate::partdb::XC2Part;

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
        let mut bits = bitbox![0; fuse_dims.0 * fuse_dims.1];

        // Initialize security/done/usercode rows to all 1s
        for x in 0..fuse_dims.0 {
            bits.set((fuse_dims.1 - 1) * fuse_dims.0 + x, true);
            bits.set((fuse_dims.1 - 2) * fuse_dims.0 + x, true);
        }

        // TODO VERY IMPORTANT done1

        Self { part, bits }
    }
}
impl<B: BitHolder> bittwiddler_core::prelude::BitArray for Coolrunner2<B> {
    fn get(&self, c: Coordinate) -> bool {
        let (fuse_dims_w, _) = self.part.device.fuse_array_dims();
        BitHolder::get(&self.bits, c.y * fuse_dims_w + c.x)
    }
    fn set(&mut self, c: Coordinate, val: bool) {
        let (fuse_dims_w, _) = self.part.device.fuse_array_dims();
        BitHolder::set(&mut self.bits, c.y * fuse_dims_w + c.x, val)
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
        bittwiddler_core::prelude::BitArray::set(&mut bitstream, Coordinate::new(1, 1), true);
        assert!(bitstream.bits[261]);
    }
}
