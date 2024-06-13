//! Top-level bitstream functions

use bittwiddler_core::prelude::{BitArray as BittwiddlerBitArray, Coordinate, PropertyAccessor};
#[cfg(feature = "alloc")]
use bittwiddler_core::prelude::{HumanLevelThatHasState, HumanSinkForStatePieces};
use bittwiddler_macros::bittwiddler_properties;
use bitvec::prelude::*;

use crate::{
    fb::FunctionBlock,
    global_bits_code::{ClockDivider, GCKEn, GSREn, GSRInv, GTSEn, GTSInv, GlobalTermAccessor},
    global_fuses::GlobalFuses,
    io::ExtraDedicatedInput,
    partdb::{XC2Device, XC2Part},
};

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
#[allow(private_bounds)]
impl<B: BitHolder> Coolrunner2<B> {
    pub fn get_prop<A: PropertyAccessor>(&self, accessor: &A) -> A::Output {
        accessor.get(self)
    }
    pub fn set_prop<A: PropertyAccessor>(&mut self, accessor: &A, val: A::Output) {
        accessor.set(self, val);
    }
}
#[cfg(feature = "alloc")]
impl<B: BitHolder> HumanLevelThatHasState for Coolrunner2<B> {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
#[allow(private_bounds)]
impl<B: BitHolder> Coolrunner2<B> {
    pub fn fb(&self, fb: u8) -> FunctionBlock {
        assert!((fb as usize) < self.part.device.num_fbs());
        FunctionBlock {
            device: self.part.device,
            fb,
        }
    }

    #[bittwiddler::property]
    pub fn gck_enabled(&self, gck_idx: u8) -> GCKEn {
        assert!(gck_idx < 3);
        GCKEn {
            device: self.part.device,
            gck_idx,
        }
    }

    #[bittwiddler::property]
    pub fn gsr_enabled(&self) -> GSREn {
        GSREn {
            device: self.part.device,
        }
    }
    #[bittwiddler::property]
    pub fn gsr_invert(&self) -> GSRInv {
        GSRInv {
            device: self.part.device,
        }
    }

    #[bittwiddler::property]
    pub fn gts_enabled(&self, gts_idx: u8) -> GTSEn {
        assert!(gts_idx < 4);
        GTSEn {
            device: self.part.device,
            gts_idx,
        }
    }
    #[bittwiddler::property]
    pub fn gts_invert(&self, gts_idx: u8) -> GTSInv {
        assert!(gts_idx < 4);
        GTSInv {
            device: self.part.device,
            gts_idx,
        }
    }

    #[bittwiddler::property]
    pub fn global_termination(&self) -> GlobalTermAccessor {
        GlobalTermAccessor {
            device: self.part.device,
        }
    }

    #[bittwiddler::conditional]
    pub fn extra_dedicated_input(&self) -> ExtraDedicatedInput {
        ExtraDedicatedInput {}
    }

    #[bittwiddler::conditional]
    pub fn clock_divider(&self) -> ClockDivider {
        ClockDivider {
            device: self.part.device,
        }
    }
}
#[cfg(feature = "alloc")]
impl<B: BitHolder> Coolrunner2AutomagicRequiredFunctions for Coolrunner2<B> {
    fn _automagic_construct_all_gck_enabled(&self) -> impl Iterator<Item = GCKEn> {
        (0..3).map(|gck_idx| self.gck_enabled(gck_idx))
    }
    fn _automagic_construct_all_gts_enabled(&self) -> impl Iterator<Item = GTSEn> {
        (0..4).map(|gts_idx| self.gts_enabled(gts_idx))
    }
    fn _automagic_construct_all_gts_invert(&self) -> impl Iterator<Item = GTSInv> {
        (0..4).map(|gts_idx| self.gts_invert(gts_idx))
    }
    fn _automagic_construct_all_fb(&self) -> impl Iterator<Item = FunctionBlock> {
        (0..self.part.device.num_fbs()).map(|fb| self.fb(fb as u8))
    }
    fn _automagic_construct_all_extra_dedicated_input(
        &self,
    ) -> impl Iterator<Item = ExtraDedicatedInput> {
        let mut x = [self.extra_dedicated_input()].into_iter();
        if self.part.device != XC2Device::XC2C32 && self.part.device != XC2Device::XC2C32A {
            x.next();
        }
        x
    }
    fn _automagic_construct_all_clock_divider(&self) -> impl Iterator<Item = ClockDivider> {
        let mut x = [self.clock_divider()].into_iter();
        if !self.part.device.has_large_macrocells() {
            x.next();
        }
        x
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

    #[test]
    #[cfg(feature = "alloc")]
    fn smoke_test_rw_in_rust() {
        let mut bitstream = Coolrunner2::new(XC2Part::new(XC2Device::XC2C32A, None, None).unwrap());

        bitstream.set_prop(&bitstream.fb(0).and_term(1).inp(2), true);
    }
}
