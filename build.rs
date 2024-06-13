use std::io::Write;
use std::{env, fs::File, path::PathBuf};

use bittwiddler_dsl::property::{emit_bit_property, parse_bit_property};
use bittwiddler_dsl::spreadsheet::{emit_spreadsheet, parse_spreadsheet};

fn generate_one_enum(base_name: &str) {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push(format!("src/enums/{base_name}.txt"));
    let f = File::open(p).unwrap();
    let result = parse_bit_property::parse(f).unwrap();
    let mut settings = emit_bit_property::Settings::default();
    settings.enable_no_std = true;
    settings.alloc_feature_gate = Some("alloc".to_string());
    let result_ts = emit_bit_property::emit(&result, &settings);

    let mut p = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    p.push(format!("{base_name}.rs"));
    let mut f = File::create(p).unwrap();
    write!(f, "{}", result_ts).unwrap();
    println!("cargo:rerun-if-changed={base_name}.rs");
}

fn main() {
    generate_one_enum("mc-clk-src");
    generate_one_enum("mc-fb");
    generate_one_enum("mc-ff-mode");
    generate_one_enum("mc-r-src");
    generate_one_enum("mc-s-src");
    generate_one_enum("mc-xor-mode");

    generate_one_enum("io-fb");
    generate_one_enum("io-oe");
    generate_one_enum("io-regcom");
    generate_one_enum("io-slew");
    generate_one_enum("io-ibuf-mode");

    generate_one_enum("global-term");
    generate_one_enum("clk-div");
    generate_one_enum("io-voltage");

    {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("src/tiles.ods");
        let tiles = parse_spreadsheet::parse(p).unwrap();

        let mut p = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        p.push("tiles-out.rs");
        let mut f = File::create(p).unwrap();
        for tile in &tiles {
            let outp_toks = emit_spreadsheet::emit(tile).unwrap();
            write!(f, "{}", outp_toks).unwrap();
        }
        println!("cargo:rerun-if-changed=tiles-out.rs");
    }
}
