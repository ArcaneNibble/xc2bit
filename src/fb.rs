//! Function block

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{partdb::XC2Device, ANDTERMS_PER_FB, MCS_PER_FB, ZIA_ROWS};

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FunctionBlock {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) fb: u8,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl FunctionBlock {
    pub fn and_term(&self, pterm_i: u8) -> AndTerm {
        assert!((pterm_i as usize) < ANDTERMS_PER_FB);
        AndTerm { x: *self, pterm_i }
    }
}
#[cfg(feature = "alloc")]
impl FunctionBlockAutomagicRequiredFunctions for FunctionBlock {
    fn _automagic_construct_all_and_term(&self) -> impl Iterator<Item = AndTerm> {
        (0..ANDTERMS_PER_FB).map(|pterm_i| self.and_term(pterm_i as u8))
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AndTerm {
    pub(crate) x: FunctionBlock,
    pub(crate) pterm_i: u8,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl AndTerm {
    #[bittwiddler::property]
    pub fn inp(&self, inp_i: u8) -> TrueInput {
        assert!((inp_i as usize) < ZIA_ROWS);
        TrueInput { x: *self, inp_i }
    }

    #[bittwiddler::property]
    pub fn inp_n(&self, inp_i: u8) -> CompInput {
        assert!((inp_i as usize) < ZIA_ROWS);
        CompInput { x: *self, inp_i }
    }
}
#[cfg(feature = "alloc")]
impl AndTermAutomagicRequiredFunctions for AndTerm {
    fn _automagic_construct_all_inp(&self) -> impl Iterator<Item = TrueInput> {
        (0..ZIA_ROWS).map(|inp_i| self.inp(inp_i as u8))
    }
    fn _automagic_construct_all_inp_n(&self) -> impl Iterator<Item = CompInput> {
        (0..ZIA_ROWS).map(|inp_i| self.inp_n(inp_i as u8))
    }
}

#[rustfmt::skip]
const NOGAP_AND_TERM_PERMUTE: [usize; ANDTERMS_PER_FB] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    14, 15, 16,
    20, 21, 22,
    26, 27, 28,
    32, 33, 34,
    38, 39, 40,
    44, 45, 46,
    50, 51, 52,
    55, 54, 53,
    49, 48, 47,
    43, 42, 41,
    37, 36, 35,
    31, 30, 29,
    25, 24, 23,
    19, 18, 17,
    13, 12, 11
];
pub(crate) fn and_get_bit_pos(
    device: XC2Device,
    fb: u8,
    pterm_i: u8,
    inp_i: u8,
    comp: bool,
) -> Coordinate {
    match device {
        XC2Device::XC2C32
        | XC2Device::XC2C32A
        | XC2Device::XC2C64
        | XC2Device::XC2C64A
        | XC2Device::XC2C256 => {
            let x_base = pterm_i as usize * 2 + if !comp { 1 } else { 0 };
            let y_base = (inp_i + if inp_i >= 20 { 8 } else { 0 }) as usize;

            let mc_offset = if device == XC2Device::XC2C256 { 10 } else { 9 };

            if fb % 2 == 0 {
                device.fb_corner(fb as u8)
                    + Coordinate::new(mc_offset, 0)
                    + Coordinate::new(x_base, y_base)
            } else {
                device
                    .fb_corner(fb as u8)
                    .sub_x_add_y(Coordinate::new(mc_offset, 0) + Coordinate::new(x_base, y_base))
            }
        }
        XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
            let x_base = NOGAP_AND_TERM_PERMUTE[pterm_i as usize] * 2 + if !comp { 1 } else { 0 };
            let y_base = inp_i as usize;

            if fb % 2 == 0 {
                device.fb_corner(fb as u8)
                    + Coordinate::new(15 + MCS_PER_FB * 2, 0)
                    + Coordinate::new(x_base, y_base)
            } else {
                device.fb_corner(fb as u8).sub_x_add_y(
                    Coordinate::new(15 + MCS_PER_FB * 2, 0) + Coordinate::new(x_base, y_base),
                )
            }
        }
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TrueInput {
    pub(crate) x: AndTerm,
    pub(crate) inp_i: u8,
}
impl PropertyAccessor for TrueInput {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (
            and_get_bit_pos(
                self.x.x.device,
                self.x.x.fb,
                self.x.pterm_i,
                self.inp_i,
                false,
            ),
            true,
        )
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CompInput {
    pub(crate) x: AndTerm,
    pub(crate) inp_i: u8,
}
impl PropertyAccessor for CompInput {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (
            and_get_bit_pos(
                self.x.x.device,
                self.x.x.fb,
                self.x.pterm_i,
                self.inp_i,
                true,
            ),
            true,
        )
    }
}
