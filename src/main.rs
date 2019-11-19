use serde_json::Value;
use std::env::args;
use std::fs;

fn main() {
    let args = args().collect::<Vec<String>>();
    let in_path = &args
        .get(1)
        .expect("Syntax error, expected input json path as first argument.");
    let out_path = args.get(2).unwrap_or(in_path);
    let in_json = fs::read_to_string(in_path).unwrap();
    let v: Value = serde_json::from_str(&in_json).unwrap();
    drop(in_json);
    let out_json = serde_json::to_string_pretty(&v).unwrap();
    fs::write(out_path, out_json).unwrap();
}

// This seems to be slower, TODO: revisit later.
//fn with_readers_and_writers_seems_to_be_slow() {
//    use serde_json::from_reader;
//    use serde_json::to_writer_pretty;
//    use std::fs::File;
//    use std::io::BufReader;
//    use std::io::BufWriter;
//    let mut in_file = File::open(in_path)
//        .expect(&format!("Failed to read file: {}", in_path));
//    let br = BufReader::new(in_file);
//    let v: Value = from_reader(br).unwrap();
//    let bw = BufWriter::new(File::create(out_path).unwrap());
//    to_writer_pretty(bw, &v).unwrap();
//}
