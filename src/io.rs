//! IO-related functions

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::mc;
use crate::{fb::FunctionBlock, partdb::XC2Device, spreadsheet_magic::xc2c32a_macrocell};

include!(concat!(env!("OUT_DIR"), "/io-fb.rs"));
include!(concat!(env!("OUT_DIR"), "/io-oe.rs"));
include!(concat!(env!("OUT_DIR"), "/io-regcom.rs"));
include!(concat!(env!("OUT_DIR"), "/io-slew.rs"));

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
}

// HACK
use IoPad as Macrocell;

mc::declare_accessor!(
    IoFeedbackSourceAccessor,
    2,
    IoFeedbackSource,
    IO_FEEDBACK_SRC
);
mc::declare_accessor!(SchmittTriggerAccessor, 1, bool, SCHMITT_TRIGGER);
mc::declare_accessor!(OutputSourceAccessor, 1, PinOutputSrc, REG_OR_COMB);
mc::declare_accessor!(OutputModeAccessor, 4, OutputMode, OUTPUT_BUF_MODE);
mc::declare_accessor!(TerminationAccessor, 1, bool, TERMINATION_ENABLED);
mc::declare_accessor!(SlewAccessor, 1, SlewRate, SLEW_RATE);
