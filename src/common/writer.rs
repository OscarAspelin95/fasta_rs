use serde::Serialize;
use serde_json;
use std::path::PathBuf;
use std::{fs::File, io::BufWriter};

pub fn write_json<T: Serialize>(outfile: &PathBuf, s: T) {
    let outbuf = File::create(outfile).unwrap();
    let writer = BufWriter::new(outbuf);

    serde_json::to_writer(writer, &s).unwrap();
}
