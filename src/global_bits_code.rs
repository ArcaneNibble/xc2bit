//! Global bits

use bittwiddler_core::prelude::*;
use bittwiddler_macros::bittwiddler_hierarchy_level;

use crate::{global_fuses::GlobalFuses, partdb::XC2Device};

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct GCK {
    #[bittwiddler::skip]
    pub(crate) device: XC2Device,
    pub(crate) gck_idx: u8,
}
impl PropertyAccessor for GCK {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (Coordinate, bool) {
        (self.device.gck()[self.gck_idx as usize], false)
    }
}
