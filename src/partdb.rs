//! Database of valid part/package/speed combinations

use core::fmt::{self, Debug, Display};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use core::fmt::Write;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Coolrunner-II devices
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    pub fn fuse_array_dims(self) -> (usize, usize) {
        match self {
            Self::XC2C32 | Self::XC2C32A => (260, 50),
            Self::XC2C64 | Self::XC2C64A => (274, 98),
            Self::XC2C128 => (752, 82),
            Self::XC2C256 => (1364, 98),
            Self::XC2C384 => (1868, 122),
            Self::XC2C512 => (1980, 162),
        }
    }
}

/// All possible speed grades
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
