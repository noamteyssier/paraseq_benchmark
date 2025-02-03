use std::fs::File;

use anyhow::Result;
use needletail::{parser::FastqReader, FastxReader};

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = FastqReader::new(file);
    let mut num_records = 0;
    while let Some(record) = reader.next() {
        let _record = record?;
        num_records += 1;
    }
    eprintln!("Number of records: {}", num_records);
    Ok(())
}
