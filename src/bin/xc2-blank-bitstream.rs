use std::process::ExitCode;

use xc2bit::crbit::CrbitWriter;

#[cfg(not(feature = "std"))]
compile_error!("This binary requires the 'std' feature");

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
    bitstream.write_crbit(std::io::stdout()).unwrap();

    ExitCode::SUCCESS
}
