#![forbid(warnings)]
#![forbid(unsafe_code)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

enum Input {
    Stdin,
    File(PathBuf),
}

enum Output {
    Stdout,
    File(PathBuf),
}

struct IoModes {
    input: Input,
    output: Output,
}

struct Args {
    help: bool,
    output_file: Option<String>,
    minimize: bool,
    free: Vec<String>,
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        help: args.contains(["-h", "--help"]),
        output_file: args.opt_value_from_str("-o")?,
        minimize: args.contains(["-m", "--minimize"]),
        free: args.free()?,
    };
    Ok(args)
}

fn resolve_io_modes(args: &Args) -> IoModes {
    if args.free.len() > 1 {
        panic!(
            "Syntax error, found multiple input filenames: {:?}",
            args.free
        );
    }
    if args.free.len() == 1 {
        let input_file = PathBuf::from(&args.free[0]);
        let input = Input::File(input_file.clone());
        let output = args
            .output_file
            .as_ref()
            .map(|s| Output::File(PathBuf::from(s)))
            .unwrap_or_else(|| Output::File(input_file));
        IoModes { input, output }
    } else {
        let input = Input::Stdin;
        let output = args
            .output_file
            .as_ref()
            .map(|s| Output::File(PathBuf::from(s)))
            .unwrap_or(Output::Stdout);
        IoModes { input, output }
    }
}

fn main() {
    let args = parse_args().unwrap();
    if args.help {
        println!("Usage: jsonfmt [optional input filename] [flags]");
        println!();
        println!("  -o [filename]    Write output to a new file.");
        println!("  -m               Minimize instead of prettify.");
        return;
    }
    let io_modes = resolve_io_modes(&args);
    let v = match io_modes.input {
        Input::File(path) => {
            // This currently seems to be faster than using BufReader and
            // serde_json::from_reader, at the cost of memory usage.
            // TODO: Review and revisit later.
            let s = fs::read_to_string(path).unwrap();
            let v: serde_json::Value = serde_json::from_str(&s).unwrap();
            v
        }
        Input::Stdin => {
            let stdin = io::stdin();
            let handle = stdin.lock();
            let bytes = 1024 * 1024;
            let br = BufReader::with_capacity(bytes, handle);
            let v: serde_json::Value = serde_json::from_reader(br).unwrap();
            v
        }
    };
    match io_modes.output {
        Output::File(path) => {
            let f = File::create(path).unwrap();
            serialize(v, &args, f);
        }
        Output::Stdout => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            serialize(v, &args, handle);
        }
    }
}

fn serialize<W: std::io::Write>(v: serde_json::Value, args: &Args, w: W) {
    let bytes = 1024 * 1024;
    let bw = BufWriter::with_capacity(bytes, w);
    let write_json = if args.minimize {
        serde_json::to_writer
    } else {
        serde_json::to_writer_pretty
    };
    write_json(bw, &v).unwrap();
}
