use std::fs::File;

use anyhow::Result;
use bio::io::fastq::Reader;

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = Reader::new(file);
    let mut num_records = 0;
    for record in reader.records() {
        let _record = record?;
        num_records += 1;
    }
    eprintln!("Number of records: {}", num_records);
    Ok(())
}
