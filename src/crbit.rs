//! Contains routines for dealing with xc2bit's "native" crbit format.
//!
//! This format is a "raw" dump of bits in the order that they'd be shifted into JTAG.
//! (Ancient comments claim that this format is intended to be compatible with `$readmemb`, but TODO VERIFY)
//! TODO: Document this format formally.

extern crate std;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::str::Utf8Error;
use std::vec::Vec;

use bitvec::prelude::*;

use crate::bitstream::{BitHolder, Coolrunner2};
use crate::partdb::{XC2Device, XC2Part};

#[derive(Debug)]
pub enum CrbitFormatError {
    InvalidCharacter,
    NoData,
    InvalidPartName,
    InvalidBitCount,
}
impl Display for CrbitFormatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CrbitFormatError::InvalidCharacter => write!(f, "invalid character in crbit"),
            CrbitFormatError::NoData => write!(f, "crbit contained no data"),
            CrbitFormatError::InvalidPartName => write!(f, "invalid part specified"),
            CrbitFormatError::InvalidBitCount => write!(f, "wrong number of bits for device"),
        }
    }
}
impl Error for CrbitFormatError {}

#[derive(Debug)]
pub enum CrbitReadError {
    IoError(io::Error),
    FormatError(CrbitFormatError),
    Utf8Error(Utf8Error),
}
impl Display for CrbitReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CrbitReadError::IoError(e) => e.fmt(f),
            CrbitReadError::FormatError(e) => e.fmt(f),
            CrbitReadError::Utf8Error(e) => e.fmt(f),
        }
    }
}
impl Error for CrbitReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CrbitReadError::IoError(e) => Some(e),
            CrbitReadError::FormatError(e) => Some(e),
            CrbitReadError::Utf8Error(e) => Some(e),
        }
    }
}
impl From<io::Error> for CrbitReadError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<CrbitFormatError> for CrbitReadError {
    fn from(value: CrbitFormatError) -> Self {
        Self::FormatError(value)
    }
}
impl From<Utf8Error> for CrbitReadError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

pub trait CrbitReader {
    fn read_crbit<R: io::Read>(r: R) -> Result<Self, CrbitReadError>
    where
        Self: Sized;
}
impl CrbitReader for Coolrunner2<BitBox> {
    fn read_crbit<R: io::Read>(mut r: R) -> Result<Self, CrbitReadError>
    where
        Self: Sized,
    {
        let mut in_bytes = Vec::new();
        let mut w = None;
        let mut dev_name_str = None;

        r.read_to_end(&mut in_bytes)?;

        // This capacity is approximate but close enough
        let mut bv = BitVec::with_capacity(in_bytes.len());

        let in_str = std::str::from_utf8(&in_bytes)?;

        for l in in_str.split('\n') {
            let l = l.trim_matches(|c| c == ' ' || c == '\r' || c == '\n');
            if l.len() == 0 {
                // ignore empty lines
                continue;
            }

            if l.starts_with("// DEVICE ") {
                dev_name_str = Some(&l["// DEVICE ".len()..]);
            } else if !l.starts_with("//") {
                // not a comment
                if w.is_none() {
                    w = Some(l.len());
                }

                for c in l.chars() {
                    match c {
                        '0' => bv.push(false),
                        '1' => bv.push(true),
                        _ => return Err(CrbitFormatError::InvalidCharacter.into()),
                    }
                }
            }
        }

        let w = w.ok_or(CrbitFormatError::NoData)?;

        let part = if let Some(dev_name_str) = dev_name_str {
            let part: XC2Part = dev_name_str
                .try_into()
                .map_err(|_| CrbitFormatError::InvalidPartName)?;

            let expected_dims = part.device.fuse_array_dims();
            if expected_dims.0 != w || bv.len() != expected_dims.0 * expected_dims.1 {
                return Err(CrbitFormatError::InvalidBitCount.into());
            }

            part
        } else {
            // guess part from bit count
            let device = if w == 260 && bv.len() == 260 * 50 {
                XC2Device::XC2C32A
            } else if w == 274 && bv.len() == 274 * 98 {
                XC2Device::XC2C64A
            } else if w == 752 && bv.len() == 752 * 82 {
                XC2Device::XC2C128
            } else if w == 1364 && bv.len() == 1364 * 98 {
                XC2Device::XC2C256
            } else if w == 1868 && bv.len() == 1868 * 122 {
                XC2Device::XC2C384
            } else if w == 1980 && bv.len() == 1980 * 162 {
                XC2Device::XC2C512
            } else {
                return Err(CrbitFormatError::InvalidBitCount.into());
            };

            XC2Part::new(device, None, None).unwrap()
        };

        Ok(Coolrunner2 {
            part,
            bits: bv.into_boxed_bitslice(),
        })
    }
}

pub trait CrbitWriter {
    fn write_crbit<W: io::Write>(&self, w: W) -> io::Result<()>;
}
impl<B: BitHolder> CrbitWriter for Coolrunner2<B> {
    fn write_crbit<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        write!(writer, "// crbit native bitstream file written by xc2bit\n")?;
        write!(writer, "// https://github.com/ArcaneNibble/xc2bit\n\n")?;

        write!(
            writer,
            "// DEVICE {}\n\n",
            std::string::ToString::to_string(&self.part)
        )?;

        let (w, h) = self.part.device.fuse_array_dims();
        for y in 0..h {
            for x in 0..w {
                if self.bits.get(y * w + x) {
                    write!(writer, "1")?;
                } else {
                    write!(writer, "0")?;
                }
            }
            write!(writer, "\n")?;
        }
        write!(writer, "\n")?;

        Ok(())
    }
}
