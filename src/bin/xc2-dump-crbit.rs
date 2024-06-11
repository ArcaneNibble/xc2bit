use std::{fs::File, process::ExitCode};

use xc2bit::crbit::CrbitReader;

#[cfg(not(feature = "std"))]
compile_error!("This binary requires the 'std' feature");

fn main() -> ExitCode {
    let args = ::std::env::args_os().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} file.crbit", args[0].to_string_lossy());
        return ExitCode::FAILURE;
    }

    let filename = &args[1];
    let inp = File::open(filename).unwrap();
    let _bitstream = xc2bit::bitstream::Coolrunner2::read_crbit(inp).unwrap();
    // bitstream.write_crbit(std::io::stdout()).unwrap();

    ExitCode::SUCCESS
}
