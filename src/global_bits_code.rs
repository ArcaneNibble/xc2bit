//! Global bits

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{global_fuses::GlobalFuses, partdb::XC2Device};

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GCKEn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gck_idx: u8,
}
crate::bitstream::single_bool_impl!(GCKEn, self, {
    (self.device.gck()[self.gck_idx as usize], false)
});

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GSREn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
crate::bitstream::single_bool_impl!(GSREn, self, { (self.device.gsr_enable(), false) });
#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GSRInv {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
crate::bitstream::single_bool_impl!(GSRInv, self, { (self.device.gsr_invert(), false) });

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GTSEn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gts_idx: u8,
}
crate::bitstream::single_bool_impl!(GTSEn, self, {
    (self.device.gts_enable()[self.gts_idx as usize], true)
});
#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GTSInv {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gts_idx: u8,
}
crate::bitstream::single_bool_impl!(
    GTSInv,
    self,
    { (self.device.gts_invert()[self.gts_idx as usize], false) },
    nodefault
);
impl PropertyAccessorWithDefault for GTSInv {
    fn is_at_default(&self, bitstream: &(impl BitArray + ?Sized)) -> bool {
        let val = self.get(bitstream);
        val == true
    }
}

include!(concat!(env!("OUT_DIR"), "/global-term.rs"));

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GlobalTermAccessor {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
impl PropertyAccessor for GlobalTermAccessor {
    type BoolArray = [bool; 1];
    type Output = GlobalTermination;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.global_term(), true)
    }
}
impl PropertyAccessorWithDefault for GlobalTermAccessor {}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GlobalTermAccessor {}

include!(concat!(env!("OUT_DIR"), "/clk-div.rs"));

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClockDivider {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl ClockDivider {
    #[bittwiddler::property]
    pub fn enabled(&self) -> ClkDivEnable {
        ClkDivEnable { x: *self }
    }
    #[bittwiddler::property]
    pub fn delay(&self) -> ClkDivDelay {
        ClkDivDelay { x: *self }
    }
    #[bittwiddler::property]
    pub fn ratio(&self) -> ClkDivRatioAccessor {
        ClkDivRatioAccessor { x: *self }
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct ClkDivEnable {
    x: ClockDivider,
}
crate::bitstream::single_bool_impl!(ClkDivEnable, self, {
    (self.x.device.clk_div_enable(), true)
});

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct ClkDivDelay {
    x: ClockDivider,
}
crate::bitstream::single_bool_impl!(ClkDivDelay, self, { (self.x.device.clk_div_delay(), true) });

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct ClkDivRatioAccessor {
    x: ClockDivider,
}
impl PropertyAccessor for ClkDivRatioAccessor {
    type BoolArray = [bool; 3];
    type Output = ClockDivRatio;

    fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool) {
        (self.x.device.clk_div_ratio()[biti], false)
    }
}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for ClkDivRatioAccessor {}
impl PropertyAccessorWithDefault for ClkDivRatioAccessor {}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DataGate {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
crate::bitstream::single_bool_impl!(DataGate, self, { (self.device.data_gate(), true) });

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UseVref {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
crate::bitstream::single_bool_impl!(UseVref, self, { (self.device.vref_enable(), true) });
