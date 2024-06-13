//! Macrocell functions

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{fb::FunctionBlock, partdb::XC2Device, spreadsheet_magic::xc2c32a_macrocell};

include!(concat!(env!("OUT_DIR"), "/mc-clk-src.rs"));
include!(concat!(env!("OUT_DIR"), "/mc-fb.rs"));
include!(concat!(env!("OUT_DIR"), "/mc-ff-mode.rs"));
include!(concat!(env!("OUT_DIR"), "/mc-r-src.rs"));
include!(concat!(env!("OUT_DIR"), "/mc-s-src.rs"));
include!(concat!(env!("OUT_DIR"), "/mc-xor-mode.rs"));

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Macrocell {
    pub(crate) x: FunctionBlock,
    pub(crate) mc: u8,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl Macrocell {
    #[bittwiddler::property]
    pub fn clk_src(&self) -> ClockSourceAccessor {
        ClockSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn clk_inv(&self) -> ClockInvertAccessor {
        ClockInvertAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn is_ddr(&self) -> IsDDRAccessor {
        IsDDRAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn r_src(&self) -> ResetSourceAccessor {
        ResetSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn s_src(&self) -> SetSourceAccessor {
        SetSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn init_state(&self) -> InitStateAccessor {
        InitStateAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn ff_mode(&self) -> FlipFlopModeAccessor {
        FlipFlopModeAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn fb_src(&self) -> MacroocellFeedbackSourceAccessor {
        MacroocellFeedbackSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn use_iob(&self) -> UseIOAccessor {
        UseIOAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn xor_mode(&self) -> XorModeAccessor {
        XorModeAccessor { x: *self }
    }
}

macro_rules! declare_accessor {
    ($name:ident, $nbits:expr, $out:ident,$spreadsheet:ident) => {
        crate::mc::declare_accessor!($name, $nbits, $out, false, $spreadsheet);
    };
    ($name:ident, $nbits:expr, $out:ident,$spreadsheet:ident, nodefault) => {
        rate::mc::declare_accessor!($name, $nbits, $out, false, $spreadsheet, nodefault);
    };
    ($name:ident, $nbits:expr, $out:ident, $invert:expr, $spreadsheet:ident) => {
        crate::mc::declare_accessor!($name, $nbits, $out, $invert, $spreadsheet, nodefault);
        impl PropertyAccessorWithDefault for $name {}
    };
    ($name:ident, $nbits:expr, $out:ident, $invert:expr, $spreadsheet:ident, nodefault) => {
        #[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
        pub struct $name {
            x: Macrocell,
        }
        impl PropertyAccessor for $name {
            type BoolArray = [bool; $nbits];
            type Output = $out;

            fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool) {
                let device = self.x.x.device;
                let fb = self.x.x.fb;

                match device {
                    XC2Device::XC2C32 | XC2Device::XC2C32A => (
                        {
                            let c = Coordinate::new(0, self.x.mc as usize * xc2c32a_macrocell::H)
                                + xc2c32a_macrocell::$spreadsheet[biti];
                            if fb % 2 == 0 {
                                device.fb_corner(fb) + c
                            } else {
                                device.fb_corner(fb).sub_x_add_y(c)
                            }
                        },
                        $invert,
                    ),
                    XC2Device::XC2C64 | XC2Device::XC2C64A => todo!(),
                    XC2Device::XC2C128 => todo!(),
                    XC2Device::XC2C256 => todo!(),
                    XC2Device::XC2C384 => todo!(),
                    XC2Device::XC2C512 => todo!(),
                }
            }
        }
        #[cfg(feature = "alloc")]
        impl PropertyAccessorWithStringConv for $name {}
    };
}
pub(crate) use declare_accessor;

declare_accessor!(ClockSourceAccessor, 3, RegClkSrc, CLOCK_SOURCE);
declare_accessor!(ClockInvertAccessor, 1, bool, CLOCK_INV);
declare_accessor!(IsDDRAccessor, 1, bool, DOUBLE_DATA_RATE);
declare_accessor!(ResetSourceAccessor, 2, RegResetSrc, RESET_SOURCE);
declare_accessor!(SetSourceAccessor, 2, RegSetSrc, SET_SOURCE);
declare_accessor!(InitStateAccessor, 1, bool, true, POWERUP_STATE, nodefault);
declare_accessor!(FlipFlopModeAccessor, 2, FlipFlopMode, FLIP_FLOP_MODE);
declare_accessor!(
    MacroocellFeedbackSourceAccessor,
    2,
    MacrocellFeedbackSrc,
    MC_FEEDBACK_SRC
);
declare_accessor!(UseIOAccessor, 1, bool, true, USE_IOB);
declare_accessor!(XorModeAccessor, 2, XorMode, XOR_MODE);

impl PropertyAccessorWithDefault for InitStateAccessor {
    fn is_at_default(&self, bitstream: &(impl BitArray + ?Sized)) -> bool {
        let val = self.get(bitstream);
        val == true
    }
}
