//! Global bits

use bittwiddler_core::prelude::*;
use bittwiddler_macros::bittwiddler_hierarchy_level;

use crate::{global_fuses::GlobalFuses, partdb::XC2Device};

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GCKEn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gck_idx: u8,
}
impl PropertyAccessor for GCKEn {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gck()[self.gck_idx as usize], false)
    }
}
impl PropertyAccessorWithDefault for GCKEn {}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GCKEn {}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GSREn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
impl PropertyAccessor for GSREn {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gsr_enable(), false)
    }
}
impl PropertyAccessorWithDefault for GSREn {}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GSREn {}
#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GSRInv {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
}
impl PropertyAccessor for GSRInv {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gsr_invert(), false)
    }
}
impl PropertyAccessorWithDefault for GSRInv {}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GSRInv {}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GTSEn {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gts_idx: u8,
}
impl PropertyAccessor for GTSEn {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gts_enable()[self.gts_idx as usize], true)
    }
}
impl PropertyAccessorWithDefault for GTSEn {}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GTSEn {}
#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GTSInv {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gts_idx: u8,
}
impl PropertyAccessor for GTSInv {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gts_invert()[self.gts_idx as usize], false)
    }
}
impl PropertyAccessorWithDefault for GTSInv {
    fn is_at_default(&self, bitstream: &(impl BitArray + ?Sized)) -> bool {
        let val = self.get(bitstream);
        val == true
    }
}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for GTSInv {}

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
