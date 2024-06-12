//! Xilinx ISE JEDEC programming file compatibility

use bittwiddler_core::prelude::Coordinate;

use crate::{global_fuses::*, partdb::XC2Device};

const XC2C64_MACROCELL_PERMUTE: [Coordinate; 27] = [
    // row 0
    Coordinate::new(8, 0),
    Coordinate::new(7, 0),
    Coordinate::new(5, 0),
    Coordinate::new(6, 0),
    Coordinate::new(4, 0),
    Coordinate::new(2, 0),
    Coordinate::new(3, 0),
    Coordinate::new(0, 0),
    Coordinate::new(1, 0),
    // row 1 (mostly)
    Coordinate::new(7, 1),
    Coordinate::new(8, 1),
    Coordinate::new(5, 1),
    Coordinate::new(6, 1),
    Coordinate::new(3, 1),
    Coordinate::new(4, 1),
    Coordinate::new(2, 1),
    Coordinate::new(1, 1),
    Coordinate::new(7, 2),
    // row 2 (mostly)
    Coordinate::new(8, 2),
    Coordinate::new(0, 1),
    Coordinate::new(3, 2),
    Coordinate::new(4, 2),
    Coordinate::new(5, 2),
    Coordinate::new(6, 2),
    Coordinate::new(2, 2),
    Coordinate::new(1, 2),
    Coordinate::new(0, 2),
];

pub trait JedecCompat {
    fn num_jed_fuses(&self) -> usize;
    fn jed_index_to_crbit(&self, jed_idx: usize) -> Coordinate;

    fn _is_zia(&self, jed_idx: usize) -> Option<(usize, usize)>;
    fn _is_and(&self, jed_idx: usize) -> Option<(usize, usize)>;
    fn _is_or(&self, jed_idx: usize) -> Option<(usize, usize)>;
    fn _is_mc(&self, jed_idx: usize) -> Option<(usize, usize)>;
}
impl JedecCompat for XC2Device {
    fn num_jed_fuses(&self) -> usize {
        match self {
            XC2Device::XC2C32 => 12274,
            XC2Device::XC2C32A => 12278,
            XC2Device::XC2C64 => 25808,
            XC2Device::XC2C64A => 25812,
            XC2Device::XC2C128 => 55341,
            XC2Device::XC2C256 => 123249,
            XC2Device::XC2C384 => 209357,
            XC2Device::XC2C512 => 296403,
        }
    }

    fn jed_index_to_crbit(&self, jed_idx: usize) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if let Some((fb, offs)) = self._is_zia(jed_idx) {
                    let zia_row = offs / 8;
                    let zia_offs = offs % 8;

                    let x = 122 + (7 - zia_offs) * 2 + fb;
                    let y = zia_row + if zia_row >= 20 { 8 } else { 0 };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_and(jed_idx) {
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x_base = (1 - offs_80 % 2) + group_80 * 2;
                    let y = offs_80 / 2 + if offs_80 >= 40 { 8 } else { 0 };
                    let x = if fb == 0 { 10 + x_base } else { 249 - x_base };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_or(jed_idx) {
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let x_base = offs_16 % 2 + group_16 * 2;
                    let y = 20 + offs_16 / 2;
                    let x = if fb == 0 { 10 + x_base } else { 249 - x_base };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_mc(jed_idx) {
                    let x_bit = offs % 9;
                    let y_bit = offs / 9;
                    if fb == 0 {
                        Coordinate::new(1 + x_bit, y_bit)
                    } else {
                        Coordinate::new(258 - x_bit, y_bit)
                    }
                } else if (12256..12259).contains(&jed_idx) {
                    self.gck()[jed_idx - 12256]
                } else if (12259..12261).contains(&jed_idx) {
                    if jed_idx == 12259 {
                        self.gsr_invert()
                    } else {
                        self.gsr_enable()
                    }
                } else if (12261..12269).contains(&jed_idx) {
                    let offs = jed_idx - 12261;
                    let gts_idx = offs / 2;
                    if offs % 2 == 0 {
                        self.gts_invert()[gts_idx]
                    } else {
                        self.gts_enable()[gts_idx]
                    }
                } else if jed_idx == 12269 {
                    self.global_term()
                } else if jed_idx == 12270 {
                    XC2C32_OVOLTAGE
                } else if jed_idx == 12271 {
                    XC2C32_IVOLTAGE
                } else if jed_idx == 12272 {
                    XC2C32_EXTRA_IBUF_SCHMITT_TRIGGER
                } else if jed_idx == 12273 {
                    XC2C32_EXTRA_IBUF_TERMINATION
                } else if *self == XC2Device::XC2C32A && jed_idx == 12274 {
                    self.io_input_voltage()[0]
                } else if *self == XC2Device::XC2C32A && jed_idx == 12275 {
                    self.io_output_voltage()[0]
                } else if *self == XC2Device::XC2C32A && jed_idx == 12276 {
                    self.io_input_voltage()[1]
                } else if *self == XC2Device::XC2C32A && jed_idx == 12277 {
                    self.io_output_voltage()[1]
                } else {
                    unreachable!()
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if let Some((fb, offs)) = self._is_zia(jed_idx) {
                    let zia_row = offs / 16;
                    let zia_offs = offs % 16;

                    let x = 121 + (15 - zia_offs) * 2 + fb % 2;
                    let y = zia_row + if zia_row >= 20 { 8 } else { 0 } + (fb / 2) * 48;

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_and(jed_idx) {
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x_base = (1 - offs_80 % 2) + group_80 * 2;
                    let y = offs_80 / 2 + if offs_80 >= 40 { 8 } else { 0 } + (fb / 2) * 48;
                    let x = if fb % 2 == 0 {
                        9 + x_base
                    } else {
                        264 - x_base
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_or(jed_idx) {
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let x_base = offs_16 % 2 + group_16 * 2;
                    let y = 20 + offs_16 / 2 + (fb / 2) * 48;
                    let x = if fb % 2 == 0 {
                        9 + x_base
                    } else {
                        264 - x_base
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_mc(jed_idx) {
                    let mc = offs / 27;
                    let mc_offs = offs % 27;
                    let permute_c = XC2C64_MACROCELL_PERMUTE[mc_offs];

                    if fb % 2 == 0 {
                        Coordinate::new(0, mc * 3 + (fb / 2) * 48) + permute_c
                    } else {
                        Coordinate::new(273 - permute_c.x, mc * 3 + (fb / 2) * 48 + permute_c.y)
                    }
                } else if (25792..25795).contains(&jed_idx) {
                    self.gck()[jed_idx - 25792]
                } else if (25795..25797).contains(&jed_idx) {
                    if jed_idx == 25795 {
                        self.gsr_invert()
                    } else {
                        self.gsr_enable()
                    }
                } else if (25797..25805).contains(&jed_idx) {
                    let offs = jed_idx - 25797;
                    let gts_idx = offs / 2;
                    if offs % 2 == 0 {
                        self.gts_invert()[gts_idx]
                    } else {
                        self.gts_enable()[gts_idx]
                    }
                } else if jed_idx == 25805 {
                    self.global_term()
                } else if jed_idx == 25806 {
                    XC2C64_IVOLTAGE
                } else if jed_idx == 25807 {
                    XC2C64_OVOLTAGE
                } else if *self == XC2Device::XC2C64A && jed_idx == 25808 {
                    self.io_input_voltage()[0]
                } else if *self == XC2Device::XC2C64A && jed_idx == 25809 {
                    self.io_output_voltage()[0]
                } else if *self == XC2Device::XC2C64A && jed_idx == 25810 {
                    self.io_input_voltage()[1]
                } else if *self == XC2Device::XC2C64A && jed_idx == 25811 {
                    self.io_output_voltage()[1]
                } else {
                    unreachable!()
                }
            }
            XC2Device::XC2C128 => todo!(),
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => todo!(),
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_zia(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (0..320).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6128..6448).contains(&jed_idx) {
                    Some((1, jed_idx - 6128))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (0..640).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6448..7088).contains(&jed_idx) {
                    Some((1, jed_idx - 6448))
                } else if (12896..13536).contains(&jed_idx) {
                    Some((2, jed_idx - 12896))
                } else if (19344..19984).contains(&jed_idx) {
                    Some((3, jed_idx - 19344))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => todo!(),
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => todo!(),
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_and(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (320..4800).contains(&jed_idx) {
                    Some((0, jed_idx - 320))
                } else if (6448..10928).contains(&jed_idx) {
                    Some((1, jed_idx - 6448))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (640..5120).contains(&jed_idx) {
                    Some((0, jed_idx - 640))
                } else if (7088..11568).contains(&jed_idx) {
                    Some((1, jed_idx - 7088))
                } else if (13536..18016).contains(&jed_idx) {
                    Some((2, jed_idx - 13536))
                } else if (19984..24464).contains(&jed_idx) {
                    Some((3, jed_idx - 19984))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => todo!(),
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => todo!(),
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_or(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (4800..5696).contains(&jed_idx) {
                    Some((0, jed_idx - 4800))
                } else if (10928..11824).contains(&jed_idx) {
                    Some((1, jed_idx - 10928))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (5120..6016).contains(&jed_idx) {
                    Some((0, jed_idx - 5120))
                } else if (11568..12464).contains(&jed_idx) {
                    Some((1, jed_idx - 11568))
                } else if (18016..18912).contains(&jed_idx) {
                    Some((2, jed_idx - 18016))
                } else if (24464..25360).contains(&jed_idx) {
                    Some((3, jed_idx - 24464))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => todo!(),
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => todo!(),
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_mc(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (5696..6128).contains(&jed_idx) {
                    Some((0, jed_idx - 5696))
                } else if (11824..12256).contains(&jed_idx) {
                    Some((1, jed_idx - 11824))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (6016..6448).contains(&jed_idx) {
                    Some((0, jed_idx - 6016))
                } else if (12464..12896).contains(&jed_idx) {
                    Some((1, jed_idx - 12464))
                } else if (18912..19344).contains(&jed_idx) {
                    Some((2, jed_idx - 18912))
                } else if (25360..25792).contains(&jed_idx) {
                    Some((3, jed_idx - 25360))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => todo!(),
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => todo!(),
            XC2Device::XC2C512 => todo!(),
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    extern crate std;
    use std::path::PathBuf;
    use std::println;
    use std::vec::Vec;

    fn check_map(device: XC2Device, filename: PathBuf) {
        if let Ok(map_tsv) = std::fs::read_to_string(&filename) {
            let map_bits = map_tsv
                .lines()
                .map(|l| l.split('\t').collect::<Vec<_>>())
                .collect::<Vec<_>>();

            for (x, xi) in map_bits.iter().enumerate() {
                for (y, map_bit) in xi.iter().enumerate() {
                    if *map_bit != "" {
                        if let Ok(jed_idx) = usize::from_str_radix(map_bit, 10) {
                            // *sigh* hack
                            if device == XC2Device::XC2C32A
                                && jed_idx >= 1
                                && jed_idx <= 30
                                && x >= 219
                                && x < 249
                                && y == 49
                            {
                                continue;
                            }
                            if device == XC2Device::XC2C64A
                                && jed_idx >= 1
                                && jed_idx <= 30
                                && x >= 243
                                && x < 273
                                && y == 97
                            {
                                continue;
                            }
                            println!("checking ({}, {}) = {}", x, y, jed_idx);
                            let c = device.jed_index_to_crbit(jed_idx);
                            assert_eq!(c, Coordinate::new(x, y));
                        } else if *map_bit == "spare" {
                            // do nothing
                        } else {
                            println!("TODO: {}", map_bit);
                        }
                    }
                }
            }
        } else {
            println!("WARNING: Skipping test because {:?} not found", filename);
        }
    }

    #[test]
    fn check_jed_xc2c32() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c32.map");
        check_map(XC2Device::XC2C32, p);
    }

    #[test]
    fn check_jed_xc2c32a() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c32a.map");
        check_map(XC2Device::XC2C32A, p);
    }

    #[test]
    fn check_jed_xc2c64() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c64.map");
        check_map(XC2Device::XC2C64, p);
    }

    #[test]
    fn check_jed_xc2c64a() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c64a.map");
        check_map(XC2Device::XC2C64A, p);
    }

    #[test]
    fn check_jed_xc2c128() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c128.map");
        check_map(XC2Device::XC2C128, p);
    }
}
