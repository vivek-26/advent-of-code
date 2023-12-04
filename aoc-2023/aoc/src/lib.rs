use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn get_inputs_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("inputs");

    path
}

pub fn read_input(day: u8) -> String {
    let file_name = get_inputs_dir().join(format!("{:02}.in", day));

    read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("cannot open input file {}", &file_name.to_string_lossy()))
}

pub fn read_input_lines(day: u8) -> Vec<String> {
    let file_name = get_inputs_dir().join(format!("{:02}.in", day));
    let file = File::open(&file_name)
        .unwrap_or_else(|_| panic!("cannot open input file {}", &file_name.to_string_lossy()));

    let reader = io::BufReader::new(file);

    reader.lines().flatten().collect()
}
