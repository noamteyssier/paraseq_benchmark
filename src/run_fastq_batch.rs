use std::fs::File;

use anyhow::Result;
use fastq::Parser;

pub fn run(path: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = Parser::new(file);
    let mut num_records = 0;
    let result: std::io::Result<Vec<_>> = reader.parallel_each(1, |record_sets| {
        let mut num_records = 0;
        for rset in record_sets {
            for record in rset.iter() {
                let _ = record;
                num_records += 1;
            }
        }
        num_records
    });
    match result {
        Ok(records) => {
            for r in records {
                num_records += r;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    eprintln!("Number of records: {}", num_records);
    Ok(())
}
