//! Top-level bitstream functions

use bittwiddler_core::prelude::{
    BitArray as BittwiddlerBitArray, Coordinate, HumanLevelThatHasState, HumanSinkForStatePieces,
};
use bittwiddler_macros::bittwiddler_properties;
use bitvec::prelude::*;

use crate::{global_bits_code::GCK, global_fuses::GlobalFuses, partdb::XC2Part};

pub(crate) trait BitHolder {
    fn get(&self, idx: usize) -> bool;
    fn set(&mut self, idx: usize, val: bool);
    fn wipe(&mut self);
}
impl BitHolder for &mut BitArray {
    fn get(&self, idx: usize) -> bool {
        self[idx]
    }
    fn set(&mut self, idx: usize, val: bool) {
        BitSlice::set(self, idx, val)
    }
    fn wipe(&mut self) {
        self.fill(false);
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
    fn wipe(&mut self) {
        self.fill(false);
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
        ret.make_blank(false);

        ret
    }
}
#[allow(private_bounds)]
impl<B: BitHolder> Coolrunner2<B> {
    pub fn make_blank(&mut self, full_wipe: bool) {
        if full_wipe {
            self.bits.wipe();
        }

        let fuse_dims = self.part.device.fuse_array_dims();

        // Initialize security/done/usercode rows to all 1s
        for x in 0..fuse_dims.0 {
            self.set(Coordinate::new(x, fuse_dims.1 - 1), true);
            self.set(Coordinate::new(x, fuse_dims.1 - 2), true);
        }

        // done1
        self.set(self.part.device.done1(), false);
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

#[cfg(feature = "alloc")]
impl<B: BitHolder> HumanLevelThatHasState for Coolrunner2<B> {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
#[allow(private_bounds)]
impl<B: BitHolder> Coolrunner2<B> {
    #[bittwiddler::property]
    pub fn gck_enabled(&self, gck_idx: u8) -> GCK {
        assert!(gck_idx < 3);
        GCK {
            device: self.part.device,
            gck_idx,
        }
    }
}
#[cfg(feature = "alloc")]
impl<B: BitHolder> Coolrunner2AutomagicRequiredFunctions for Coolrunner2<B> {
    fn _automagic_construct_all_gck_enabled(&self) -> impl Iterator<Item = GCK> {
        (0..3).map(|gck_idx| self.gck_enabled(gck_idx))
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
