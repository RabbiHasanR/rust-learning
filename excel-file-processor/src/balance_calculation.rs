use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use walkdir::WalkDir;
use std::time::Instant;

pub fn balance_calculation() -> Result<(), Box<dyn Error>> {
    // Start measuring time
    let start_time = Instant::now();
    // Specify the input directory containing CSV files
    let csv_files_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_csv_files";

    // Collect CSV file paths
    let mut csv_files: Vec<_> = WalkDir::new(csv_files_dir)
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

    // Sort the files to ensure consistent order
    csv_files.sort();

    // Print all CSV file paths one by one
    println!("Found CSV files:");
    for file in &csv_files {
        println!("{}", file.display());
    }

    // Open all CSV files for reading and create temporary files for writing
    let mut readers = Vec::new();
    let mut writers = Vec::new();
    let mut temp_files = Vec::new();

    for file_path in &csv_files {
        // Open the CSV file for reading
        let file = File::open(file_path)?;
        let reader = ReaderBuilder::new()
            .has_headers(true) // Adjust based on your CSV files
            .from_reader(file);
        readers.push(reader);

        // Create a temporary file for writing
        let temp_file = NamedTempFile::new_in(csv_files_dir)?;
        let writer = WriterBuilder::new()
            .has_headers(true)
            .from_writer(temp_file.reopen()?);
        writers.push(writer);
        temp_files.push(temp_file);
    }

    // Get headers from each reader and write updated headers to writers
    let headers_list: Vec<_> = readers
        .iter_mut()
        .enumerate()
        .map(|(i, reader)| {
            let mut headers = reader.headers().unwrap().clone();
            headers.push_field("new_column");
            writers[i].write_record(&headers).unwrap();
            headers
        })
        .collect();

    // Read and process rows in synchronization
    loop {
        let mut records = Vec::new();
        let mut all_eof = true;

        // Read one record from each reader
        for (i, reader) in readers.iter_mut().enumerate() {
            match reader.records().next() {
                Some(Ok(mut record)) => {
                    all_eof = false;

                    // Add the new column with value 'hello'
                    record.push_field("hello");

                    // Write the updated record to the corresponding writer
                    writers[i].write_record(&record).unwrap();

                    records.push(Some((i, record)));
                }
                Some(Err(e)) => {
                    eprintln!(
                        "Error reading record from file {}: {}",
                        csv_files[i].display(),
                        e
                    );
                    records.push(None);
                }
                None => {
                    records.push(None);
                }
            }
        }

        if all_eof {
            break; // All readers have reached EOF
        }
    }

    // Flush and close all writers
    for writer in &mut writers {
        writer.flush()?;
    }

    // Replace original files with the updated temporary files
    for (i, temp_file) in temp_files.into_iter().enumerate() {
        let original_path = &csv_files[i];
        temp_file.persist(&original_path)?;
        println!("Updated file: {}", original_path.display());
    }


    // Calculate total elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "Completed processing in {:.2?} seconds",
        elapsed_time.as_secs_f64()
    );

    Ok(())
}
