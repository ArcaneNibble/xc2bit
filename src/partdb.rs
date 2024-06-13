//! Database of valid part/package/speed combinations

use core::fmt::{self, Debug, Display};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use core::fmt::Write;

use bittwiddler_core::prelude::Coordinate;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::zia::{table as zia_table, ZIATableEntry};

/// Coolrunner-II devices
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum XC2Device {
    XC2C32,
    XC2C32A,
    XC2C64,
    XC2C64A,
    XC2C128,
    XC2C256,
    XC2C384,
    XC2C512,
}
impl fmt::Display for XC2Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TryFrom<&str> for XC2Device {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        if value.eq_ignore_ascii_case("xc2c32") {
            Ok(Self::XC2C32)
        } else if value.eq_ignore_ascii_case("xc2c32a") {
            Ok(Self::XC2C32A)
        } else if value.eq_ignore_ascii_case("xc2c64") {
            Ok(Self::XC2C64)
        } else if value.eq_ignore_ascii_case("xc2c64a") {
            Ok(Self::XC2C64A)
        } else if value.eq_ignore_ascii_case("xc2c128") {
            Ok(Self::XC2C128)
        } else if value.eq_ignore_ascii_case("xc2c256") {
            Ok(Self::XC2C256)
        } else if value.eq_ignore_ascii_case("xc2c384") {
            Ok(Self::XC2C384)
        } else if value.eq_ignore_ascii_case("xc2c512") {
            Ok(Self::XC2C512)
        } else {
            Err(())
        }
    }
}
impl XC2Device {
    /// Dimensions (W, H) of the physical fusemap
    pub const fn fuse_array_dims(self) -> (usize, usize) {
        match self {
            Self::XC2C32 | Self::XC2C32A => (260, 50),
            Self::XC2C64 | Self::XC2C64A => (274, 98),
            Self::XC2C128 => (752, 82),
            Self::XC2C256 => (1364, 98),
            Self::XC2C384 => (1868, 122),
            Self::XC2C512 => (1980, 162),
        }
    }

    pub fn has_io_at(&self, fb: u8, mc: u8) -> bool {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => true,
            XC2Device::XC2C64 | XC2Device::XC2C64A => true,
            XC2Device::XC2C128 => match fb {
                0 | 1 | 5 | 7 => !(6..10).contains(&mc),
                2 | 3 | 4 | 6 => !(7..10).contains(&mc),
                _ => unreachable!(),
            },
            XC2Device::XC2C256 => match fb {
                0 | 1 | 2 | 3 | 4 | 5 | 12 | 13 => !(6..11).contains(&mc),
                6 | 7 | 8 | 9 | 10 | 11 | 14 | 15 => !(6..10).contains(&mc),
                _ => unreachable!(),
            },
            XC2Device::XC2C384 => !(5..11).contains(&mc),
            XC2Device::XC2C512 => match fb {
                0 | 1 | 3 | 5 | 7 | 8 | 9 | 10 | 12 | 14 | 17 | 19 | 21 | 23 | 24 | 26 | 28
                | 30 => !(4..12).contains(&mc),
                2 | 4 | 6 | 11 | 13 | 15 | 16 | 18 | 20 | 22 | 25 | 27 | 29 | 31 => {
                    !(5..12).contains(&mc)
                }
                _ => unreachable!(),
            },
        }
    }

    pub const fn num_fbs(self) -> usize {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => 2,
            XC2Device::XC2C64 | XC2Device::XC2C64A => 4,
            XC2Device::XC2C128 => 8,
            XC2Device::XC2C256 => 16,
            XC2Device::XC2C384 => 24,
            XC2Device::XC2C512 => 32,
        }
    }

    pub const fn zia_width(self) -> usize {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => 8,
            XC2Device::XC2C64 | XC2Device::XC2C64A => 16,
            XC2Device::XC2C128 => 28,
            XC2Device::XC2C256 => 48,
            XC2Device::XC2C384 => 74,
            XC2Device::XC2C512 => 88,
        }
    }

    pub fn zia_table_get_row(self, row: u8) -> &'static [ZIATableEntry] {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => &zia_table::ZIA_MAP_32[row as usize],
            XC2Device::XC2C64 | XC2Device::XC2C64A => &zia_table::ZIA_MAP_64[row as usize],
            XC2Device::XC2C128 => &zia_table::ZIA_MAP_128[row as usize],
            XC2Device::XC2C256 => &zia_table::ZIA_MAP_256[row as usize],
            XC2Device::XC2C384 => &zia_table::ZIA_MAP_384[row as usize],
            XC2Device::XC2C512 => &zia_table::ZIA_MAP_512[row as usize],
        }
    }

    pub const fn num_zia_choices(self) -> usize {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => 6,
            XC2Device::XC2C64 | XC2Device::XC2C64A => 12,
            XC2Device::XC2C128 => 22,
            XC2Device::XC2C256 => 40,
            XC2Device::XC2C384 => 62,
            XC2Device::XC2C512 => 78,
        }
    }

    pub const fn fb_corner(self, fb: u8) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => match fb {
                0 => Coordinate::new(1, 0),
                1 => Coordinate::new(258, 0),
                _ => unreachable!(),
            },
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                let x = match fb {
                    0 | 2 => 0,
                    1 | 3 => 273,
                    _ => unreachable!(),
                };
                let y = match fb {
                    0 | 1 => 0,
                    2 | 3 => 48,
                    _ => unreachable!(),
                };
                Coordinate::new(x, y)
            }
            XC2Device::XC2C128 => {
                let x = match fb {
                    0 | 2 => 1,
                    1 | 3 => 374,
                    4 | 6 => 377,
                    5 | 7 => 750,
                    _ => unreachable!(),
                };
                let y = match fb {
                    0 | 1 | 4 | 5 => 0,
                    2 | 3 | 6 | 7 => 40,
                    _ => unreachable!(),
                };
                Coordinate::new(x, y)
            }
            XC2Device::XC2C256 => {
                let x = match fb {
                    0 | 2 => 1,
                    1 | 3 => 340,
                    4 | 6 => 341,
                    5 | 7 => 680,
                    8 | 10 => 683,
                    9 | 11 => 1022,
                    12 | 14 => 1023,
                    13 | 15 => 1362,
                    _ => unreachable!(),
                };
                let y = match fb {
                    0 | 1 | 4 | 5 | 8 | 9 | 12 | 13 => 0,
                    2 | 3 | 6 | 7 | 10 | 11 | 14 | 15 => 48,
                    _ => unreachable!(),
                };
                Coordinate::new(x, y)
            }
            XC2Device::XC2C384 => {
                let x = match fb {
                    0 | 2 | 4 => 1,
                    1 | 3 | 5 => 466,
                    6 | 8 | 10 => 467,
                    7 | 9 | 11 => 932,
                    12 | 14 | 16 => 935,
                    13 | 15 | 17 => 1400,
                    18 | 20 | 22 => 1401,
                    19 | 21 | 23 => 1866,
                    _ => unreachable!(),
                };
                let y = match fb {
                    0 | 1 | 6 | 7 | 12 | 13 | 18 | 19 => 0,
                    2 | 3 | 8 | 9 | 14 | 15 | 20 | 21 => 40,
                    4 | 5 | 10 | 11 | 16 | 17 | 22 | 23 => 80,
                    _ => unreachable!(),
                };
                Coordinate::new(x, y)
            }
            XC2Device::XC2C512 => {
                let x = match fb {
                    0 | 2 | 4 | 6 => 1,
                    1 | 3 | 5 | 7 => 494,
                    8 | 10 | 12 | 14 => 495,
                    9 | 11 | 13 | 15 => 988,
                    16 | 18 | 20 | 22 => 991,
                    17 | 19 | 21 | 23 => 1484,
                    24 | 26 | 28 | 30 => 1485,
                    25 | 27 | 29 | 31 => 1978,
                    _ => unreachable!(),
                };
                let y = match fb {
                    0 | 1 | 8 | 9 | 16 | 17 | 24 | 25 => 0,
                    2 | 3 | 10 | 11 | 18 | 19 | 26 | 27 => 40,
                    4 | 5 | 12 | 13 | 20 | 21 | 28 | 29 => 80,
                    6 | 7 | 14 | 15 | 22 | 23 | 30 | 31 => 120,
                    _ => unreachable!(),
                };
                Coordinate::new(x, y)
            }
        }
    }

    pub const fn has_large_macrocells(self) -> bool {
        if let XC2Device::XC2C128 | XC2Device::XC2C256 | XC2Device::XC2C384 | XC2Device::XC2C512 =
            self
        {
            true
        } else {
            false
        }
    }
}

/// All possible speed grades
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpeedGrade {
    _4,
    _5,
    _6,
    _7,
    _10,
}
impl fmt::Display for SpeedGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::_4 => write!(f, "4"),
            Self::_5 => write!(f, "5"),
            Self::_6 => write!(f, "6"),
            Self::_7 => write!(f, "7"),
            Self::_10 => write!(f, "10"),
        }
    }
}
impl TryFrom<&str> for SpeedGrade {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "4" => Ok(Self::_4),
            "5" => Ok(Self::_5),
            "6" => Ok(Self::_6),
            "7" => Ok(Self::_7),
            "10" => Ok(Self::_10),
            _ => Err(()),
        }
    }
}

/// All possible physical packages, not including Pb-free-ness
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhysicalPackageShape {
    QF32,
    PC44,
    VQ44,
    QF48,
    CP56,
    VQ100,
    CP132,
    TQ144,
    PQ208,
    FT256,
    FG324,
}
impl fmt::Display for PhysicalPackageShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// All possible physical packages
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalPackage {
    pub shape: PhysicalPackageShape,
    pub pbfree: bool,
}
impl TryFrom<&str> for PhysicalPackage {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        let pkg_code = value.get(0..2).ok_or(())?;
        let maybe_pbfree = value.get(2..3).ok_or(())?;
        let (pbfree, pins_str) = if maybe_pbfree.eq_ignore_ascii_case("g") {
            (true, value.get(3..).ok_or(())?)
        } else {
            (false, value.get(2..).ok_or(())?)
        };
        if pkg_code.eq_ignore_ascii_case("pc") && pins_str == "44" {
            Ok(Self {
                shape: PhysicalPackageShape::PC44,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("qf") && pins_str == "32" {
            Ok(Self {
                shape: PhysicalPackageShape::QF32,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("vq") && pins_str == "44" {
            Ok(Self {
                shape: PhysicalPackageShape::VQ44,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("qf") && pins_str == "48" {
            Ok(Self {
                shape: PhysicalPackageShape::QF48,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("cp") && pins_str == "56" {
            Ok(Self {
                shape: PhysicalPackageShape::CP56,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("vq") && pins_str == "100" {
            Ok(Self {
                shape: PhysicalPackageShape::VQ100,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("cp") && pins_str == "132" {
            Ok(Self {
                shape: PhysicalPackageShape::CP132,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("tq") && pins_str == "144" {
            Ok(Self {
                shape: PhysicalPackageShape::TQ144,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("pq") && pins_str == "208" {
            Ok(Self {
                shape: PhysicalPackageShape::PQ208,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("ft") && pins_str == "256" {
            Ok(Self {
                shape: PhysicalPackageShape::FT256,
                pbfree,
            })
        } else if pkg_code.eq_ignore_ascii_case("fg") && pins_str == "324" {
            Ok(Self {
                shape: PhysicalPackageShape::FG324,
                pbfree,
            })
        } else {
            Err(())
        }
    }
}
impl Display for PhysicalPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.shape {
            PhysicalPackageShape::QF32 => write!(f, "QF{}32", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::PC44 => write!(f, "PC{}44", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::VQ44 => write!(f, "VQ{}44", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::QF48 => write!(f, "QF{}48", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::CP56 => write!(f, "CP{}56", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::VQ100 => write!(f, "VQ{}100", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::CP132 => write!(f, "CP{}132", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::TQ144 => write!(f, "TQ{}144", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::PQ208 => write!(f, "PQ{}208", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::FT256 => write!(f, "FT{}256", if self.pbfree { "G" } else { "" }),
            PhysicalPackageShape::FG324 => write!(f, "FG{}324", if self.pbfree { "G" } else { "" }),
        }
    }
}

/// One specific part, i.e. device type, speed grade, and package all in one struct
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XC2Part {
    pub device: XC2Device,
    pub speed: Option<SpeedGrade>,
    pub package: Option<PhysicalPackage>,
}
impl XC2Part {
    /// Determine if the given combination of device, speed, and package is a legal combination or not
    ///
    /// Note that Pb-free-ness isn't checked, and that this is in general only done on a best-effort basis
    /// (erring on the side of being more permissive)
    pub fn new(
        device: XC2Device,
        speed: Option<SpeedGrade>,
        package: Option<PhysicalPackage>,
    ) -> Option<Self> {
        let valid_combination = match device {
            XC2Device::XC2C32 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_4 || speed == SpeedGrade::_6
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::PC44
                        || package.shape == PhysicalPackageShape::VQ44
                        || package.shape == PhysicalPackageShape::CP56
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C32A => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_4 || speed == SpeedGrade::_6
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::PC44
                        || package.shape == PhysicalPackageShape::VQ44
                        || package.shape == PhysicalPackageShape::CP56
                        || package.shape == PhysicalPackageShape::QF32
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C64 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_5 || speed == SpeedGrade::_7
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::PC44
                        || package.shape == PhysicalPackageShape::VQ44
                        || package.shape == PhysicalPackageShape::CP56
                        || package.shape == PhysicalPackageShape::VQ100
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C64A => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_5 || speed == SpeedGrade::_7
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::PC44
                        || package.shape == PhysicalPackageShape::VQ44
                        || package.shape == PhysicalPackageShape::CP56
                        || package.shape == PhysicalPackageShape::VQ100
                        || package.shape == PhysicalPackageShape::QF48
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C128 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_6 || speed == SpeedGrade::_7
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::VQ100
                        || package.shape == PhysicalPackageShape::CP132
                        || package.shape == PhysicalPackageShape::TQ144
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C256 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_6 || speed == SpeedGrade::_7
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::VQ100
                        || package.shape == PhysicalPackageShape::CP132
                        || package.shape == PhysicalPackageShape::TQ144
                        || package.shape == PhysicalPackageShape::PQ208
                        || package.shape == PhysicalPackageShape::FT256
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C384 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_7 || speed == SpeedGrade::_10
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::TQ144
                        || package.shape == PhysicalPackageShape::PQ208
                        || package.shape == PhysicalPackageShape::FT256
                        || package.shape == PhysicalPackageShape::FG324
                });
                valid_speed && valid_package
            }
            XC2Device::XC2C512 => {
                let valid_speed = speed.map_or(true, |speed| {
                    speed == SpeedGrade::_7 || speed == SpeedGrade::_10
                });
                let valid_package = package.map_or(true, |package| {
                    package.shape == PhysicalPackageShape::PQ208
                        || package.shape == PhysicalPackageShape::FT256
                        || package.shape == PhysicalPackageShape::FG324
                });
                valid_speed && valid_package
            }
        };

        if valid_combination {
            Some(Self {
                device,
                speed,
                package,
            })
        } else {
            None
        }
    }
}
impl TryFrom<&str> for XC2Part {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        let mut split = value.split('-');
        let dev_str = split.next().ok_or(())?;
        let spd_str = split.next().ok_or(())?;
        let pkg_str = split.next().ok_or(())?;
        if split.next().is_some() {
            return Err(());
        }

        let device = dev_str.try_into()?;
        let speed = if spd_str.eq_ignore_ascii_case("unknown") {
            None
        } else {
            Some(spd_str.try_into()?)
        };
        let package = if pkg_str.eq_ignore_ascii_case("unknown") {
            None
        } else {
            Some(pkg_str.try_into()?)
        };

        Self::new(device, speed, package).ok_or(())
    }
}
#[cfg(feature = "alloc")]
impl alloc::string::ToString for XC2Part {
    fn to_string(&self) -> alloc::string::String {
        let mut str = alloc::string::String::new();
        write!(&mut str, "{}-", self.device).unwrap();
        if let Some(spd) = self.speed {
            write!(&mut str, "{}-", spd).unwrap();
        } else {
            write!(&mut str, "UNKNOWN-").unwrap();
        }
        if let Some(pkg) = self.package {
            write!(&mut str, "{}", pkg).unwrap();
        } else {
            write!(&mut str, "UNKNOWN").unwrap();
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_test_valid_part() {
        assert_eq!(
            XC2Part::try_from("xc2c32a-6-vq44"),
            Ok(XC2Part {
                device: XC2Device::XC2C32A,
                speed: Some(SpeedGrade::_6),
                package: Some(PhysicalPackage {
                    shape: PhysicalPackageShape::VQ44,
                    pbfree: false
                })
            })
        );
        assert_eq!(
            XC2Part::try_from("XC2C128-7-VQG100"),
            Ok(XC2Part {
                device: XC2Device::XC2C128,
                speed: Some(SpeedGrade::_7),
                package: Some(PhysicalPackage {
                    shape: PhysicalPackageShape::VQ100,
                    pbfree: true
                })
            })
        );
        assert_eq!(
            XC2Part::try_from("XC2C32A-UNKNOWN-vqg44"),
            Ok(XC2Part {
                device: XC2Device::XC2C32A,
                speed: None,
                package: Some(PhysicalPackage {
                    shape: PhysicalPackageShape::VQ44,
                    pbfree: true
                })
            })
        );
    }

    #[test]
    fn spot_test_invalid_part() {
        assert_eq!(XC2Part::try_from("xc2c32a-1-vq44"), Err(()));
        assert_eq!(XC2Part::try_from("xc2c32a-5-vq100"), Err(()));
    }

    #[test]
    fn malformed_part_names() {
        assert_eq!(XC2Part::try_from("asdf"), Err(()));
        assert_eq!(XC2Part::try_from("xc2c32a-5-vq44-asdf"), Err(()));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn part_stringifying() {
        use alloc::string::ToString;

        assert_eq!(
            XC2Part::new(
                XC2Device::XC2C32A,
                Some(SpeedGrade::_4),
                Some(PhysicalPackage {
                    shape: PhysicalPackageShape::VQ44,
                    pbfree: true
                })
            )
            .unwrap()
            .to_string(),
            "XC2C32A-4-VQG44"
        );
        assert_eq!(
            XC2Part::new(
                XC2Device::XC2C32A,
                None,
                Some(PhysicalPackage {
                    shape: PhysicalPackageShape::VQ44,
                    pbfree: true
                })
            )
            .unwrap()
            .to_string(),
            "XC2C32A-UNKNOWN-VQG44"
        );
    }
}
