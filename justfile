# 10M records
NUM_RECORDS := "10000000"

# Nucleotide sizes for the records
RECORD_SIZES := "30 50 100 200 300"

gen_data:
    @echo "Generating data"
    @mkdir -p data
    for size in {{RECORD_SIZES}}; do \
        echo "Generating records of size ${size}"; \
        nucgen -fq -n {{NUM_RECORDS}} -l $size data/records_${size}.fastq; \
    done

run_benchmarks:
    @echo "Running benchmarks"
    for size in {{RECORD_SIZES}}; do \
        hyperfine --warmup 10 --export-csv "results/benchmark_${size}.csv" \
            "psb -i data/records_${size}.fastq -l seq_io" \
            "psb -i data/records_${size}.fastq -l seq_io_batch" \
            "psb -i data/records_${size}.fastq -l needletail" \
            "psb -i data/records_${size}.fastq -l paraseq" \
            "psb -i data/records_${size}.fastq -l fastq-rs" \
            "psb -i data/records_${size}.fastq -l fastq-rs_batch" \
            "psb -i data/records_${size}.fastq -l bio"; \
    done
