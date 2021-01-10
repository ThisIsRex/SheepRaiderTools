pub enum Mode {
    Decompile,
    Recompile,
}

const PARAM_DECOMPILE: &str = "-d";
const PARAM_RECOMPILE: &str = "-r";

pub struct Parameters {
    pub mode: Mode,
    pub replace_table_file: Option<String>,
    pub files: Vec<String>,
}

impl Parameters {
    pub fn parse(args: &[String]) -> Option<Parameters> {
        if args.len() < 1 {
            return None;
        }

        let mode: Mode;
        let mut start: usize = 2;

        if args[0].starts_with("-") {
            if args[0] == PARAM_RECOMPILE {
                mode = Mode::Recompile;
                if args.len() < 3 {
                    return None;
                }
            } else if args[0] == PARAM_DECOMPILE {
                mode = Mode::Decompile;
            } else {
                return None;
            }
        } else {
            mode = Mode::Decompile;
            start = 0;
        }

        let s_files = &args[start..args.len()];
        if s_files.len() == 0 {
            return None;
        }

        let files = s_files.iter().map(|f| String::from(f)).collect();

        Some(Parameters {
            mode: mode,
            replace_table_file: Some(args[1].to_string()),
            files: files,
        })
    }
}
