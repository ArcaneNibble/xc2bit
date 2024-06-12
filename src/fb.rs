//! Function block

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{partdb::XC2Device, ANDTERMS_PER_FB, ZIA_ROWS};

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
        todo!()
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
        todo!()
    }
}
