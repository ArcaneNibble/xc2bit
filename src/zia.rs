//! ZIA global interconnect

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{fb::FunctionBlock, partdb::XC2Device, ANDTERMS_PER_FB, MCS_PER_FB};

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ZIARowAccessor {
    pub(crate) x: FunctionBlock,
    pub(crate) zia_row: u8,
}
impl PropertyAccessor for ZIARowAccessor {
    type BoolArray = [bool; 0];
    type Output = ZIARow;

    fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool) {
        let x_base = (self.x.device.zia_width() - 1 - biti) * 2 + self.x.fb as usize % 2;
        let y_base = match self.x.device {
            XC2Device::XC2C32
            | XC2Device::XC2C32A
            | XC2Device::XC2C64
            | XC2Device::XC2C64A
            | XC2Device::XC2C256 => self.zia_row + if self.zia_row >= 20 { 8 } else { 0 },
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => self.zia_row,
        } as usize;

        let mc_or_and_offset = match self.x.device {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                9 + ANDTERMS_PER_FB * 2
            }
            XC2Device::XC2C256 => 10 + ANDTERMS_PER_FB * 2,
            XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                15 + MCS_PER_FB * 2 + ANDTERMS_PER_FB * 2
            }
        };

        (
            self.x.device.fb_corner((self.x.fb & !1) as u8)
                + Coordinate::new(mc_or_and_offset, 0)
                + Coordinate::new(x_base, y_base),
            false,
        )
    }

    fn get(&self, bitstream: &(impl BitArray + ?Sized)) -> Self::Output {
        todo!()
    }

    fn set(&self, bitstream: &mut (impl BitArray + ?Sized), val: Self::Output) {
        todo!()
    }
}

pub enum ZIARow {
    GND,
    VCC,
    MuxChoice(u8),
    Invalid([bool; 88]),
}
/// FAKE implementation which is bypassed with specialization in [ZIARowAccessor]
impl PropertyLeaf<[bool; 0]> for ZIARow {
    fn from_bits(_bits: &[bool; 0]) -> Self {
        unreachable!()
    }
    fn to_bits(&self) -> [bool; 0] {
        unreachable!()
    }
}
impl Default for ZIARow {
    fn default() -> Self {
        Self::VCC
    }
}
#[cfg(feature = "alloc")]
impl PropertyLeafWithStringConv<[bool; 0], ZIARowAccessor> for ZIARow {
    // todo custom formatting
}
