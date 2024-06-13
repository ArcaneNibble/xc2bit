//! ZIA global interconnect

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;

use crate::{fb::FunctionBlock, partdb::XC2Device, ANDTERMS_PER_FB, MCS_PER_FB};

const F: bool = false;
const T: bool = true;

const ZIA_PATS_32: &[&[bool]] = &[
    &[F, T, T, T, T, T, T, F],
    &[F, T, T, T, T, T, F, T],
    &[F, T, T, T, T, F, T, T],
    &[F, T, T, T, F, T, T, T],
    &[F, T, T, F, T, T, T, T],
    &[F, T, F, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T],
];
const ZIA_PATS_64: &[&[bool]] = &[
    &[T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, F],
    &[T, T, T, T, T, T, T, F, T, T, T, T, F, T, F, T],
    &[T, T, T, T, T, T, T, F, T, T, T, T, F, F, T, T],
    &[T, T, T, T, T, T, T, F, T, T, T, F, F, T, T, T],
    &[T, T, T, T, T, T, T, F, T, T, F, T, F, T, T, T],
    &[T, T, T, T, T, T, T, F, T, F, T, T, F, T, T, T],
    &[T, T, T, F, T, T, F, F, T, T, T, T, T, T, T, T],
    &[T, T, T, F, T, F, T, F, T, T, T, T, T, T, T, T],
    &[T, T, T, F, F, T, T, F, T, T, T, T, T, T, T, T],
    &[T, T, F, F, T, T, T, F, T, T, T, T, T, T, T, T],
    &[T, F, T, F, T, T, T, F, T, T, T, T, T, T, T, T],
    &[F, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    // TODO: This one isn't certain
    &[T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T],
];
#[rustfmt::skip]
const ZIA_PATS_128: &[&[bool]] = &[
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, F, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, F, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, F, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, F, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, F, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    // TODO: This one isn't certain
    &[T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
];
#[rustfmt::skip]
const ZIA_PATS_256: &[&[bool]] = &[
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, F, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, F, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, F, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, F, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    // TODO: This one isn't certain
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T],
];
#[rustfmt::skip]
const ZIA_PATS_384: &[&[bool]] = &[
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, F, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    // TODO: This one isn't certain
    &[T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
];
#[rustfmt::skip]
const ZIA_PATS_512: &[&[bool]] = &[
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
    // TODO: This one isn't certain
    &[T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
];

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
        let mut bits = [false; 88];
        for biti in 0..self.x.device.zia_width() {
            let (c, inv) = self.get_bit_pos(biti);
            bits[biti] = bitstream.get(c) ^ inv;
        }

        let match_pats = match self.x.device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => ZIA_PATS_32,
            XC2Device::XC2C64 | XC2Device::XC2C64A => ZIA_PATS_64,
            XC2Device::XC2C128 => ZIA_PATS_128,
            XC2Device::XC2C256 => ZIA_PATS_256,
            XC2Device::XC2C384 => ZIA_PATS_384,
            XC2Device::XC2C512 => ZIA_PATS_512,
        };

        for zia_row_i in 0..(self.x.device.num_zia_choices() + 2) {
            if &bits[..self.x.device.zia_width()] == match_pats[zia_row_i] {
                if zia_row_i == self.x.device.num_zia_choices() {
                    return ZIARow::VCC;
                } else if zia_row_i == self.x.device.num_zia_choices() + 1 {
                    return ZIARow::GND;
                } else {
                    return ZIARow::MuxChoice(zia_row_i as u8);
                }
            }
        }

        ZIARow::Invalid(bits)
    }

    fn set(&self, bitstream: &mut (impl BitArray + ?Sized), val: Self::Output) {
        let match_pats = match self.x.device {
            XC2Device::XC2C32 | XC2Device::XC2C32A => ZIA_PATS_32,
            XC2Device::XC2C64 | XC2Device::XC2C64A => ZIA_PATS_64,
            XC2Device::XC2C128 => ZIA_PATS_128,
            XC2Device::XC2C256 => ZIA_PATS_256,
            XC2Device::XC2C384 => ZIA_PATS_384,
            XC2Device::XC2C512 => ZIA_PATS_512,
        };

        let bits = match val {
            ZIARow::VCC => match_pats[self.x.device.num_zia_choices()],
            ZIARow::GND => match_pats[self.x.device.num_zia_choices() + 1],
            ZIARow::MuxChoice(row) => match_pats[row as usize],
            ZIARow::Invalid(ref bits) => bits,
        };

        for biti in 0..self.x.device.zia_width() {
            let (c, inv) = self.get_bit_pos(biti);
            bitstream.set(c, bits[biti] ^ inv);
        }
    }
}
#[cfg(feature = "alloc")]
impl PropertyAccessorWithStringConv for ZIARowAccessor {}
impl PropertyAccessorWithDefault for ZIARowAccessor {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ZIARow {
    GND,
    VCC,
    MuxChoice(u8),
    Invalid([bool; 88]),
}
/// FAKE implementation which is bypassed with specialization in [ZIARowAccessor)
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
    fn to_string(&self, accessor: &ZIARowAccessor) -> alloc::borrow::Cow<'static, str> {
        let device = accessor.x.device;
        let zia_row = accessor.zia_row;
        match self {
            ZIARow::VCC => "1".into(),
            ZIARow::GND => "0".into(),
            ZIARow::MuxChoice(zia_choice) => {
                let zia_ent = device.zia_table_get_row(zia_row)[*zia_choice as usize];
                match zia_ent {
                    ZIATableEntry::Macrocell { fb, mc } => {
                        alloc::format!("Macrocell(FB{fb}, MC{mc})").into()
                    }
                    ZIATableEntry::InputPin { fb, mc } => {
                        alloc::format!("InputPin(FB{fb}, MC{mc})").into()
                    }
                    ZIATableEntry::DedicatedInputPin => "DedicatedInputPin".into(),
                }
            }
            ZIARow::Invalid(bits) => {
                let mut ret = alloc::string::String::from("Invalid(");
                for &bit in &bits[..device.zia_width()] {
                    if bit {
                        ret.push('1');
                    } else {
                        ret.push('0');
                    }
                }
                ret.push(')');
                ret.into()
            }
        }
    }

    fn from_string(s: &str, accessor: &ZIARowAccessor) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let device = accessor.x.device;

        let zia_choice = if s == "1" {
            return Ok(Self::VCC);
        } else if s == "0" {
            return Ok(Self::GND);
        } else if s == "DedicatedInputPin" {
            ZIATableEntry::DedicatedInputPin
        } else if let Some(s) = s.strip_prefix("Invalid(") {
            let mut bits = [false; 88];
            let mut chars = s.chars();
            for i in 0..device.zia_width() {
                let c = chars.next().ok_or(())?;
                if c == '1' {
                    bits[i] = true;
                } else if c == '0' {
                    bits[i] = false;
                } else {
                    return Err(());
                }
            }
            if chars.next().ok_or(())? != ')' {
                return Err(());
            }
            if chars.next().is_some() {
                return Err(());
            }
            return Ok(Self::Invalid(bits));
        } else if let Some(s) = s.strip_prefix("Macrocell(") {
            if let Some(s) = s.strip_suffix(")") {
                if let Some((fb, mc)) = s.split_once(',') {
                    let fb = fb.trim();
                    let mc = mc.trim();

                    let fb = if let Some(fb) = fb.strip_prefix("FB") {
                        fb.parse::<u8>().map_err(|_| ())?
                    } else {
                        return Err(());
                    };
                    let mc = if let Some(mc) = mc.strip_prefix("MC") {
                        mc.parse::<u8>().map_err(|_| ())?
                    } else {
                        return Err(());
                    };

                    ZIATableEntry::Macrocell { fb, mc }
                } else {
                    return Err(());
                }
            } else {
                return Err(());
            }
        } else if let Some(s) = s.strip_prefix("InputPin(") {
            if let Some(s) = s.strip_suffix(")") {
                if let Some((fb, mc)) = s.split_once(',') {
                    let fb = fb.trim();
                    let mc = mc.trim();

                    let fb = if let Some(fb) = fb.strip_prefix("FB") {
                        fb.parse::<u8>().map_err(|_| ())?
                    } else {
                        return Err(());
                    };
                    let mc = if let Some(mc) = mc.strip_prefix("MC") {
                        mc.parse::<u8>().map_err(|_| ())?
                    } else {
                        return Err(());
                    };

                    ZIATableEntry::InputPin { fb, mc }
                } else {
                    return Err(());
                }
            } else {
                return Err(());
            }
        } else {
            return Err(());
        };

        let zia_row = device.zia_table_get_row(accessor.zia_row);

        for zia_choice_i in 0..device.num_zia_choices() {
            if zia_row[zia_choice_i] == zia_choice {
                return Ok(Self::MuxChoice(zia_choice_i as u8));
            }
        }

        Err(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ZIATableEntry {
    Macrocell { fb: u8, mc: u8 },
    InputPin { fb: u8, mc: u8 },
    DedicatedInputPin,
}

pub mod table;
