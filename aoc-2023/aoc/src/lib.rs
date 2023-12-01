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
    let contents = read_to_string(&file_name)
        .expect(format!("cannot open input file {}", &file_name.to_string_lossy()).as_str());

    contents
}

pub fn read_input_lines(day: u8) -> Vec<String> {
    let file_name = get_inputs_dir().join(format!("{:02}.in", day));
    let file = File::open(&file_name)
        .expect(format!("cannot open input file {}", &file_name.to_string_lossy()).as_str());

    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(line_content) = line {
            if !line_content.trim().is_empty() {
                lines.push(line_content)
            }
        }
    }

    lines
}
