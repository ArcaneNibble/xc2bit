//! Xilinx ISE JEDEC programming file compatibility

#[cfg(feature = "std")]
extern crate std;

use core::fmt::Display;

use bittwiddler_core::prelude::{BitArray as BittwiddlerBitArray, Coordinate, PropertyAccessor};
use bitvec::prelude::*;
use jedec::*;

use crate::{
    bitstream::{BitHolder, Coolrunner2},
    fb::{and_get_bit_pos, or_get_bit_pos, FunctionBlock},
    global_fuses::*,
    io::IoPad,
    mc::Macrocell,
    partdb::{XC2Device, XC2Part},
    ANDTERMS_PER_FB, MCS_PER_FB, ZIA_ROWS,
};

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
    fn guess_device_from_fuses(fuse_count: usize) -> Option<Self>
    where
        Self: Sized;
    fn jed_index_to_crbit(&self, jed_idx: usize) -> Coordinate;

    fn _is_fb(&self, jed_idx: usize) -> Option<(usize, usize)>;

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

    fn guess_device_from_fuses(fuse_count: usize) -> Option<Self> {
        match fuse_count {
            12274 => Some(XC2Device::XC2C32),
            12278 => Some(XC2Device::XC2C32A),
            25808 => Some(XC2Device::XC2C64),
            25812 => Some(XC2Device::XC2C64A),
            55341 => Some(XC2Device::XC2C128),
            123249 => Some(XC2Device::XC2C256),
            209357 => Some(XC2Device::XC2C384),
            296403 => Some(XC2Device::XC2C512),
            _ => None,
        }
    }

    fn jed_index_to_crbit(&self, jed_idx: usize) -> Coordinate {
        if let Some((fb, offs)) = self._is_mc(jed_idx) {
            match self {
                XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                    let mc = offs / 27;
                    let a_mc = Macrocell {
                        x: FunctionBlock {
                            device: *self,
                            fb: fb as u8,
                        },
                        mc: mc as u8,
                    };
                    let a_io = IoPad {
                        x: FunctionBlock {
                            device: *self,
                            fb: fb as u8,
                        },
                        mc: mc as u8,
                    };

                    match offs % 27 {
                        0 => a_mc.clk_src().get_bit_pos(0).0,
                        1 => a_mc.clk_inv().get_bit_pos(0).0,
                        2 => a_mc.clk_src().get_bit_pos(1).0,
                        3 => a_mc.clk_src().get_bit_pos(2).0,
                        4 => a_mc.is_ddr().get_bit_pos(0).0,
                        5 => a_mc.r_src().get_bit_pos(0).0,
                        6 => a_mc.r_src().get_bit_pos(1).0,
                        7 => a_mc.s_src().get_bit_pos(0).0,
                        8 => a_mc.s_src().get_bit_pos(1).0,
                        9 => a_mc.ff_mode().get_bit_pos(0).0,
                        10 => a_mc.ff_mode().get_bit_pos(1).0,
                        11 => a_io.fb_src().get_bit_pos(0).0,
                        12 => a_io.fb_src().get_bit_pos(1).0,
                        13 => a_mc.fb_src().get_bit_pos(0).0,
                        14 => a_mc.fb_src().get_bit_pos(1).0,
                        15 => a_mc.use_iob().get_bit_pos(0).0,
                        16 => a_io.schmitt_trigger().get_bit_pos(0).0,
                        17 => a_mc.xor_mode().get_bit_pos(0).0,
                        18 => a_mc.xor_mode().get_bit_pos(1).0,
                        19 => a_io.output_src().get_bit_pos(0).0,
                        20 => a_io.output_pad_mode().get_bit_pos(0).0,
                        21 => a_io.output_pad_mode().get_bit_pos(1).0,
                        22 => a_io.output_pad_mode().get_bit_pos(2).0,
                        23 => a_io.output_pad_mode().get_bit_pos(3).0,
                        24 => a_io.termination_enabled().get_bit_pos(0).0,
                        25 => a_io.slew_rate().get_bit_pos(0).0,
                        26 => a_mc.init_state().get_bit_pos(0).0,
                        _ => unreachable!(),
                    }
                }
                XC2Device::XC2C128
                | XC2Device::XC2C256
                | XC2Device::XC2C384
                | XC2Device::XC2C512 => {
                    let (mc, mc_offs) = get_fat_mc_idx(*self, fb, offs);
                    let a_mc = Macrocell {
                        x: FunctionBlock {
                            device: *self,
                            fb: fb as u8,
                        },
                        mc: mc as u8,
                    };
                    let a_io = IoPad {
                        x: FunctionBlock {
                            device: *self,
                            fb: fb as u8,
                        },
                        mc: mc as u8,
                    };

                    if self.has_io_at(fb as u8, mc as u8) {
                        match mc_offs {
                            0 => a_mc.clk_src().get_bit_pos(0).0,
                            1 => a_mc.clk_src().get_bit_pos(1).0,
                            2 => a_mc.clk_src().get_bit_pos(2).0,
                            3 => a_mc.is_ddr().get_bit_pos(0).0,
                            4 => a_mc.clk_inv().get_bit_pos(0).0,
                            5 => a_io.use_data_gate().get_bit_pos(0).0,
                            6 => a_mc.fb_src().get_bit_pos(0).0,
                            7 => a_mc.fb_src().get_bit_pos(1).0,
                            8 => a_io.input_pad_mode().get_bit_pos(0).0,
                            9 => a_io.input_pad_mode().get_bit_pos(1).0,
                            10 => a_mc.use_iob().get_bit_pos(0).0,
                            11 => a_io.fb_src().get_bit_pos(0).0,
                            12 => a_io.fb_src().get_bit_pos(1).0,
                            13 => a_io.output_pad_mode().get_bit_pos(0).0,
                            14 => a_io.output_pad_mode().get_bit_pos(1).0,
                            15 => a_io.output_pad_mode().get_bit_pos(2).0,
                            16 => a_io.output_pad_mode().get_bit_pos(3).0,
                            17 => a_mc.s_src().get_bit_pos(0).0,
                            18 => a_mc.s_src().get_bit_pos(1).0,
                            19 => a_mc.init_state().get_bit_pos(0).0,
                            20 => a_io.output_src().get_bit_pos(0).0,
                            21 => a_mc.ff_mode().get_bit_pos(0).0,
                            22 => a_mc.ff_mode().get_bit_pos(1).0,
                            23 => a_mc.r_src().get_bit_pos(0).0,
                            24 => a_mc.r_src().get_bit_pos(1).0,
                            25 => a_io.slew_rate().get_bit_pos(0).0,
                            26 => a_io.termination_enabled().get_bit_pos(0).0,
                            27 => a_mc.xor_mode().get_bit_pos(0).0,
                            28 => a_mc.xor_mode().get_bit_pos(1).0,
                            _ => unreachable!(),
                        }
                    } else {
                        match mc_offs {
                            0 => a_mc.clk_src().get_bit_pos(0).0,
                            1 => a_mc.clk_src().get_bit_pos(1).0,
                            2 => a_mc.clk_src().get_bit_pos(2).0,
                            3 => a_mc.is_ddr().get_bit_pos(0).0,
                            4 => a_mc.clk_inv().get_bit_pos(0).0,
                            5 => a_mc.fb_src().get_bit_pos(0).0,
                            6 => a_mc.fb_src().get_bit_pos(1).0,
                            7 => a_mc.s_src().get_bit_pos(0).0,
                            8 => a_mc.s_src().get_bit_pos(1).0,
                            9 => a_mc.init_state().get_bit_pos(0).0,
                            10 => a_mc.ff_mode().get_bit_pos(0).0,
                            11 => a_mc.ff_mode().get_bit_pos(1).0,
                            12 => a_mc.r_src().get_bit_pos(0).0,
                            13 => a_mc.r_src().get_bit_pos(1).0,
                            14 => a_mc.xor_mode().get_bit_pos(0).0,
                            15 => a_mc.xor_mode().get_bit_pos(1).0,
                            _ => unreachable!(),
                        }
                    }
                }
            }
        } else if let Some((fb, offs)) = self._is_or(jed_idx) {
            let pterm_i = offs / 16;
            let mc = offs % 16;
            or_get_bit_pos(*self, fb as u8, mc as u8, pterm_i as u8)
        } else if let Some((fb, offs)) = self._is_and(jed_idx) {
            let pterm_i = offs / 80;
            let offs_80 = offs % 80;
            and_get_bit_pos(
                *self,
                fb as u8,
                pterm_i as u8,
                offs_80 as u8 / 2,
                offs_80 % 2 == 1,
            )
        } else if let Some((fb, offs)) = self._is_zia(jed_idx) {
            let zia_row = offs / self.zia_width();
            let zia_offs = offs % self.zia_width();

            let x_base = (self.zia_width() - 1 - zia_offs) * 2 + fb % 2;
            let y_base = match self {
                XC2Device::XC2C32
                | XC2Device::XC2C32A
                | XC2Device::XC2C64
                | XC2Device::XC2C64A
                | XC2Device::XC2C256 => zia_row + if zia_row >= 20 { 8 } else { 0 },
                XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => zia_row,
            };

            let mc_or_and_offset = match self {
                XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                    9 + ANDTERMS_PER_FB * 2
                }
                XC2Device::XC2C256 => 10 + ANDTERMS_PER_FB * 2,
                XC2Device::XC2C128 | XC2Device::XC2C384 | XC2Device::XC2C512 => {
                    15 + MCS_PER_FB * 2 + ANDTERMS_PER_FB * 2
                }
            };

            self.fb_corner((fb & !1) as u8)
                + Coordinate::new(mc_or_and_offset, 0)
                + Coordinate::new(x_base, y_base)
        } else {
            match self {
                XC2Device::XC2C32 | XC2Device::XC2C32A => {
                    if (12256..12259).contains(&jed_idx) {
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
                    if (25792..25795).contains(&jed_idx) {
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
                    if (55316..55319).contains(&jed_idx) {
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
                XC2Device::XC2C256 => {
                    if (123224..123227).contains(&jed_idx) {
                        self.gck()[jed_idx - 123224]
                    } else if jed_idx == 123227 {
                        self.clk_div_enable()
                    } else if (123228..123231).contains(&jed_idx) {
                        self.clk_div_ratio()[jed_idx - 123228]
                    } else if jed_idx == 123231 {
                        self.clk_div_delay()
                    } else if (123232..123234).contains(&jed_idx) {
                        if jed_idx == 123232 {
                            self.gsr_invert()
                        } else {
                            self.gsr_enable()
                        }
                    } else if (123234..123242).contains(&jed_idx) {
                        let offs = jed_idx - 123234;
                        let gts_idx = offs / 2;
                        if offs % 2 == 0 {
                            self.gts_invert()[gts_idx]
                        } else {
                            self.gts_enable()[gts_idx]
                        }
                    } else if jed_idx == 123242 {
                        self.global_term()
                    } else if jed_idx == 123243 {
                        self.data_gate()
                    } else if (123244..123246).contains(&jed_idx) {
                        self.io_input_voltage()[jed_idx - 123244]
                    } else if (123246..123248).contains(&jed_idx) {
                        self.io_output_voltage()[jed_idx - 123246]
                    } else if jed_idx == 123248 {
                        self.vref_enable()
                    } else {
                        unreachable!()
                    }
                }
                XC2Device::XC2C384 => {
                    if (209328..209331).contains(&jed_idx) {
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
                XC2Device::XC2C512 => {
                    if (296374..296377).contains(&jed_idx) {
                        self.gck()[jed_idx - 296374]
                    } else if jed_idx == 296377 {
                        self.clk_div_enable()
                    } else if (296378..296381).contains(&jed_idx) {
                        self.clk_div_ratio()[jed_idx - 296378]
                    } else if jed_idx == 296381 {
                        self.clk_div_delay()
                    } else if (296382..296384).contains(&jed_idx) {
                        if jed_idx == 296382 {
                            self.gsr_invert()
                        } else {
                            self.gsr_enable()
                        }
                    } else if (296384..296392).contains(&jed_idx) {
                        let offs = jed_idx - 296384;
                        let gts_idx = offs / 2;
                        if offs % 2 == 0 {
                            self.gts_invert()[gts_idx]
                        } else {
                            self.gts_enable()[gts_idx]
                        }
                    } else if jed_idx == 296392 {
                        self.global_term()
                    } else if jed_idx == 296393 {
                        self.data_gate()
                    } else if (296394..296398).contains(&jed_idx) {
                        self.io_input_voltage()[jed_idx - 296394]
                    } else if (296398..296402).contains(&jed_idx) {
                        self.io_output_voltage()[jed_idx - 296398]
                    } else if jed_idx == 296402 {
                        self.vref_enable()
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    }

    fn _is_fb(&self, jed_idx: usize) -> Option<(usize, usize)> {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => {
                if (0..6128).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6128..12256).contains(&jed_idx) {
                    Some((1, jed_idx - 6128))
                } else {
                    None
                }
            }
            XC2Device::XC2C64 | XC2Device::XC2C64A => {
                if (0..6448).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6448..12896).contains(&jed_idx) {
                    Some((1, jed_idx - 6448))
                } else if (12896..19344).contains(&jed_idx) {
                    Some((2, jed_idx - 12896))
                } else if (19344..25792).contains(&jed_idx) {
                    Some((3, jed_idx - 19344))
                } else {
                    None
                }
            }
            XC2Device::XC2C128 => {
                if (0..6908).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (6908..13816).contains(&jed_idx) {
                    Some((1, jed_idx - 6908))
                } else if (13816..20737).contains(&jed_idx) {
                    Some((2, jed_idx - 13816))
                } else if (20737..27658).contains(&jed_idx) {
                    Some((3, jed_idx - 20737))
                } else if (27658..34579).contains(&jed_idx) {
                    Some((4, jed_idx - 27658))
                } else if (34579..41487).contains(&jed_idx) {
                    Some((5, jed_idx - 34579))
                } else if (41487..48408).contains(&jed_idx) {
                    Some((6, jed_idx - 41487))
                } else if (48408..55316).contains(&jed_idx) {
                    Some((7, jed_idx - 48408))
                } else {
                    None
                }
            }
            XC2Device::XC2C256 => {
                if (0..7695).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (7695..15390).contains(&jed_idx) {
                    Some((1, jed_idx - 7695))
                } else if (15390..23085).contains(&jed_idx) {
                    Some((2, jed_idx - 15390))
                } else if (23085..30780).contains(&jed_idx) {
                    Some((3, jed_idx - 23085))
                } else if (30780..38475).contains(&jed_idx) {
                    Some((4, jed_idx - 30780))
                } else if (38475..46170).contains(&jed_idx) {
                    Some((5, jed_idx - 38475))
                } else if (46170..53878).contains(&jed_idx) {
                    Some((6, jed_idx - 46170))
                } else if (53878..61586).contains(&jed_idx) {
                    Some((7, jed_idx - 53878))
                } else if (61586..69294).contains(&jed_idx) {
                    Some((8, jed_idx - 61586))
                } else if (69294..77002).contains(&jed_idx) {
                    Some((9, jed_idx - 69294))
                } else if (77002..84710).contains(&jed_idx) {
                    Some((10, jed_idx - 77002))
                } else if (84710..92418).contains(&jed_idx) {
                    Some((11, jed_idx - 84710))
                } else if (92418..100113).contains(&jed_idx) {
                    Some((12, jed_idx - 92418))
                } else if (100113..107808).contains(&jed_idx) {
                    Some((13, jed_idx - 100113))
                } else if (107808..115516).contains(&jed_idx) {
                    Some((14, jed_idx - 107808))
                } else if (115516..123224).contains(&jed_idx) {
                    Some((15, jed_idx - 115516))
                } else {
                    None
                }
            }
            XC2Device::XC2C384 => {
                if (0..8722).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (8722..17444).contains(&jed_idx) {
                    Some((1, jed_idx - 8722))
                } else if (17444..26166).contains(&jed_idx) {
                    Some((2, jed_idx - 17444))
                } else if (26166..34888).contains(&jed_idx) {
                    Some((3, jed_idx - 26166))
                } else if (34888..43610).contains(&jed_idx) {
                    Some((4, jed_idx - 34888))
                } else if (43610..52332).contains(&jed_idx) {
                    Some((5, jed_idx - 43610))
                } else if (52332..61054).contains(&jed_idx) {
                    Some((6, jed_idx - 52332))
                } else if (61054..69776).contains(&jed_idx) {
                    Some((7, jed_idx - 61054))
                } else if (69776..78498).contains(&jed_idx) {
                    Some((8, jed_idx - 69776))
                } else if (78498..87220).contains(&jed_idx) {
                    Some((9, jed_idx - 78498))
                } else if (87220..95942).contains(&jed_idx) {
                    Some((10, jed_idx - 87220))
                } else if (95942..104664).contains(&jed_idx) {
                    Some((11, jed_idx - 95942))
                } else if (104664..113386).contains(&jed_idx) {
                    Some((12, jed_idx - 104664))
                } else if (113386..122108).contains(&jed_idx) {
                    Some((13, jed_idx - 113386))
                } else if (122108..130830).contains(&jed_idx) {
                    Some((14, jed_idx - 122108))
                } else if (130830..139552).contains(&jed_idx) {
                    Some((15, jed_idx - 130830))
                } else if (139552..148274).contains(&jed_idx) {
                    Some((16, jed_idx - 139552))
                } else if (148274..156996).contains(&jed_idx) {
                    Some((17, jed_idx - 148274))
                } else if (156996..165718).contains(&jed_idx) {
                    Some((18, jed_idx - 156996))
                } else if (165718..174440).contains(&jed_idx) {
                    Some((19, jed_idx - 165718))
                } else if (174440..183162).contains(&jed_idx) {
                    Some((20, jed_idx - 174440))
                } else if (183162..191884).contains(&jed_idx) {
                    Some((21, jed_idx - 183162))
                } else if (191884..200606).contains(&jed_idx) {
                    Some((22, jed_idx - 191884))
                } else if (200606..209328).contains(&jed_idx) {
                    Some((23, jed_idx - 200606))
                } else {
                    None
                }
            }
            XC2Device::XC2C512 => {
                if (0..9256).contains(&jed_idx) {
                    Some((0, jed_idx))
                } else if (9256..18512).contains(&jed_idx) {
                    Some((1, jed_idx - 9256))
                } else if (18512..27781).contains(&jed_idx) {
                    Some((2, jed_idx - 18512))
                } else if (27781..37037).contains(&jed_idx) {
                    Some((3, jed_idx - 27781))
                } else if (37037..46306).contains(&jed_idx) {
                    Some((4, jed_idx - 37037))
                } else if (46306..55562).contains(&jed_idx) {
                    Some((5, jed_idx - 46306))
                } else if (55562..64831).contains(&jed_idx) {
                    Some((6, jed_idx - 55562))
                } else if (64831..74087).contains(&jed_idx) {
                    Some((7, jed_idx - 64831))
                } else if (74087..83343).contains(&jed_idx) {
                    Some((8, jed_idx - 74087))
                } else if (83343..92599).contains(&jed_idx) {
                    Some((9, jed_idx - 83343))
                } else if (92599..101855).contains(&jed_idx) {
                    Some((10, jed_idx - 92599))
                } else if (101855..111124).contains(&jed_idx) {
                    Some((11, jed_idx - 101855))
                } else if (111124..120380).contains(&jed_idx) {
                    Some((12, jed_idx - 111124))
                } else if (120380..129649).contains(&jed_idx) {
                    Some((13, jed_idx - 120380))
                } else if (129649..138905).contains(&jed_idx) {
                    Some((14, jed_idx - 129649))
                } else if (138905..148174).contains(&jed_idx) {
                    Some((15, jed_idx - 138905))
                } else if (148174..157443).contains(&jed_idx) {
                    Some((16, jed_idx - 148174))
                } else if (157443..166699).contains(&jed_idx) {
                    Some((17, jed_idx - 157443))
                } else if (166699..175968).contains(&jed_idx) {
                    Some((18, jed_idx - 166699))
                } else if (175968..185224).contains(&jed_idx) {
                    Some((19, jed_idx - 175968))
                } else if (185224..194493).contains(&jed_idx) {
                    Some((20, jed_idx - 185224))
                } else if (194493..203749).contains(&jed_idx) {
                    Some((21, jed_idx - 194493))
                } else if (203749..213018).contains(&jed_idx) {
                    Some((22, jed_idx - 203749))
                } else if (213018..222274).contains(&jed_idx) {
                    Some((23, jed_idx - 213018))
                } else if (222274..231530).contains(&jed_idx) {
                    Some((24, jed_idx - 222274))
                } else if (231530..240799).contains(&jed_idx) {
                    Some((25, jed_idx - 231530))
                } else if (240799..250055).contains(&jed_idx) {
                    Some((26, jed_idx - 240799))
                } else if (250055..259324).contains(&jed_idx) {
                    Some((27, jed_idx - 250055))
                } else if (259324..268580).contains(&jed_idx) {
                    Some((28, jed_idx - 259324))
                } else if (268580..277849).contains(&jed_idx) {
                    Some((29, jed_idx - 268580))
                } else if (277849..287105).contains(&jed_idx) {
                    Some((30, jed_idx - 277849))
                } else if (287105..296374).contains(&jed_idx) {
                    Some((31, jed_idx - 287105))
                } else {
                    None
                }
            }
        }
    }

    fn _is_zia(&self, jed_idx: usize) -> Option<(usize, usize)> {
        if let Some((fb, offs)) = self._is_fb(jed_idx) {
            if offs < self.zia_width() * ZIA_ROWS {
                Some((fb, offs))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn _is_and(&self, jed_idx: usize) -> Option<(usize, usize)> {
        if let Some((fb, offs)) = self._is_fb(jed_idx) {
            if offs >= self.zia_width() * ZIA_ROWS
                && offs < self.zia_width() * ZIA_ROWS + ZIA_ROWS * 2 * ANDTERMS_PER_FB
            {
                Some((fb, offs - self.zia_width() * ZIA_ROWS))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn _is_or(&self, jed_idx: usize) -> Option<(usize, usize)> {
        if let Some((fb, offs)) = self._is_fb(jed_idx) {
            if offs >= self.zia_width() * ZIA_ROWS + ZIA_ROWS * 2 * ANDTERMS_PER_FB
                && offs
                    < self.zia_width() * ZIA_ROWS
                        + ZIA_ROWS * 2 * ANDTERMS_PER_FB
                        + MCS_PER_FB * ANDTERMS_PER_FB
            {
                Some((
                    fb,
                    offs - self.zia_width() * ZIA_ROWS - ZIA_ROWS * 2 * ANDTERMS_PER_FB,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn _is_mc(&self, jed_idx: usize) -> Option<(usize, usize)> {
        if let Some((fb, offs)) = self._is_fb(jed_idx) {
            if offs
                >= self.zia_width() * ZIA_ROWS
                    + ZIA_ROWS * 2 * ANDTERMS_PER_FB
                    + MCS_PER_FB * ANDTERMS_PER_FB
            {
                Some((
                    fb,
                    offs - self.zia_width() * ZIA_ROWS
                        - ZIA_ROWS * 2 * ANDTERMS_PER_FB
                        - MCS_PER_FB * ANDTERMS_PER_FB,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub enum JedReadError {
    IoError(std::io::Error),
    ParseError(JedParserError),
    UnknownPart,
}
#[cfg(feature = "std")]
impl Display for JedReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            JedReadError::IoError(e) => e.fmt(f),
            JedReadError::ParseError(e) => e.fmt(f),
            JedReadError::UnknownPart => write!(f, "unknown part, wrong fuse count"),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for JedReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JedReadError::IoError(e) => Some(e),
            JedReadError::ParseError(e) => Some(e),
            JedReadError::UnknownPart => None,
        }
    }
}
#[cfg(feature = "std")]
impl From<std::io::Error> for JedReadError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
#[cfg(feature = "std")]
impl From<JedParserError> for JedReadError {
    fn from(value: JedParserError) -> Self {
        Self::ParseError(value)
    }
}

#[cfg(feature = "std")]
pub trait JedReader {
    fn read_jed<R: std::io::Read>(r: R) -> Result<Self, JedReadError>
    where
        Self: Sized;
}
#[cfg(feature = "std")]
impl JedReader for Coolrunner2<BitBox> {
    fn read_jed<R: std::io::Read>(mut r: R) -> Result<Self, JedReadError>
    where
        Self: Sized,
    {
        let mut inp_bytes = std::vec::Vec::new();
        r.read_to_end(&mut inp_bytes)?;
        let jed_file = JEDECFile::read_into_vecs(&inp_bytes, &Quirks::new().no_design_spec(true))?;

        let mut part: Result<XC2Part, ()> = Err(());
        for note in jed_file.notes {
            if let Ok(note) = core::str::from_utf8(note) {
                let note = note.trim();
                if let Some(dev_str) = note.strip_prefix("DEVICE ") {
                    part = dev_str.try_into();
                }
            }
        }

        let part = match part {
            Ok(part) => part,
            Err(_) => {
                // Guess part from fuse count
                XC2Part::new(
                    XC2Device::guess_device_from_fuses(jed_file.f.len())
                        .ok_or(JedReadError::UnknownPart)?,
                    None,
                    None,
                )
                .unwrap()
            }
        };

        let mut bitstream = Coolrunner2::new(part);

        for fuse_idx in 0..part.device.num_jed_fuses() {
            let phys_fuse = part.device.jed_index_to_crbit(fuse_idx);
            BittwiddlerBitArray::set(&mut bitstream, phys_fuse, jed_file.f[fuse_idx]);
        }

        Ok(bitstream)
    }
}

#[cfg(feature = "std")]
pub trait JedWriter {
    fn write_jed<W: std::io::Write>(&self, w: W) -> std::io::Result<()>;
}
#[cfg(feature = "std")]
impl<B: BitHolder> JedWriter for Coolrunner2<B> {
    fn write_jed<W: std::io::Write>(&self, w: W) -> std::io::Result<()> {
        let mut jed_fuses = bitbox![0; self.part.device.num_jed_fuses()];

        for fuse_idx in 0..self.part.device.num_jed_fuses() {
            let phys_fuse = self.part.device.jed_index_to_crbit(fuse_idx);
            jed_fuses.set(fuse_idx, BittwiddlerBitArray::get(self, phys_fuse));
        }

        let mut linebreaks = std::vec::Vec::new();
        let mut fuse_idx = 0;
        for fb in 0..self.part.device.num_fbs() {
            for _zia_row in 0..ZIA_ROWS {
                if fuse_idx != 0 {
                    linebreaks.push(fuse_idx);
                }
                fuse_idx += self.part.device.zia_width();
            }
            linebreaks.push(fuse_idx);

            for _and_row in 0..ANDTERMS_PER_FB {
                linebreaks.push(fuse_idx);
                fuse_idx += ZIA_ROWS * 2;
            }
            linebreaks.push(fuse_idx);

            for _or_row in 0..ANDTERMS_PER_FB {
                linebreaks.push(fuse_idx);
                fuse_idx += MCS_PER_FB;
            }
            linebreaks.push(fuse_idx);

            for mc in 0..MCS_PER_FB {
                linebreaks.push(fuse_idx);
                fuse_idx += match self.part.device {
                    XC2Device::XC2C32
                    | XC2Device::XC2C32A
                    | XC2Device::XC2C64
                    | XC2Device::XC2C64A => 27,
                    XC2Device::XC2C128
                    | XC2Device::XC2C256
                    | XC2Device::XC2C384
                    | XC2Device::XC2C512 => {
                        if self.part.device.has_io_at(fb as u8, mc as u8) {
                            29
                        } else {
                            16
                        }
                    }
                }
            }
            linebreaks.push(fuse_idx);
        }
        linebreaks.push(fuse_idx);

        let jed = JEDECFile {
            f: jed_fuses,
            header: b"crbit native bitstream file written by xc2bit\nhttps://github.com/ArcaneNibble/xc2bit\n\n" as &[u8],
            footer: b"" as &[u8],
            design_spec: b"" as &[u8],
            notes: &[],
            secure_fuse: None,
        };

        jed.write_io_custom_linebreaks(
            w,
            &Quirks::new().no_design_spec(true),
            linebreaks.into_iter(),
        )
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
    fn check_jed_xc2c256() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c256.map");
        check_map(XC2Device::XC2C256, p);
    }

    #[test]
    fn check_jed_xc2c384() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c384.map");
        check_map(XC2Device::XC2C384, p);
    }

    #[test]
    fn check_jed_xc2c512() {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/xc2c512.map");
        check_map(XC2Device::XC2C512, p);
    }
}
