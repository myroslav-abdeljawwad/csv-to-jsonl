use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use clap::{ArgGroup, Parser, ValueEnum};
use csv::StringRecord;

/// Stream large CSV files to line‑delimited JSON for easy ingestion.
/// Built by Myroslav Mokhammad Abdeljawwad
#[derive(Parser)]
#[command(name = "csv-to-jsonl")]
#[command(about = "Convert CSV input to JSONL output", long_about = None)]
#[command(author, version)]
#[arg_group(
    name = "input",
    required = true,
    args = ["file", "stdin"]
)]
struct Cli {
    /// Input CSV file. If omitted, reads from stdin.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Write output to a file instead of stdout.
    #[arg(short, long, value_name = "OUT_FILE")]
    out_file: Option<PathBuf>,

    /// Use header row as JSON keys. If false, numeric indices are used.
    #[arg(long, default_value_t = true)]
    has_header: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Open input
    let reader: Box<dyn std::io::Read> = match args.file {
        Some(path) => Box::new(File::open(&path)?),
        None => Box::new(io::stdin()),
    };

    // Prepare output writer
    let writer: Box<dyn Write> = match args.out_file {
        Some(out_path) => Box::new(BufWriter::new(File::create(out_path)?)),
        None => Box::new(io::stdout()),
    };
    process_csv(reader, writer, args.has_header)?;

    Ok(())
}

fn process_csv<R: std::io::Read, W: Write>(
    mut reader: R,
    mut writer: W,
    has_header: bool,
) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .from_reader(reader);

    // Determine headers
    let headers = if has_header {
        Some(csv_reader.headers()?.clone())
    } else {
        None
    };

    for result in csv_reader.records() {
        let record = result?;
        let json_obj = build_json(&record, &headers);
        serde_json::to_writer(&mut writer, &json_obj)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn build_json(record: &StringRecord, headers: &Option<csv::StringRecord>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (i, field) in record.iter().enumerate() {
        let key = match headers {
            Some(hdrs) => hdrs.get(i).unwrap_or(&format!("column{}", i)),
            None => &format!("column{}", i),
        };
        map.insert(key.to_string(), serde_json::Value::String(field.to_string()));
    }
    serde_json::Value::Object(map)
}