use std::{fs::File, process::ExitCode};

use xc2bit::{crbit::CrbitReader, jed::JedReader};

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
    let bitstream = if filename.as_encoded_bytes().ends_with(b".crbit") {
        xc2bit::bitstream::Coolrunner2::read_crbit(inp).unwrap()
    } else {
        xc2bit::bitstream::Coolrunner2::read_jed(inp).unwrap()
    };
    bittwiddler_textfile::write(std::io::stdout(), &bitstream).unwrap();

    ExitCode::SUCCESS
}
