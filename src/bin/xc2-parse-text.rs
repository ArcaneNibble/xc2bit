use std::{fs::File, process::ExitCode};

use xc2bit::{crbit::CrbitWriter, jed::JedWriter};

#[cfg(not(feature = "std"))]
compile_error!("This binary requires the 'std' feature");

fn main() -> ExitCode {
    let args = ::std::env::args_os().collect::<Vec<_>>();

    if args.len() < 3 {
        println!(
            "Usage: {} xc2cXXX-XXX-XXX file.txt [jed]",
            args[0].to_string_lossy()
        );
        return ExitCode::FAILURE;
    }

    let part = args[1].to_string_lossy().as_ref().try_into().unwrap();

    let filename = &args[2];
    let inp = File::open(filename).unwrap();

    let mut bitstream = xc2bit::bitstream::Coolrunner2::new(part);
    bittwiddler_textfile::parse(inp, &mut bitstream).unwrap();

    if args.len() < 4 || args[3] != "jed" {
        bitstream.write_crbit(std::io::stdout()).unwrap();
    } else {
        bitstream.write_jed(std::io::stdout()).unwrap();
    }

    ExitCode::SUCCESS
}
