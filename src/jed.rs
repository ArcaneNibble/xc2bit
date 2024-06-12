//! Xilinx ISE JEDEC programming file compatibility

use bittwiddler_core::prelude::Coordinate;

use crate::{bitstream::BuriedMacrocells, global_fuses::*, partdb::XC2Device};

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

const BIG_MC_WITH_IO_PERMUTE: [Coordinate; 29] = [
    // row 1 (mostly)
    Coordinate::new(8, 0),
    Coordinate::new(9, 0),
    Coordinate::new(10, 0),
    Coordinate::new(11, 0),
    Coordinate::new(12, 0),
    Coordinate::new(4, 0),
    Coordinate::new(2, 0),
    Coordinate::new(3, 0),
    Coordinate::new(5, 0),
    Coordinate::new(6, 0),
    Coordinate::new(13, 0),
    Coordinate::new(0, 0),
    Coordinate::new(1, 0),
    // row 2 (mostly)
    Coordinate::new(2, 1),
    Coordinate::new(3, 1),
    Coordinate::new(4, 1),
    Coordinate::new(5, 1),
    Coordinate::new(13, 1),
    Coordinate::new(14, 1),
    Coordinate::new(14, 0),
    Coordinate::new(8, 1),
    Coordinate::new(9, 1),
    Coordinate::new(10, 1),
    Coordinate::new(11, 1),
    Coordinate::new(12, 1),
    Coordinate::new(6, 1),
    Coordinate::new(7, 0),
    Coordinate::new(0, 1),
    Coordinate::new(1, 1),
];
const BIG_MC_NO_IO_PERMUTE: [Coordinate; 16] = [
    // row 1 (mostly)
    Coordinate::new(8, 0),
    Coordinate::new(9, 0),
    Coordinate::new(10, 0),
    Coordinate::new(11, 0),
    Coordinate::new(12, 0),
    // Coordinate::new(4, 0),
    Coordinate::new(2, 0),
    Coordinate::new(3, 0),
    // Coordinate::new(5, 0),
    // Coordinate::new(6, 0),
    // Coordinate::new(13, 0),
    // Coordinate::new(0, 0),
    // Coordinate::new(1, 0),
    // row 2 (mostly)
    // Coordinate::new(2, 1),
    // Coordinate::new(3, 1),
    // Coordinate::new(4, 1),
    // Coordinate::new(5, 1),
    Coordinate::new(13, 1),
    Coordinate::new(14, 1),
    Coordinate::new(14, 0),
    // Coordinate::new(8, 1),
    Coordinate::new(9, 1),
    Coordinate::new(10, 1),
    Coordinate::new(11, 1),
    Coordinate::new(12, 1),
    // Coordinate::new(6, 1),
    // Coordinate::new(7, 0),
    Coordinate::new(0, 1),
    Coordinate::new(1, 1),
];
const BIG_MC_STARTING_ROW: [usize; 16] =
    [0, 3, 5, 8, 10, 13, 15, 18, 20, 23, 25, 28, 30, 33, 35, 38];

const SIDE_OR_ROW_PERMUTE: [usize; 28] = [
    17, 19, 22, 20, 0, 1, 3, 4, 5, 7, 8, 11, 12, 13, 15, 16, 23, 24, 26, 27, 28, 31, 32, 34, 35,
    36, 38, 39,
];
#[rustfmt::skip]
const NOGAP_AND_TERM_PERMUTE: [usize; 56] = [
    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    // 55, 54, 53,
    // 11, 12, 13,
    // 52, 51, 50,
    // 14, 15, 16,
    // 49, 48, 47,
    // 17, 18, 19,
    // 46, 45, 44,
    // 20, 21, 22,
    // 43, 42, 41,
    // 23, 24, 25,
    // 40, 39, 38,
    // 26, 27, 28,
    // 37, 36, 35,
    // 29, 30, 31,
    // 34, 33, 32
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    14, 15, 16,
    20, 21, 22,
    26, 27, 28,
    32, 33, 34,
    38, 39, 40,
    44, 45, 46,
    50, 51, 52,
    55, 54, 53,
    49, 48, 47,
    43, 42, 41,
    37, 36, 35,
    31, 30, 29,
    25, 24, 23,
    19, 18, 17,
    13, 12, 11
];

fn get_fat_mc_idx(device: XC2Device, fb: usize, offs: usize) -> (usize, usize) {
    let mut accum_offs = 0;
    for mc in 0..16 {
        let old_accum_offs = accum_offs;
        if device.has_io_at(fb as u8, mc as u8) {
            accum_offs += 29;
        } else {
            accum_offs += 16;
        }

        if accum_offs > offs {
            return (mc, offs - old_accum_offs);
        }
    }

    unreachable!()
}

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
            XC2Device::XC2C128 => {
                if let Some((fb, offs)) = self._is_zia(jed_idx) {
                    let zia_row = offs / 28;
                    let zia_offs = offs % 28;

                    let x_base = (27 - zia_offs) * 2 + fb % 2;
                    let y = zia_row + if let 2 | 3 | 6 | 7 = fb { 40 } else { 0 };
                    let x = match fb {
                        0 | 1 | 2 | 3 => 160 + x_base,
                        4 | 5 | 6 | 7 => 536 + x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_and(jed_idx) {
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x_base = NOGAP_AND_TERM_PERMUTE[group_80] * 2 + 1 - offs_80 % 2;
                    let y = offs_80 / 2 + if let 2 | 3 | 6 | 7 = fb { 40 } else { 0 };
                    let x = match fb {
                        0 | 2 => 48 + x_base,
                        1 | 3 => 327 - x_base,
                        4 | 6 => 424 + x_base,
                        5 | 7 => 703 - x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_or(jed_idx) {
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let y_base = SIDE_OR_ROW_PERMUTE[group_16 / 2];
                    let x_base = offs_16 * 2
                        + if y_base >= 23 {
                            1 - group_16 % 2
                        } else {
                            group_16 % 2
                        };
                    let y = y_base + if let 2 | 3 | 6 | 7 = fb { 40 } else { 0 };
                    let x = match fb {
                        0 | 2 => 16 + x_base,
                        1 | 3 => 359 - x_base,
                        4 | 6 => 392 + x_base,
                        5 | 7 => 735 - x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_mc(jed_idx) {
                    let (mc, mc_offs) = get_fat_mc_idx(*self, fb, offs);
                    let permute_c = if self.has_io_at(fb as u8, mc as u8) {
                        BIG_MC_WITH_IO_PERMUTE[mc_offs]
                    } else {
                        BIG_MC_NO_IO_PERMUTE[mc_offs]
                    };
                    let y = BIG_MC_STARTING_ROW[mc]
                        + if let 2 | 3 | 6 | 7 = fb { 40 } else { 0 }
                        + permute_c.y;
                    let x = match fb {
                        0 | 2 => 1 + permute_c.x,
                        1 | 3 => 374 - permute_c.x,
                        4 | 6 => 377 + permute_c.x,
                        5 | 7 => 750 - permute_c.x,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if (55316..55319).contains(&jed_idx) {
                    self.gck()[jed_idx - 55316]
                } else if jed_idx == 55319 {
                    self.clk_div_enable()
                } else if (55320..55323).contains(&jed_idx) {
                    self.clk_div_ratio()[jed_idx - 55320]
                } else if jed_idx == 55323 {
                    self.clk_div_delay()
                } else if (55324..55326).contains(&jed_idx) {
                    if jed_idx == 55324 {
                        self.gsr_invert()
                    } else {
                        self.gsr_enable()
                    }
                } else if (55326..55334).contains(&jed_idx) {
                    let offs = jed_idx - 55326;
                    let gts_idx = offs / 2;
                    if offs % 2 == 0 {
                        self.gts_invert()[gts_idx]
                    } else {
                        self.gts_enable()[gts_idx]
                    }
                } else if jed_idx == 55334 {
                    self.global_term()
                } else if jed_idx == 55335 {
                    self.data_gate()
                } else if (55336..55338).contains(&jed_idx) {
                    self.io_input_voltage()[jed_idx - 55336]
                } else if (55338..55340).contains(&jed_idx) {
                    self.io_output_voltage()[jed_idx - 55338]
                } else if jed_idx == 55340 {
                    self.vref_enable()
                } else {
                    unreachable!()
                }
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                if let Some((fb, offs)) = self._is_zia(jed_idx) {
                    let zia_row = offs / 74;
                    let zia_offs = offs % 74;

                    let x_base = (73 - zia_offs) * 2 + fb % 2;
                    let y = zia_row
                        + match fb {
                            0 | 1 | 6 | 7 | 12 | 13 | 18 | 19 => 0,
                            2 | 3 | 8 | 9 | 14 | 15 | 20 | 21 => 40,
                            4 | 5 | 10 | 11 | 16 | 17 | 22 | 23 => 80,
                            _ => unreachable!(),
                        };
                    let x = match fb {
                        0 | 1 | 2 | 3 | 4 | 5 => 160 + x_base,
                        6 | 7 | 8 | 9 | 10 | 11 => 626 + x_base,
                        12 | 13 | 14 | 15 | 16 | 17 => 1094 + x_base,
                        18 | 19 | 20 | 21 | 22 | 23 => 1560 + x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_and(jed_idx) {
                    let group_80 = offs / 80;
                    let offs_80 = offs % 80;

                    let x_base = NOGAP_AND_TERM_PERMUTE[group_80] * 2 + 1 - offs_80 % 2;
                    let y = offs_80 / 2
                        + match fb {
                            0 | 1 | 6 | 7 | 12 | 13 | 18 | 19 => 0,
                            2 | 3 | 8 | 9 | 14 | 15 | 20 | 21 => 40,
                            4 | 5 | 10 | 11 | 16 | 17 | 22 | 23 => 80,
                            _ => unreachable!(),
                        };
                    let x = match fb {
                        0 | 2 | 4 => 48 + x_base,
                        1 | 3 | 5 => 419 - x_base,
                        6 | 8 | 10 => 514 + x_base,
                        7 | 9 | 11 => 885 - x_base,
                        12 | 14 | 16 => 982 + x_base,
                        13 | 15 | 17 => 1353 - x_base,
                        18 | 20 | 22 => 1448 + x_base,
                        19 | 21 | 23 => 1819 - x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_or(jed_idx) {
                    let group_16 = offs / 16;
                    let offs_16 = offs % 16;

                    let y_base = SIDE_OR_ROW_PERMUTE[group_16 / 2];
                    let x_base = offs_16 * 2
                        + if y_base >= 23 {
                            1 - group_16 % 2
                        } else {
                            group_16 % 2
                        };
                    let y = y_base
                        + match fb {
                            0 | 1 | 6 | 7 | 12 | 13 | 18 | 19 => 0,
                            2 | 3 | 8 | 9 | 14 | 15 | 20 | 21 => 40,
                            4 | 5 | 10 | 11 | 16 | 17 | 22 | 23 => 80,
                            _ => unreachable!(),
                        };
                    let x = match fb {
                        0 | 2 | 4 => 16 + x_base,
                        1 | 3 | 5 => 451 - x_base,
                        6 | 8 | 10 => 482 + x_base,
                        7 | 9 | 11 => 917 - x_base,
                        12 | 14 | 16 => 950 + x_base,
                        13 | 15 | 17 => 1385 - x_base,
                        18 | 20 | 22 => 1416 + x_base,
                        19 | 21 | 23 => 1851 - x_base,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if let Some((fb, offs)) = self._is_mc(jed_idx) {
                    let (mc, mc_offs) = get_fat_mc_idx(*self, fb, offs);
                    let permute_c = if self.has_io_at(fb as u8, mc as u8) {
                        BIG_MC_WITH_IO_PERMUTE[mc_offs]
                    } else {
                        BIG_MC_NO_IO_PERMUTE[mc_offs]
                    };
                    let y = BIG_MC_STARTING_ROW[mc]
                        + match fb {
                            0 | 1 | 6 | 7 | 12 | 13 | 18 | 19 => 0,
                            2 | 3 | 8 | 9 | 14 | 15 | 20 | 21 => 40,
                            4 | 5 | 10 | 11 | 16 | 17 | 22 | 23 => 80,
                            _ => unreachable!(),
                        }
                        + permute_c.y;
                    let x = match fb {
                        0 | 2 | 4 => 1 + permute_c.x,
                        1 | 3 | 5 => 466 - permute_c.x,
                        6 | 8 | 10 => 467 + permute_c.x,
                        7 | 9 | 11 => 932 - permute_c.x,
                        12 | 14 | 16 => 935 + permute_c.x,
                        13 | 15 | 17 => 1400 - permute_c.x,
                        18 | 20 | 22 => 1401 + permute_c.x,
                        19 | 21 | 23 => 1866 - permute_c.x,
                        _ => unreachable!(),
                    };

                    Coordinate::new(x, y)
                } else if (209328..209331).contains(&jed_idx) {
                    self.gck()[jed_idx - 209328]
                } else if jed_idx == 209331 {
                    self.clk_div_enable()
                } else if (209332..209335).contains(&jed_idx) {
                    self.clk_div_ratio()[jed_idx - 209332]
                } else if jed_idx == 209335 {
                    self.clk_div_delay()
                } else if (209336..209338).contains(&jed_idx) {
                    if jed_idx == 209336 {
                        self.gsr_invert()
                    } else {
                        self.gsr_enable()
                    }
                } else if (209338..209346).contains(&jed_idx) {
                    let offs = jed_idx - 209338;
                    let gts_idx = offs / 2;
                    if offs % 2 == 0 {
                        self.gts_invert()[gts_idx]
                    } else {
                        self.gts_enable()[gts_idx]
                    }
                } else if jed_idx == 209346 {
                    self.global_term()
                } else if jed_idx == 209347 {
                    self.data_gate()
                } else if (209348..209352).contains(&jed_idx) {
                    self.io_input_voltage()[jed_idx - 209348]
                } else if (209352..209356).contains(&jed_idx) {
                    self.io_output_voltage()[jed_idx - 209352]
                } else if jed_idx == 209356 {
                    self.vref_enable()
                } else {
                    unreachable!()
                }
            }
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_zia(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (0..8 * 40).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6128..6128 + 8 * 40).contains(&jed_idx) {
                    Some((1, jed_idx - 6128))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (0..16 * 40).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6448..6448 + 16 * 40).contains(&jed_idx) {
                    Some((1, jed_idx - 6448))
                } else if (12896..12896 + 16 * 40).contains(&jed_idx) {
                    Some((2, jed_idx - 12896))
                } else if (19344..19344 + 16 * 40).contains(&jed_idx) {
                    Some((3, jed_idx - 19344))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => {
                if (0..28 * 40).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6908..6908 + 28 * 40).contains(&jed_idx) {
                    Some((1, jed_idx - 6908))
                } else if (13816..13816 + 28 * 40).contains(&jed_idx) {
                    Some((2, jed_idx - 13816))
                } else if (20737..20737 + 28 * 40).contains(&jed_idx) {
                    Some((3, jed_idx - 20737))
                } else if (27658..27658 + 28 * 40).contains(&jed_idx) {
                    Some((4, jed_idx - 27658))
                } else if (34579..34579 + 28 * 40).contains(&jed_idx) {
                    Some((5, jed_idx - 34579))
                } else if (41487..41487 + 28 * 40).contains(&jed_idx) {
                    Some((6, jed_idx - 41487))
                } else if (48408..48408 + 28 * 40).contains(&jed_idx) {
                    Some((7, jed_idx - 48408))
                } else {
                    None
                }
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                if (0..74 * 40).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (8722..8722 + 74 * 40).contains(&jed_idx) {
                    Some((1, jed_idx - 8722))
                } else if (17444..17444 + 74 * 40).contains(&jed_idx) {
                    Some((2, jed_idx - 17444))
                } else if (26166..26166 + 74 * 40).contains(&jed_idx) {
                    Some((3, jed_idx - 26166))
                } else if (34888..34888 + 74 * 40).contains(&jed_idx) {
                    Some((4, jed_idx - 34888))
                } else if (43610..43610 + 74 * 40).contains(&jed_idx) {
                    Some((5, jed_idx - 43610))
                } else if (52332..52332 + 74 * 40).contains(&jed_idx) {
                    Some((6, jed_idx - 52332))
                } else if (61054..61054 + 74 * 40).contains(&jed_idx) {
                    Some((7, jed_idx - 61054))
                } else if (69776..69776 + 74 * 40).contains(&jed_idx) {
                    Some((8, jed_idx - 69776))
                } else if (78498..78498 + 74 * 40).contains(&jed_idx) {
                    Some((9, jed_idx - 78498))
                } else if (87220..87220 + 74 * 40).contains(&jed_idx) {
                    Some((10, jed_idx - 87220))
                } else if (95942..95942 + 74 * 40).contains(&jed_idx) {
                    Some((11, jed_idx - 95942))
                } else if (104664..104664 + 74 * 40).contains(&jed_idx) {
                    Some((12, jed_idx - 104664))
                } else if (113386..113386 + 74 * 40).contains(&jed_idx) {
                    Some((13, jed_idx - 113386))
                } else if (122108..122108 + 74 * 40).contains(&jed_idx) {
                    Some((14, jed_idx - 122108))
                } else if (130830..130830 + 74 * 40).contains(&jed_idx) {
                    Some((15, jed_idx - 130830))
                } else if (139552..139552 + 74 * 40).contains(&jed_idx) {
                    Some((16, jed_idx - 139552))
                } else if (148274..148274 + 74 * 40).contains(&jed_idx) {
                    Some((17, jed_idx - 148274))
                } else if (156996..156996 + 74 * 40).contains(&jed_idx) {
                    Some((18, jed_idx - 156996))
                } else if (165718..165718 + 74 * 40).contains(&jed_idx) {
                    Some((19, jed_idx - 165718))
                } else if (174440..174440 + 74 * 40).contains(&jed_idx) {
                    Some((20, jed_idx - 174440))
                } else if (183162..183162 + 74 * 40).contains(&jed_idx) {
                    Some((21, jed_idx - 183162))
                } else if (191884..191884 + 74 * 40).contains(&jed_idx) {
                    Some((22, jed_idx - 191884))
                } else if (200606..200606 + 74 * 40).contains(&jed_idx) {
                    Some((23, jed_idx - 200606))
                } else {
                    None
                }
            }
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_and(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (320..320 + 56 * 80).contains(&jed_idx) {
                    Some((0, jed_idx - 320))
                } else if (6448..6448 + 56 * 80).contains(&jed_idx) {
                    Some((1, jed_idx - 6448))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (640..640 + 56 * 80).contains(&jed_idx) {
                    Some((0, jed_idx - 640))
                } else if (7088..7088 + 56 * 80).contains(&jed_idx) {
                    Some((1, jed_idx - 7088))
                } else if (13536..13536 + 56 * 80).contains(&jed_idx) {
                    Some((2, jed_idx - 13536))
                } else if (19984..19984 + 56 * 80).contains(&jed_idx) {
                    Some((3, jed_idx - 19984))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => {
                if (1120..1120 + 56 * 80).contains(&jed_idx) {
                    Some((0, jed_idx - 1120))
                } else if (8028..8028 + 56 * 80).contains(&jed_idx) {
                    Some((1, jed_idx - 8028))
                } else if (14936..14936 + 56 * 80).contains(&jed_idx) {
                    Some((2, jed_idx - 14936))
                } else if (21857..21857 + 56 * 80).contains(&jed_idx) {
                    Some((3, jed_idx - 21857))
                } else if (28778..28778 + 56 * 80).contains(&jed_idx) {
                    Some((4, jed_idx - 28778))
                } else if (35699..35699 + 56 * 80).contains(&jed_idx) {
                    Some((5, jed_idx - 35699))
                } else if (42607..42607 + 56 * 80).contains(&jed_idx) {
                    Some((6, jed_idx - 42607))
                } else if (49528..49528 + 56 * 80).contains(&jed_idx) {
                    Some((7, jed_idx - 49528))
                } else {
                    None
                }
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                if (2960..2960 + 56 * 80).contains(&jed_idx) {
                    Some((0, jed_idx - 2960))
                } else if (11682..11682 + 56 * 80).contains(&jed_idx) {
                    Some((1, jed_idx - 11682))
                } else if (20404..20404 + 56 * 80).contains(&jed_idx) {
                    Some((2, jed_idx - 20404))
                } else if (29126..29126 + 56 * 80).contains(&jed_idx) {
                    Some((3, jed_idx - 29126))
                } else if (37848..37848 + 56 * 80).contains(&jed_idx) {
                    Some((4, jed_idx - 37848))
                } else if (46570..46570 + 56 * 80).contains(&jed_idx) {
                    Some((5, jed_idx - 46570))
                } else if (55292..55292 + 56 * 80).contains(&jed_idx) {
                    Some((6, jed_idx - 55292))
                } else if (64014..64014 + 56 * 80).contains(&jed_idx) {
                    Some((7, jed_idx - 64014))
                } else if (72736..72736 + 56 * 80).contains(&jed_idx) {
                    Some((8, jed_idx - 72736))
                } else if (81458..81458 + 56 * 80).contains(&jed_idx) {
                    Some((9, jed_idx - 81458))
                } else if (90180..90180 + 56 * 80).contains(&jed_idx) {
                    Some((10, jed_idx - 90180))
                } else if (98902..98902 + 56 * 80).contains(&jed_idx) {
                    Some((11, jed_idx - 98902))
                } else if (107624..107624 + 56 * 80).contains(&jed_idx) {
                    Some((12, jed_idx - 107624))
                } else if (116346..116346 + 56 * 80).contains(&jed_idx) {
                    Some((13, jed_idx - 116346))
                } else if (125068..125068 + 56 * 80).contains(&jed_idx) {
                    Some((14, jed_idx - 125068))
                } else if (133790..133790 + 56 * 80).contains(&jed_idx) {
                    Some((15, jed_idx - 133790))
                } else if (142512..142512 + 56 * 80).contains(&jed_idx) {
                    Some((16, jed_idx - 142512))
                } else if (151234..151234 + 56 * 80).contains(&jed_idx) {
                    Some((17, jed_idx - 151234))
                } else if (159956..159956 + 56 * 80).contains(&jed_idx) {
                    Some((18, jed_idx - 159956))
                } else if (168678..168678 + 56 * 80).contains(&jed_idx) {
                    Some((19, jed_idx - 168678))
                } else if (177400..177400 + 56 * 80).contains(&jed_idx) {
                    Some((20, jed_idx - 177400))
                } else if (186122..186122 + 56 * 80).contains(&jed_idx) {
                    Some((21, jed_idx - 186122))
                } else if (194844..194844 + 56 * 80).contains(&jed_idx) {
                    Some((22, jed_idx - 194844))
                } else if (203566..203566 + 56 * 80).contains(&jed_idx) {
                    Some((23, jed_idx - 203566))
                } else {
                    None
                }
            }
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn _is_or(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (4800..4800 + 16 * 56).contains(&jed_idx) {
                    Some((0, jed_idx - 4800))
                } else if (10928..10928 + 16 * 56).contains(&jed_idx) {
                    Some((1, jed_idx - 10928))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (5120..5120 + 16 * 56).contains(&jed_idx) {
                    Some((0, jed_idx - 5120))
                } else if (11568..11568 + 16 * 56).contains(&jed_idx) {
                    Some((1, jed_idx - 11568))
                } else if (18016..18016 + 16 * 56).contains(&jed_idx) {
                    Some((2, jed_idx - 18016))
                } else if (24464..24464 + 16 * 56).contains(&jed_idx) {
                    Some((3, jed_idx - 24464))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => {
                if (5600..5600 + 16 * 56).contains(&jed_idx) {
                    Some((0, jed_idx - 5600))
                } else if (12508..12508 + 16 * 56).contains(&jed_idx) {
                    Some((1, jed_idx - 12508))
                } else if (19416..19416 + 16 * 56).contains(&jed_idx) {
                    Some((2, jed_idx - 19416))
                } else if (26337..26337 + 16 * 56).contains(&jed_idx) {
                    Some((3, jed_idx - 26337))
                } else if (33258..33258 + 16 * 56).contains(&jed_idx) {
                    Some((4, jed_idx - 33258))
                } else if (40179..40179 + 16 * 56).contains(&jed_idx) {
                    Some((5, jed_idx - 40179))
                } else if (47087..47087 + 16 * 56).contains(&jed_idx) {
                    Some((6, jed_idx - 47087))
                } else if (54008..54008 + 16 * 56).contains(&jed_idx) {
                    Some((7, jed_idx - 54008))
                } else {
                    None
                }
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                if (7440..7440 + 16 * 56).contains(&jed_idx) {
                    Some((0, jed_idx - 7440))
                } else if (16162..16162 + 16 * 56).contains(&jed_idx) {
                    Some((1, jed_idx - 16162))
                } else if (24884..24884 + 16 * 56).contains(&jed_idx) {
                    Some((2, jed_idx - 24884))
                } else if (33606..33606 + 16 * 56).contains(&jed_idx) {
                    Some((3, jed_idx - 33606))
                } else if (42328..42328 + 16 * 56).contains(&jed_idx) {
                    Some((4, jed_idx - 42328))
                } else if (51050..51050 + 16 * 56).contains(&jed_idx) {
                    Some((5, jed_idx - 51050))
                } else if (59772..59772 + 16 * 56).contains(&jed_idx) {
                    Some((6, jed_idx - 59772))
                } else if (68494..68494 + 16 * 56).contains(&jed_idx) {
                    Some((7, jed_idx - 68494))
                } else if (77216..77216 + 16 * 56).contains(&jed_idx) {
                    Some((8, jed_idx - 77216))
                } else if (85938..85938 + 16 * 56).contains(&jed_idx) {
                    Some((9, jed_idx - 85938))
                } else if (94660..94660 + 16 * 56).contains(&jed_idx) {
                    Some((10, jed_idx - 94660))
                } else if (103382..103382 + 16 * 56).contains(&jed_idx) {
                    Some((11, jed_idx - 103382))
                } else if (112104..112104 + 16 * 56).contains(&jed_idx) {
                    Some((12, jed_idx - 112104))
                } else if (120826..120826 + 16 * 56).contains(&jed_idx) {
                    Some((13, jed_idx - 120826))
                } else if (129548..129548 + 16 * 56).contains(&jed_idx) {
                    Some((14, jed_idx - 129548))
                } else if (138270..138270 + 16 * 56).contains(&jed_idx) {
                    Some((15, jed_idx - 138270))
                } else if (146992..146992 + 16 * 56).contains(&jed_idx) {
                    Some((16, jed_idx - 146992))
                } else if (155714..155714 + 16 * 56).contains(&jed_idx) {
                    Some((17, jed_idx - 155714))
                } else if (164436..164436 + 16 * 56).contains(&jed_idx) {
                    Some((18, jed_idx - 164436))
                } else if (173158..173158 + 16 * 56).contains(&jed_idx) {
                    Some((19, jed_idx - 173158))
                } else if (181880..181880 + 16 * 56).contains(&jed_idx) {
                    Some((20, jed_idx - 181880))
                } else if (190602..190602 + 16 * 56).contains(&jed_idx) {
                    Some((21, jed_idx - 190602))
                } else if (199324..199324 + 16 * 56).contains(&jed_idx) {
                    Some((22, jed_idx - 199324))
                } else if (208046..208046 + 16 * 56).contains(&jed_idx) {
                    Some((23, jed_idx - 208046))
                } else {
                    None
                }
            }
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
            XC2Device::XC2C128 => {
                if (6496..6908).contains(&jed_idx) {
                    Some((0, jed_idx - 6496))
                } else if (13404..13816).contains(&jed_idx) {
                    Some((1, jed_idx - 13404))
                } else if (20312..20737).contains(&jed_idx) {
                    Some((2, jed_idx - 20312))
                } else if (27233..27658).contains(&jed_idx) {
                    Some((3, jed_idx - 27233))
                } else if (34154..34579).contains(&jed_idx) {
                    Some((4, jed_idx - 34154))
                } else if (41075..41487).contains(&jed_idx) {
                    Some((5, jed_idx - 41075))
                } else if (47983..48408).contains(&jed_idx) {
                    Some((6, jed_idx - 47983))
                } else if (54904..55316).contains(&jed_idx) {
                    Some((7, jed_idx - 54904))
                } else {
                    None
                }
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                if (8336..17058).contains(&jed_idx) {
                    Some((0, jed_idx - 8336))
                } else if (17058..25780).contains(&jed_idx) {
                    Some((1, jed_idx - 17058))
                } else if (25780..34502).contains(&jed_idx) {
                    Some((2, jed_idx - 25780))
                } else if (34502..43224).contains(&jed_idx) {
                    Some((3, jed_idx - 34502))
                } else if (43224..51946).contains(&jed_idx) {
                    Some((4, jed_idx - 43224))
                } else if (51946..60668).contains(&jed_idx) {
                    Some((5, jed_idx - 51946))
                } else if (60668..69390).contains(&jed_idx) {
                    Some((6, jed_idx - 60668))
                } else if (69390..78112).contains(&jed_idx) {
                    Some((7, jed_idx - 69390))
                } else if (78112..86834).contains(&jed_idx) {
                    Some((8, jed_idx - 78112))
                } else if (86834..95556).contains(&jed_idx) {
                    Some((9, jed_idx - 86834))
                } else if (95556..104278).contains(&jed_idx) {
                    Some((10, jed_idx - 95556))
                } else if (104278..113000).contains(&jed_idx) {
                    Some((11, jed_idx - 104278))
                } else if (113000..121722).contains(&jed_idx) {
                    Some((12, jed_idx - 113000))
                } else if (121722..130444).contains(&jed_idx) {
                    Some((13, jed_idx - 121722))
                } else if (130444..139166).contains(&jed_idx) {
                    Some((14, jed_idx - 130444))
                } else if (139166..147888).contains(&jed_idx) {
                    Some((15, jed_idx - 139166))
                } else if (147888..156610).contains(&jed_idx) {
                    Some((16, jed_idx - 147888))
                } else if (156610..165332).contains(&jed_idx) {
                    Some((17, jed_idx - 156610))
                } else if (165332..174054).contains(&jed_idx) {
                    Some((18, jed_idx - 165332))
                } else if (174054..182776).contains(&jed_idx) {
                    Some((19, jed_idx - 174054))
                } else if (182776..191498).contains(&jed_idx) {
                    Some((20, jed_idx - 182776))
                } else if (191498..200220).contains(&jed_idx) {
                    Some((21, jed_idx - 191498))
                } else if (200220..208942).contains(&jed_idx) {
                    Some((22, jed_idx - 200220))
                } else if (208942..209328).contains(&jed_idx) {
                    Some((23, jed_idx - 208942))
                } else {
                    None
                }
            }
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

    #[test]
    fn check_jed_xc2c384() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c384.map");
        check_map(XC2Device::XC2C384, p);
    }
}
