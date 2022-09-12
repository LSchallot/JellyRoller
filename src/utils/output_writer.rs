
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn export_data(data: &str, path: String) {
    let f = File::create(path).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}