mod run_bio;
mod run_fastq;
mod run_fastq_batch;
mod run_needletail;
mod run_paraseq;
mod run_seq_io;
mod run_seq_io_batch;

use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Arguments {
    /// Input fastq file to parse
    #[clap(short, long)]
    input: String,

    /// Library to use for parsing
    #[clap(short, long, default_value = "paraseq")]
    library: Library,
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Library {
    #[clap(name = "seq_io")]
    SeqIo,
    #[clap(name = "seq_io_batch")]
    SeqIoBatch,
    #[clap(name = "needletail")]
    Needletail,
    #[clap(name = "paraseq")]
    Paraseq,
    #[clap(name = "fastq-rs")]
    FastqRs,
    #[clap(name = "fastq-rs_batch")]
    FastqRsBatch,
    #[clap(name = "bio")]
    Bio,
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    match args.library {
        Library::SeqIo => run_seq_io::run(&args.input),
        Library::SeqIoBatch => run_seq_io_batch::run(&args.input),
        Library::Needletail => run_needletail::run(&args.input),
        Library::Paraseq => run_paraseq::run(&args.input),
        Library::FastqRs => run_fastq::run(&args.input),
        Library::FastqRsBatch => run_fastq_batch::run(&args.input),
        Library::Bio => run_bio::run(&args.input),
    }
}
