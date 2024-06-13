//! IO-related functions

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::mc;
use crate::{
    fb::FunctionBlock,
    partdb::XC2Device,
    spreadsheet_magic::{big_macrocell, xc2c256_macrocell, xc2c32a_macrocell, xc2c64a_macrocell},
};

include!(concat!(env!("OUT_DIR"), "/io-fb.rs"));
include!(concat!(env!("OUT_DIR"), "/io-oe.rs"));
include!(concat!(env!("OUT_DIR"), "/io-regcom.rs"));
include!(concat!(env!("OUT_DIR"), "/io-slew.rs"));
include!(concat!(env!("OUT_DIR"), "/io-ibuf-mode.rs"));

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct IoPad {
    pub(crate) x: FunctionBlock,
    pub(crate) mc: u8,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl IoPad {
    #[bittwiddler::property]
    pub fn fb_src(&self) -> IoFeedbackSourceAccessor {
        IoFeedbackSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn schmitt_trigger(&self) -> SchmittTriggerAccessor {
        SchmittTriggerAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn input_pad_mode(&self) -> InputModeAccessor {
        InputModeAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn output_src(&self) -> OutputSourceAccessor {
        OutputSourceAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn output_pad_mode(&self) -> OutputModeAccessor {
        OutputModeAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn termination_enabled(&self) -> TerminationAccessor {
        TerminationAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn slew_rate(&self) -> SlewAccessor {
        SlewAccessor { x: *self }
    }
    #[bittwiddler::property]
    pub fn use_data_gate(&self) -> DataGateAccessor {
        DataGateAccessor { x: *self }
    }
}

// HACK
use IoPad as Macrocell;

mc::declare_accessor!(
    IoFeedbackSourceAccessor,
    2,
    IoFeedbackSource,
    IO_FEEDBACK_SRC
);
mc::declare_accessor!(OutputSourceAccessor, 1, PinOutputSrc, REG_OR_COMB);
mc::declare_accessor!(OutputModeAccessor, 4, OutputMode, OUTPUT_BUF_MODE);
mc::declare_accessor!(TerminationAccessor, 1, bool, TERMINATION_ENABLED);
mc::declare_accessor!(SlewAccessor, 1, SlewRate, SLEW_RATE);

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct SchmittTriggerAccessor {
    x: Macrocell,
}
impl PropertyAccessor for SchmittTriggerAccessor {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool) {
        let device = self.x.x.device;
        let fb = self.x.x.fb;

        match device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => (
                {
                    let c = Coordinate::new(0, self.x.mc as usize * xc2c32a_macrocell::H)
                        + xc2c32a_macrocell::SCHMITT_TRIGGER[biti];
                    if fb % 2 == 0 {
                        device.fb_corner(fb) + c
                    } else {
                        device.fb_corner(fb).sub_x_add_y(c)
                    }
                },
                false,
            ),
            XC2Device::XC2C64 | XC2Device::XC2C64A => (
                {
                    let c = Coordinate::new(0, self.x.mc as usize * xc2c64a_macrocell::H)
                        + xc2c64a_macrocell::SCHMITT_TRIGGER[biti];
                    if fb % 2 == 0 {
                        device.fb_corner(fb) + c
                    } else {
                        device.fb_corner(fb).sub_x_add_y(c)
                    }
                },
                false,
            ),
            _ => unreachable!(),
        }
    }
}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for SchmittTriggerAccessor {}
impl PropertyAccessorWithDefault for SchmittTriggerAccessor {}

macro_rules! declare_accessor_big_only {
    ($name:ident, $nbits:expr, $out:ident,$spreadsheet:ident) => {
        declare_accessor_big_only!($name, $nbits, $out, false, $spreadsheet);
    };
    ($name:ident, $nbits:expr, $out:ident,$spreadsheet:ident, nodefault) => {
        declare_accessor_big_only!($name, $nbits, $out, false, $spreadsheet, nodefault);
    };
    ($name:ident, $nbits:expr, $out:ident, $invert:expr, $spreadsheet:ident) => {
        declare_accessor_big_only!($name, $nbits, $out, $invert, $spreadsheet, nodefault);
        impl PropertyAccessorWithDefault for $name {}
    };
    ($name:ident, $nbits:expr, $out:ident, $invert:expr, $spreadsheet:ident, nodefault) => {
        #[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
        pub struct $name {
            x: IoPad,
        }
        impl PropertyAccessor for $name {
            type BoolArray = [bool; $nbits];
            type Output = $out;

            fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool) {
                let device = self.x.x.device;
                let fb = self.x.x.fb;

                match device {
                    XC2Device::XC2C32
                    | XC2Device::XC2C32A
                    | XC2Device::XC2C64
                    | XC2Device::XC2C64A => unreachable!(),
                    XC2Device::XC2C256 => (
                        {
                            let c = Coordinate::new(0, self.x.mc as usize * xc2c256_macrocell::H)
                                + xc2c256_macrocell::$spreadsheet[biti];
                            if fb % 2 == 0 {
                                device.fb_corner(fb) + c
                            } else {
                                device.fb_corner(fb).sub_x_add_y(c)
                            }
                        },
                        $invert,
                    ),
                    XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => (
                        {
                            let c = Coordinate::new(
                                0,
                                crate::mc::BIG_MC_STARTING_ROW[self.x.mc as usize],
                            ) + big_macrocell::$spreadsheet[biti];
                            if fb % 2 == 0 {
                                device.fb_corner(fb) + c
                            } else {
                                device.fb_corner(fb).sub_x_add_y(c)
                            }
                        },
                        $invert,
                    ),
                }
            }
        }
        #[cfg(feature = "alloc")]
        impl PropertyAccessorWithStringConv for $name {}
    };
}

declare_accessor_big_only!(InputModeAccessor, 2, InputBufMode, INPUT_BUF_MODE);
declare_accessor_big_only!(DataGateAccessor, 1, bool, DATA_GATE);
