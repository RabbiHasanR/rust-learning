use csv::ReaderBuilder;
use rayon::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;
use std::time::Instant;

pub fn validation_check() -> Result<(), Box<dyn std::error::Error>> {
    // Start measuring time
    let start_time = Instant::now();

    // Specify the input directory containing CSV files
    let csv_files_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_csv_files";

    // Collect CSV file paths
    let csv_files: Vec<_> = WalkDir::new(csv_files_dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() && entry.path().extension()? == "csv" {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        })
        .collect();

    println!("Found CSV files:");
    for file in &csv_files {
        println!("{}", file.display());
    }

    // Process each CSV file
    csv_files.par_iter().for_each(|file_path| {
        if let Err(e) = process_csv_file(file_path) {
            eprintln!("Error processing file {}: {}", file_path.display(), e);
        }
    });


    // Calculate total elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "Completed processing in {:.2?} seconds",
        elapsed_time.as_secs_f64()
    );

    Ok(())
}

// Function to process a single CSV file
fn process_csv_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing file: {}", file_path.display());

    // Open the CSV file
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Adjust based on your CSV files
        .from_reader(file);

    // Get headers if present
    let headers = rdr.headers()?.clone();

    // Process rows in chunks
    const CHUNK_SIZE: usize = 1000;
    let mut records = Vec::with_capacity(CHUNK_SIZE);
    let mut iter = rdr.records();

    loop {
        // Read records into the buffer
        for _ in 0..CHUNK_SIZE {
            match iter.next() {
                Some(result) => {
                    let record = result?;
                    records.push(record);
                }
                None => break,
            }
        }

        if records.is_empty() {
            break; // No more records to process
        }

        // Clone headers for thread-safe access
        let headers = headers.clone();

        // Process the chunk of records in parallel
        process_records(&headers, &records);

        records.clear(); // Clear the buffer for the next chunk
    }

    println!("Finished processing file: {}", file_path.display());
    Ok(())
}

// Function to process a chunk of records in parallel
fn process_records(headers: &csv::StringRecord, records: &[csv::StringRecord]) {
    // Use Arc for thread-safe reference counting if needed
    let headers = Arc::new(headers.clone());

    // Process records in parallel
    records.par_iter().for_each(|record| {
        // Access headers
        let headers = headers.clone();

        // Create a HashMap of header-value pairs
        let json_map: HashMap<&str, &str> = headers
            .iter()
            .zip(record.iter())
            .collect();

        // Convert the HashMap to a JSON value
        let json_value = json!(json_map);

        // Convert the JSON value to a string
        let json_string = serde_json::to_string(&json_value).unwrap();

        // Print the JSON string
        // println!("{}", json_string);
    });
}
