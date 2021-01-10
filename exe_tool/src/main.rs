use encodelib::chartable::CharTable;
use encodelib::utils::Utils;

use serde::{Deserialize, Serialize};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use encoding::all::WINDOWS_1252;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor, Read, SeekFrom, Write};

const SIZE_OF_PTR: u32 = 4;

enum Mode {
    Extract,
    Insert,
}

#[derive(Deserialize)]
struct Config {
    pub text_offset: u64,
    pub text_max_size: usize,
    pub pointers_offset: u64,
    pub pointers_runtime_offset: u64,
    pub pointers_count: usize,
}

#[derive(Deserialize, Serialize)]
struct ExeText {
    pub strings: Vec<String>,
}
impl ExeText {
    pub fn new() -> Self {
        ExeText { strings: vec![] }
    }
}

fn main() {
    print_credits();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        fail();
    }
    let mode = match args[0].as_str() {
        "-e" => Mode::Extract,
        "-i" => Mode::Insert,
        _ => {
            fail();
        }
    };
    if let Mode::Insert = mode {
        if args.len() < 5 {
            fail();
        }
    }
    let exe_file = &args[1];
    let config_file = &args[2];

    let config: Config =
        serde_json::from_str(&Utils::read_file(config_file).expect("Error reading a config file"))
            .expect("Bad config file");

    let mut exe = File::open(exe_file).expect("Error opening exe file");

    println!("Processing {}...", exe_file);

    match mode {
        Mode::Extract => do_extract(
            &mut BufReader::new(exe),
            &(Utils::get_filename_without_extension(exe_file) + ".json"),
            &config,
        ),
        Mode::Insert => {
            let str_file = &args[3];
            let ct_file = &args[4];
            do_insert(
                &mut exe,
                str_file,
                ct_file,
                &(Utils::get_filename_without_extension(exe_file) + ".compiled.exe"),
                &config,
            )
        }
    }

    println!("Done");
}

fn do_extract(exe: &mut BufReader<File>, save_as: &str, cfg: &Config) {
    let mut text = ExeText::new();
    exe.seek(SeekFrom::Start(cfg.pointers_offset)).unwrap();
    for _i in 0..cfg.pointers_count {
        let rt_address = exe.read_i32::<LittleEndian>().unwrap();
        let ptr_pos = exe.seek(SeekFrom::Current(0)).unwrap();

        exe.seek(SeekFrom::Start(
            rt_address as u64 - cfg.pointers_runtime_offset + cfg.text_offset,
        ))
        .unwrap();

        let mut raw = vec![];
        exe.read_until(0u8, &mut raw).unwrap();
        raw.pop();
        text.strings
            .push(WINDOWS_1252.decode(&raw, DecoderTrap::Strict).unwrap());

        exe.seek(SeekFrom::Start(ptr_pos)).unwrap();
    }

    let json = serde_json::to_string_pretty(&text).unwrap();
    Utils::write_file(save_as, json.as_bytes());
}

fn do_insert(exe: &mut File, str_file: &str, ct_file: &str, save_as: &str, cfg: &Config) {
    let text: ExeText =
        serde_json::from_str(&Utils::read_file(str_file).expect("Error reading a json file"))
            .expect("Bad json text file");

    let ct: CharTable = serde_json::from_str(
        &Utils::read_file(ct_file).expect("Error reading a replace table file"),
    )
    .expect("Bad replace table file");

    let mut buffer = vec![];
    exe.read_to_end(&mut buffer)
        .expect("Error reading an exe file");

    let mut c_exe = Cursor::new(&mut buffer);
    let mut c_text = Cursor::new(vec![]);

    for i in 0..text.strings.len() {
        if i > cfg.pointers_count {
            println!("WARNING: writing out of bounds! The output assembly might not work");
        }

        let pos = c_text.position();
        let replaced = ct.replace_letters(&text.strings[i]);
        c_text
            .write(
                &WINDOWS_1252
                    .encode(&replaced, EncoderTrap::Replace)
                    .unwrap(),
            )
            .unwrap();

        c_exe
            .seek(SeekFrom::Start(
                cfg.pointers_offset + i as u64 * SIZE_OF_PTR as u64,
            ))
            .unwrap();
        c_exe
            .write_i32::<LittleEndian>((cfg.pointers_runtime_offset + pos) as i32)
            .unwrap();
    }

    let b_text = c_text.get_ref();
    if b_text.len() > cfg.text_max_size {
        println!("WARNING: new text size is larger than maximum. This will break an assembly!!!");
    }

    c_exe.seek(SeekFrom::Start(cfg.text_offset)).unwrap();
    c_exe.write(b_text).unwrap();

    Utils::write_file(save_as, c_exe.get_ref());
}

fn print_credits() {
    println!(
        r" ___      ___    ___  __   __           __                __       __   ___     
|__  \_/ |__      |  /  \ /  \ |       |__) \ /     |\/| / _`     |__) |__  \_/ 
|___ / \ |___     |  \__/ \__/ |___    |__)  |      |  | \__> ___ |  \ |___ / \ 
"
    );
}

fn print_usage() {
    println!("Usage:");
    println!("exe_tool <options> <executable> <config_file> [text_file] [replace_table]");
    println!();
    println!("Options:");
    println!("-e - extract");
    println!("-i - insert -- text_file & replace_table required");
}

fn fail() -> ! {
    print_usage();
    std::process::exit(-1);
}
