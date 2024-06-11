//! Xilinx ISE JEDEC programming file compatibility

use bittwiddler_core::prelude::Coordinate;

use crate::{
    global_fuses::{
        GlobalFuses, XC2C32_EXTRA_IBUF_SCHMITT_TRIGGER, XC2C32_EXTRA_IBUF_TERMINATION,
        XC2C32_IVOLTAGE, XC2C32_OVOLTAGE,
    },
    partdb::XC2Device,
};

pub trait JedecCompat {
    fn num_jed_fuses(&self) -> usize;
    fn jed_index_to_crbit(&self, jed_idx: usize) -> Coordinate;
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
                if jed_idx < 320 {
                    // FB1 ZIA
                    let zia_row = jed_idx / 8;
                    let zia_offs = jed_idx % 8;

                    let x = 122 + (7 - zia_offs) * 2;
                    let y = zia_row + if zia_row >= 20 { 8 } else { 0 };

                    Coordinate::new(x, y)
                } else if jed_idx >= 6128 && jed_idx < 6448 {
                    // FB2 ZIA
                    let offs = jed_idx - 6128;
                    let zia_row = offs / 8;
                    let zia_offs = offs % 8;

                    let x = 123 + (7 - zia_offs) * 2;
                    let y = zia_row + if zia_row >= 20 { 8 } else { 0 };

                    Coordinate::new(x, y)
                } else if jed_idx >= 320 && jed_idx < 4800 {
                    // FB1 AND array
                    let offs = jed_idx - 320;
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x = 10 + (1 - offs_80 % 2) + group_80 * 2;
                    let y = offs_80 / 2 + if offs_80 >= 40 { 8 } else { 0 };

                    Coordinate::new(x, y)
                } else if jed_idx >= 6448 && jed_idx < 10928 {
                    // FB2 AND array
                    let offs = jed_idx - 6448;
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x = 249 - (1 - offs_80 % 2) - group_80 * 2;
                    let y = offs_80 / 2 + if offs_80 >= 40 { 8 } else { 0 };

                    Coordinate::new(x, y)
                } else if jed_idx >= 4800 && jed_idx < 5696 {
                    // FB1 OR array
                    let offs = jed_idx - 4800;
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let x = 10 + offs_16 % 2 + group_16 * 2;
                    let y = 20 + offs_16 / 2;

                    Coordinate::new(x, y)
                } else if jed_idx >= 10928 && jed_idx < 11824 {
                    // FB2 OR array
                    let offs = jed_idx - 10928;
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let x = 249 - offs_16 % 2 - group_16 * 2;
                    let y = 20 + offs_16 / 2;

                    Coordinate::new(x, y)
                } else if jed_idx >= 5696 && jed_idx < 6128 {
                    // left side macrocells
                    let offs = jed_idx - 5696;
                    let x_bit = offs % 9;
                    let y_bit = offs / 9;
                    Coordinate::new(1 + x_bit, y_bit)
                } else if jed_idx >= 11824 && jed_idx < 12256 {
                    // right side macrocells
                    let offs = jed_idx - 11824;
                    let x_bit = offs % 9;
                    let y_bit = offs / 9;
                    Coordinate::new(258 - x_bit, y_bit)
                } else if jed_idx >= 12256 && jed_idx < 12259 {
                    // GCK
                    self.gck()[jed_idx - 12256]
                } else if jed_idx >= 12259 && jed_idx < 12261 {
                    // GSR
                    if jed_idx == 12259 {
                        self.gsr_invert()
                    } else {
                        self.gsr_enable()
                    }
                } else if jed_idx >= 12261 && jed_idx < 12269 {
                    // GTS
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
            XC2Device::XC2C64 | XC2Device::XC2C64A => todo!(),
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
                            if jed_idx >= 1 && jed_idx <= 30 && x >= 219 && x < 249 && y == 49 {
                                continue;
                            }
                            // println!("checking ({}, {}) = {}", x, y, jed_idx);
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
}
