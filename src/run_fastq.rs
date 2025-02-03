use std::fs::File;

use anyhow::Result;
use fastq::Parser;

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = Parser::new(file);
    let mut num_records = 0;
    reader.each(|_| {
        num_records += 1;
        true
    })?;

    eprintln!("Number of records: {}", num_records);
    Ok(())
}
