use std::{cell::Cell, process::ExitCode};

use bittwiddler_core::prelude::{BitArray, HumanLevelDynamicAccessor, HumanLevelThatHasState};

#[cfg(not(feature = "std"))]
compile_error!("This binary requires the 'std' feature");

struct TrackingBitAccessor<U: BitArray> {
    u: U,
    tracking: Vec<Vec<Cell<usize>>>,
}
impl<U: BitArray> BitArray for TrackingBitAccessor<U> {
    fn get(&self, c: bittwiddler_core::prelude::Coordinate) -> bool {
        self.tracking[c.y][c.x].set(self.tracking[c.y][c.x].get() + 1);
        self.u.get(c)
    }

    fn set(&mut self, c: bittwiddler_core::prelude::Coordinate, val: bool) {
        self.tracking[c.y][c.x].set(self.tracking[c.y][c.x].get() + 1);
        self.u.set(c, val)
    }
}
impl<U: HumanLevelDynamicAccessor + BitArray> HumanLevelDynamicAccessor for TrackingBitAccessor<U> {
    fn _human_fields(&self) -> &'static [&'static str] {
        self.u._human_fields()
    }

    fn _human_sublevels(&self) -> &'static [&'static str] {
        self.u._human_sublevels()
    }

    fn _human_construct_field(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Result<Box<dyn bittwiddler_core::prelude::PropertyAccessorDyn>, ()> {
        self.u._human_construct_field(idx, params)
    }

    fn _human_construct_all_fields<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn bittwiddler_core::prelude::PropertyAccessorDyn>> + 's>
    {
        self.u._human_construct_all_fields(idx)
    }

    fn _human_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Result<Box<dyn HumanLevelDynamicAccessor>, ()> {
        self.u._human_descend_sublevel(idx, params)
    }

    fn _human_construct_all_sublevels<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn HumanLevelDynamicAccessor>> + 's> {
        self.u._human_construct_all_sublevels(idx)
    }
}
impl<U: HumanLevelThatHasState + BitArray> HumanLevelThatHasState for TrackingBitAccessor<U> {
    fn _human_dump_my_state(
        &self,
        dump: &mut dyn bittwiddler_core::prelude::HumanSinkForStatePieces,
    ) {
        self.u._human_dump_my_state(dump)
    }
}

fn main() -> ExitCode {
    let args = ::std::env::args_os().collect::<Vec<_>>();

    if args.len() != 2 {
        println!(
            "Usage: {} <device>-<speed>-<package>",
            args[0].to_string_lossy()
        );
        return ExitCode::FAILURE;
    }

    let part = args[1].to_string_lossy().as_ref().try_into().unwrap();
    let bitstream = xc2bit::bitstream::Coolrunner2::new(part);
    let tracked_bitstream = TrackingBitAccessor {
        u: bitstream,
        tracking: vec![
            vec![Cell::new(0); part.device.fuse_array_dims().0];
            part.device.fuse_array_dims().1
        ],
    };
    bittwiddler_textfile::write(std::io::empty(), &tracked_bitstream).unwrap();

    for y in 0..part.device.fuse_array_dims().1 {
        for x in 0..part.device.fuse_array_dims().0 {
            let accesses = tracked_bitstream.tracking[y][x].get();

            if accesses > 1 {
                panic!("BAD! Accessed ({x}, {y}) more than once!");
            }

            if accesses == 0 {
                print!("x")
            } else {
                print!(" ")
            }
        }
        println!("|\n")
    }

    let mut ranges = vec![Vec::new(); part.device.fuse_array_dims().1];

    for y in 0..part.device.fuse_array_dims().1 {
        let mut range_start = None;
        let mut range_end = 0;

        for x in 0..part.device.fuse_array_dims().0 {
            let accesses = tracked_bitstream.tracking[y][x].get();

            if accesses == 0 {
                if range_start.is_some() {
                    range_end = x;
                } else {
                    range_start = Some(x);
                    range_end = x;
                }
            } else {
                if let Some(range_start) = range_start.take() {
                    // println!("{}..={}", range_start, range_end);
                    ranges[y].push((range_start, range_end));
                }
            }
        }
        if let Some(range_start) = range_start.take() {
            // println!("{}..={}", range_start, part.device.fuse_array_dims().0 - 1);
            ranges[y].push((range_start, part.device.fuse_array_dims().0 - 1));
        }
    }

    // dbg!(ranges);

    // extra paranoid double checking
    for (y, ranges) in ranges.iter().enumerate() {
        for x in 0..part.device.fuse_array_dims().0 {
            let accesses = tracked_bitstream.tracking[y][x].get();

            let mut found = false;
            for &(range_start, range_end) in ranges {
                assert!(range_end >= range_start);
                if x >= range_start && x <= range_end {
                    assert!(!found);
                    found = true;
                }
            }

            if accesses == 0 {
                assert!(found);
            } else {
                assert!(!found);
            }
        }
    }

    for (y, ranges) in ranges.iter().enumerate() {
        print!("{y} => ");
        if ranges.len() > 0 {
            for (i, &(range_start, range_end)) in ranges.iter().enumerate() {
                if i != 0 {
                    print!("||");
                }
                if range_start != 0 {
                    print!("x >= {range_start} && x <= {range_end}");
                } else {
                    print!("x <= {range_end}");
                }
            }
        } else {
            print!("false")
        }
        println!(",")
    }

    ExitCode::SUCCESS
}
