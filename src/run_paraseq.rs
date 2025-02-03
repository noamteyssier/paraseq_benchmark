use std::fs::File;

use anyhow::Result;
use paraseq::fastq::{Reader, RecordSet};

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = Reader::new(file);
    let mut rset = RecordSet::default();
    let mut num_records = 0;
    while rset.fill(&mut reader)? {
        for record in rset.iter() {
            let _record = record?;
            num_records += 1;
        }
    }
    eprintln!("Number of records: {}", num_records);
    Ok(())
}
