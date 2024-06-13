//! Function block (and PLA)

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{
    io::IoPad, mc::Macrocell, partdb::XC2Device, zia::ZIARowAccessor, ANDTERMS_PER_FB, MCS_PER_FB,
    ZIA_ROWS,
};

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
    pub fn or_term(&self, mc: u8) -> OrTerm {
        assert!((mc as usize) < MCS_PER_FB);
        OrTerm { x: *self, mc }
    }
    pub fn mc(&self, mc: u8) -> Macrocell {
        assert!((mc as usize) < MCS_PER_FB);
        Macrocell { x: *self, mc }
    }
    pub fn io(&self, mc: u8) -> IoPad {
        assert!((mc as usize) < MCS_PER_FB);
        IoPad { x: *self, mc }
    }
    #[bittwiddler::property]
    pub fn zia_row(&self, zia_row: u8) -> ZIARowAccessor {
        assert!((zia_row as usize) < ZIA_ROWS);
        ZIARowAccessor { x: *self, zia_row }
    }
}
#[cfg(feature = "alloc")]
impl FunctionBlockAutomagicRequiredFunctions for FunctionBlock {
    fn _automagic_construct_all_and_term(&self) -> impl Iterator<Item = AndTerm> {
        (0..ANDTERMS_PER_FB).map(|pterm_i| self.and_term(pterm_i as u8))
    }
    fn _automagic_construct_all_or_term(&self) -> impl Iterator<Item = OrTerm> {
        (0..MCS_PER_FB).map(|mc| self.or_term(mc as u8))
    }
    fn _automagic_construct_all_mc(&self) -> impl Iterator<Item = Macrocell> {
        (0..MCS_PER_FB).map(|mc| self.mc(mc as u8))
    }
    fn _automagic_construct_all_io(&self) -> impl Iterator<Item = IoPad> {
        (0..MCS_PER_FB).map(|mc| self.io(mc as u8))
    }
    fn _automagic_construct_all_zia_row(&self) -> impl Iterator<Item = ZIARowAccessor> {
        (0..ZIA_ROWS).map(|zia_row| self.zia_row(zia_row as u8))
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
    pub fn inp(&self, zia_row: u8) -> TrueInput {
        assert!((zia_row as usize) < ZIA_ROWS);
        TrueInput { x: *self, zia_row }
    }

    #[bittwiddler::property]
    pub fn inp_n(&self, zia_row: u8) -> CompInput {
        assert!((zia_row as usize) < ZIA_ROWS);
        CompInput { x: *self, zia_row }
    }
}
#[cfg(feature = "alloc")]
impl AndTermAutomagicRequiredFunctions for AndTerm {
    fn _automagic_construct_all_inp(&self) -> impl Iterator<Item = TrueInput> {
        (0..ZIA_ROWS).map(|zia_row| self.inp(zia_row as u8))
    }
    fn _automagic_construct_all_inp_n(&self) -> impl Iterator<Item = CompInput> {
        (0..ZIA_ROWS).map(|zia_row| self.inp_n(zia_row as u8))
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TrueInput {
    pub(crate) x: AndTerm,
    pub(crate) zia_row: u8,
}
crate::bitstream::single_bool_impl!(TrueInput, self, {
    (
        and_get_bit_pos(
            self.x.x.device,
            self.x.x.fb,
            self.x.pterm_i,
            self.zia_row,
            false,
        ),
        true,
    )
});

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CompInput {
    pub(crate) x: AndTerm,
    pub(crate) zia_row: u8,
}
crate::bitstream::single_bool_impl!(CompInput, self, {
    (
        and_get_bit_pos(
            self.x.x.device,
            self.x.x.fb,
            self.x.pterm_i,
            self.zia_row,
            true,
        ),
        true,
    )
});

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
fn and_get_bit_pos(device: XC2Device, fb: u8, pterm_i: u8, inp_i: u8, comp: bool) -> Coordinate {
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
pub struct OrTerm {
    pub(crate) x: FunctionBlock,
    pub(crate) mc: u8,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl OrTerm {
    #[bittwiddler::property]
    pub fn inp(&self, pterm_i: u8) -> OrInput {
        assert!((pterm_i as usize) < ANDTERMS_PER_FB);
        OrInput { x: *self, pterm_i }
    }
}
#[cfg(feature = "alloc")]
impl OrTermAutomagicRequiredFunctions for OrTerm {
    fn _automagic_construct_all_inp(&self) -> impl Iterator<Item = OrInput> {
        (0..ANDTERMS_PER_FB).map(|pterm_i| self.inp(pterm_i as u8))
    }
}

const SIDE_OR_ROW_PERMUTE: [usize; 28] = [
    17, 19, 22, 20, 0, 1, 3, 4, 5, 7, 8, 11, 12, 13, 15, 16, 23, 24, 26, 27, 28, 31, 32, 34, 35,
    36, 38, 39,
];

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct OrInput {
    pub(crate) x: OrTerm,
    pub(crate) pterm_i: u8,
}
crate::bitstream::single_bool_impl!(OrInput, self, {
    (
        {
            let device = self.x.x.device;
            let fb = self.x.x.fb;
            let mc = self.x.mc;
            let pterm_i = self.pterm_i;
            match device {
                XC2Device::XC2C32
                | XC2Device::XC2C32A
                | XC2Device::XC2C64
                | XC2Device::XC2C64A
                | XC2Device::XC2C256 => {
                    let x_base = mc as usize % 2 + pterm_i as usize * 2;
                    let y_base = 20 + mc as usize / 2;

                    let mc_offset = if device == XC2Device::XC2C256 { 10 } else { 9 };

                    if fb % 2 == 0 {
                        device.fb_corner(fb as u8)
                            + Coordinate::new(mc_offset, 0)
                            + Coordinate::new(x_base, y_base)
                    } else {
                        device.fb_corner(fb as u8).sub_x_add_y(
                            Coordinate::new(mc_offset, 0) + Coordinate::new(x_base, y_base),
                        )
                    }
                }
                XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                    let y_base = SIDE_OR_ROW_PERMUTE[pterm_i as usize / 2];
                    let x_base = mc as usize * 2
                        + if y_base >= 23 {
                            1 - pterm_i as usize % 2
                        } else {
                            pterm_i as usize % 2
                        };

                    if fb % 2 == 0 {
                        device.fb_corner(fb as u8)
                            + Coordinate::new(15, 0)
                            + Coordinate::new(x_base, y_base)
                    } else {
                        device
                            .fb_corner(fb as u8)
                            .sub_x_add_y(Coordinate::new(15, 0) + Coordinate::new(x_base, y_base))
                    }
                }
            }
        },
        true,
    )
});
