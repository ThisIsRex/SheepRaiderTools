use encodelib::chartable::CharTable;
use encodelib::utils::Utils;
use std::env;
use std::fs::File;
use std::io::Read;

mod checksum;
mod cmd_parser;
mod mlt;
use cmd_parser::{Mode, Parameters};
use mlt::MltFile;

fn main() {
    print_credits();
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let params = match Parameters::parse(&raw_args) {
        Some(p) => p,
        None => {
            print_usage();
            std::process::exit(-1);
        }
    };

    let replace_table = if let Mode::Recompile = params.mode {
        Some(
            CharTable::from_json(
                &Utils::read_file(&params.replace_table_file.unwrap())
                    .expect("Error reading a replace table file"),
            )
            .expect("Bad replace table file"),
        )
    } else {
        None
    };

    for file in &params.files {
        print!("Processing {}...", file);
        match params.mode {
            Mode::Decompile => do_decompile(&file),
            Mode::Recompile => do_recompile(&file, &replace_table.as_ref().unwrap()),
        }
        println!("OK");
    }

    println!("Done.");
}

fn do_decompile(file: &str) {
    let mut buffer: Vec<u8> = vec![];
    File::open(&file).unwrap().read_to_end(&mut buffer).unwrap();

    let mlt = MltFile::from(&buffer).expect("Error reading a .mlt file");
    let f_dest = Utils::get_filename_without_extension(&file) + ".json";
    let json = serde_json::to_string_pretty(&mlt).unwrap();
    Utils::write_file(&f_dest, json.as_bytes());
}

fn do_recompile(file: &str, tbl: &CharTable) {
    let json = Utils::read_file(&file).expect("Error reading a .json file");

    let mlt = match MltFile::from_json(&json) {
        Ok(mlt) => mlt,
        Err(err) => {
            println!("Bad .json file: {}, skipping...", err);
            return;
        }
    };

    let buffer = mlt.encode(tbl);
    let f_dest = Utils::get_filename_without_extension(&file) + ".MLT";
    Utils::write_file(&f_dest, &buffer);
}

fn print_credits() {
    println!(
        r"           ___    ___  __   __           __                __       __   ___     
 |\/| |     |      |  /  \ /  \ |       |__) \ /     |\/| / _`     |__) |__  \_/ 
 |  | |___  |      |  \__/ \__/ |___    |__)  |      |  | \__> ___ |  \ |___ / \ 
 "
    );
}

fn print_usage() {
    println!("Usage:");
    println!("mlt_tool [options] [replace_table] <files>");
    println!();
    println!("Options:");
    println!("-d - decompile");
    println!("-r - recompile -- replace_table required");
    println!("No options - decompile");
}
