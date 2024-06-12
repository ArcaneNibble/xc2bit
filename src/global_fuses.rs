//! Locations of various global fuses

use bittwiddler_core::prelude::Coordinate;

use crate::partdb::XC2Device;

pub const XC2C32_OVOLTAGE: Coordinate = Coordinate::new(130, 24);
pub const XC2C32_IVOLTAGE: Coordinate = Coordinate::new(130, 25);
pub const XC2C32_EXTRA_IBUF_SCHMITT_TRIGGER: Coordinate = Coordinate::new(131, 24);
pub const XC2C32_EXTRA_IBUF_TERMINATION: Coordinate = Coordinate::new(132, 24);

pub const XC2C64_OVOLTAGE: Coordinate = Coordinate::new(138, 23);
pub const XC2C64_IVOLTAGE: Coordinate = Coordinate::new(137, 23);

pub trait GlobalFuses {
    fn done1(&self) -> Coordinate;
    fn gck(&self) -> [Coordinate; 3];
    fn gts_enable(&self) -> [Coordinate; 4];
    fn gts_invert(&self) -> [Coordinate; 4];
    fn gsr_enable(&self) -> Coordinate;
    fn gsr_invert(&self) -> Coordinate;
    fn global_term(&self) -> Coordinate;
    fn io_input_voltage(&self) -> &'static [Coordinate];
    fn io_output_voltage(&self) -> &'static [Coordinate];
    fn vref_enable(&self) -> Coordinate;
    fn clk_div_enable(&self) -> Coordinate;
    fn clk_div_ratio(&self) -> [Coordinate; 3];
    fn clk_div_delay(&self) -> Coordinate;
    fn data_gate(&self) -> Coordinate;
}
impl GlobalFuses for XC2Device {
    fn done1(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => Coordinate::new(9, 48),
            XC2Device::XC2C64 | XC2Device::XC2C64A => Coordinate::new(8, 96),
            XC2Device::XC2C128 => Coordinate::new(9, 80),
            XC2Device::XC2C256 => Coordinate::new(9, 96),
            XC2Device::XC2C384 => Coordinate::new(9, 120),
            XC2Device::XC2C512 => Coordinate::new(9, 160),
        }
    }

    fn gck(&self) -> [Coordinate; 3] {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => [
                Coordinate::new(126, 23),
                Coordinate::new(127, 23),
                Coordinate::new(128, 23),
            ],
            XC2Device::XC2C64 | XC2Device::XC2C64A => [
                Coordinate::new(133, 23),
                Coordinate::new(134, 23),
                Coordinate::new(135, 23),
            ],
            XC2Device::XC2C128 => [
                Coordinate::new(365, 67),
                Coordinate::new(366, 67),
                Coordinate::new(367, 67),
            ],
            XC2Device::XC2C256 => [
                Coordinate::new(519, 23),
                Coordinate::new(520, 23),
                Coordinate::new(521, 23),
            ],
            XC2Device::XC2C384 => [
                Coordinate::new(467, 102),
                Coordinate::new(468, 102),
                Coordinate::new(469, 102),
            ],
            XC2Device::XC2C512 => [
                Coordinate::new(979, 147),
                Coordinate::new(980, 147),
                Coordinate::new(981, 147),
            ],
        }
    }

    fn gts_enable(&self) -> [Coordinate; 4] {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => [
                Coordinate::new(127, 24),
                Coordinate::new(129, 24),
                Coordinate::new(127, 25),
                Coordinate::new(129, 25),
            ],
            XC2Device::XC2C64 | XC2Device::XC2C64A => [
                Coordinate::new(134, 24),
                Coordinate::new(136, 24),
                Coordinate::new(138, 73),
                Coordinate::new(138, 24),
            ],
            XC2Device::XC2C128 => [
                Coordinate::new(5, 27),
                Coordinate::new(7, 27),
                Coordinate::new(5, 67),
                Coordinate::new(7, 67),
            ],
            XC2Device::XC2C256 => [
                Coordinate::new(182, 23),
                Coordinate::new(177, 24),
                Coordinate::new(179, 24),
                Coordinate::new(182, 24),
            ],
            XC2Device::XC2C384 => [
                Coordinate::new(463, 107),
                Coordinate::new(464, 107),
                Coordinate::new(465, 107),
                Coordinate::new(466, 107),
            ],
            XC2Device::XC2C512 => [
                Coordinate::new(4, 27),
                Coordinate::new(481, 27),
                Coordinate::new(6, 27),
                Coordinate::new(8, 27),
            ],
        }
    }

    fn gts_invert(&self) -> [Coordinate; 4] {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => [
                Coordinate::new(126, 24),
                Coordinate::new(128, 24),
                Coordinate::new(126, 25),
                Coordinate::new(128, 25),
            ],
            XC2Device::XC2C64 | XC2Device::XC2C64A => [
                Coordinate::new(133, 24),
                Coordinate::new(135, 24),
                Coordinate::new(137, 73),
                Coordinate::new(137, 24),
            ],
            XC2Device::XC2C128 => [
                Coordinate::new(4, 27),
                Coordinate::new(6, 27),
                Coordinate::new(4, 67),
                Coordinate::new(6, 67),
            ],
            XC2Device::XC2C256 => [
                Coordinate::new(181, 23),
                Coordinate::new(176, 24),
                Coordinate::new(178, 24),
                Coordinate::new(181, 24),
            ],
            XC2Device::XC2C384 => [
                Coordinate::new(463, 102),
                Coordinate::new(464, 102),
                Coordinate::new(465, 102),
                Coordinate::new(466, 102),
            ],
            XC2Device::XC2C512 => [
                Coordinate::new(3, 27),
                Coordinate::new(480, 27),
                Coordinate::new(5, 27),
                Coordinate::new(7, 27),
            ],
        }
    }

    fn gsr_enable(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => Coordinate::new(130, 23),
            XC2Device::XC2C64 | XC2Device::XC2C64A => Coordinate::new(136, 73),
            XC2Device::XC2C128 => Coordinate::new(2, 67),
            XC2Device::XC2C256 => Coordinate::new(179, 23),
            XC2Device::XC2C384 => Coordinate::new(2, 97),
            XC2Device::XC2C512 => Coordinate::new(2, 27),
        }
    }

    fn gsr_invert(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => Coordinate::new(129, 23),
            XC2Device::XC2C64 | XC2Device::XC2C64A => Coordinate::new(135, 73),
            XC2Device::XC2C128 => Coordinate::new(1, 67),
            XC2Device::XC2C256 => Coordinate::new(178, 23),
            XC2Device::XC2C384 => Coordinate::new(1, 97),
            XC2Device::XC2C512 => Coordinate::new(1, 27),
        }
    }

    fn global_term(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A => Coordinate::new(131, 23),
            XC2Device::XC2C64 | XC2Device::XC2C64A => Coordinate::new(136, 23),
            XC2Device::XC2C128 => Coordinate::new(370, 67),
            XC2Device::XC2C256 => Coordinate::new(517, 23),
            XC2Device::XC2C384 => Coordinate::new(931, 17),
            XC2Device::XC2C512 => Coordinate::new(983, 147),
        }
    }

    fn io_input_voltage(&self) -> &'static [Coordinate] {
        match self {
            XC2Device::XC2C32 => &[XC2C32_IVOLTAGE],
            XC2Device::XC2C32A => {
                const X: &[Coordinate] = &[Coordinate::new(131, 25), Coordinate::new(133, 25)];
                X
            }
            XC2Device::XC2C64 => &[XC2C64_IVOLTAGE],
            XC2Device::XC2C64A => {
                const X: &[Coordinate] = &[Coordinate::new(139, 23), Coordinate::new(141, 23)];
                X
            }
            XC2Device::XC2C128 => {
                const X: &[Coordinate] = &[Coordinate::new(8, 67), Coordinate::new(368, 67)];
                X
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                const X: &[Coordinate] = &[
                    Coordinate::new(936, 17),
                    Coordinate::new(1864, 17),
                    Coordinate::new(1, 17),
                    Coordinate::new(929, 17),
                ];
                X
            }
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn io_output_voltage(&self) -> &'static [Coordinate] {
        match self {
            XC2Device::XC2C32 => &[XC2C32_OVOLTAGE],
            XC2Device::XC2C32A => {
                const X: &[Coordinate] = &[Coordinate::new(132, 25), Coordinate::new(134, 25)];
                X
            }
            XC2Device::XC2C64 => &[XC2C64_OVOLTAGE],
            XC2Device::XC2C64A => {
                const X: &[Coordinate] = &[Coordinate::new(140, 23), Coordinate::new(142, 23)];
                X
            }
            XC2Device::XC2C128 => {
                const X: &[Coordinate] = &[Coordinate::new(9, 67), Coordinate::new(369, 67)];
                X
            }
            XC2Device::XC2C256 => todo!(),
            XC2Device::XC2C384 => {
                const X: &[Coordinate] = &[
                    Coordinate::new(937, 17),
                    Coordinate::new(1865, 17),
                    Coordinate::new(2, 17),
                    Coordinate::new(930, 17),
                ];
                X
            }
            XC2Device::XC2C512 => todo!(),
        }
    }

    fn vref_enable(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                unreachable!()
            }
            XC2Device::XC2C128 => Coordinate::new(10, 67),
            XC2Device::XC2C256 => Coordinate::new(177, 23),
            XC2Device::XC2C384 => Coordinate::new(3, 17),
            XC2Device::XC2C512 => Coordinate::new(1, 147),
        }
    }

    fn clk_div_enable(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                unreachable!()
            }
            XC2Device::XC2C128 => Coordinate::new(364, 67),
            XC2Device::XC2C256 => Coordinate::new(519, 24),
            XC2Device::XC2C384 => Coordinate::new(471, 107),
            XC2Device::XC2C512 => Coordinate::new(978, 147),
        }
    }

    fn clk_div_ratio(&self) -> [Coordinate; 3] {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                unreachable!()
            }
            XC2Device::XC2C128 => [
                Coordinate::new(363, 67),
                Coordinate::new(362, 67),
                Coordinate::new(361, 67),
            ],
            XC2Device::XC2C256 => [
                Coordinate::new(518, 24),
                Coordinate::new(517, 24),
                Coordinate::new(516, 24),
            ],
            XC2Device::XC2C384 => [
                Coordinate::new(470, 107),
                Coordinate::new(469, 107),
                Coordinate::new(468, 107),
            ],
            XC2Device::XC2C512 => [
                Coordinate::new(977, 147),
                Coordinate::new(976, 147),
                Coordinate::new(975, 147),
            ],
        }
    }

    fn clk_div_delay(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                unreachable!()
            }
            XC2Device::XC2C128 => Coordinate::new(360, 67),
            XC2Device::XC2C256 => Coordinate::new(515, 24),
            XC2Device::XC2C384 => Coordinate::new(467, 107),
            XC2Device::XC2C512 => Coordinate::new(974, 147),
        }
    }

    fn data_gate(&self) -> Coordinate {
        match self {
            XC2Device::XC2C32 | XC2Device::XC2C32A | XC2Device::XC2C64 | XC2Device::XC2C64A => {
                unreachable!()
            }
            XC2Device::XC2C128 => Coordinate::new(371, 67),
            XC2Device::XC2C256 => Coordinate::new(518, 23),
            XC2Device::XC2C384 => Coordinate::new(932, 17),
            XC2Device::XC2C512 => Coordinate::new(982, 147),
        }
    }
}