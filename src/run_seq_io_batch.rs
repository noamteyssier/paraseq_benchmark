use std::fs::File;

use anyhow::Result;
use seq_io::fastq::{Reader, RecordSet};

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let mut reader = Reader::new(file);
    let mut rset = RecordSet::default();
    let mut num_records = 0;
    while let Some(res) = reader.read_record_set(&mut rset) {
        res?;
        for _record in rset.into_iter() {
            num_records += 1;
        }
    }
    eprintln!("Number of records: {}", num_records);
    Ok(())
}
