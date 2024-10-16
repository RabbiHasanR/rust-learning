use csv::ReaderBuilder;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use std::time::Instant;

pub fn validation_cehck() -> Result<(), Box<dyn std::error::Error>> {
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

    // Open all CSV files and create readers
    let mut readers = Vec::new();
    for file_path in &csv_files {
        let file = File::open(file_path)?;
        let reader = ReaderBuilder::new()
            .has_headers(true) // Adjust based on your CSV files
            .from_reader(file);
        readers.push(reader);
    }

    // Get headers from each reader
    let headers_list: Vec<_> = readers
        .iter_mut()
        .map(|reader| reader.headers().unwrap().clone())
        .collect();

    // Read and process rows in synchronization
    loop {
        let mut records = Vec::new();
        let mut all_eof = true;

        // Read one record from each reader
        for (i, reader) in readers.iter_mut().enumerate() {
            match reader.records().next() {
                Some(Ok(record)) => {
                    all_eof = false;
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

        // Process the collected records
        process_synchronized_records(&headers_list, &records, &csv_files);
    }


    // Calculate total elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "Completed processing in {:.2?} seconds",
        elapsed_time.as_secs_f64()
    );


    Ok(())
}

// Function to process synchronized records
fn process_synchronized_records(
    headers_list: &[csv::StringRecord],
    records: &[Option<(usize, csv::StringRecord)>],
    csv_files: &[std::path::PathBuf],
) {
    for (index, record_option) in records.iter().enumerate() {
        if let Some((i, record)) = record_option {
            let headers = &headers_list[*i];
            let json_map: HashMap<&str, &str> = headers.iter().zip(record.iter()).collect();

            // Convert the HashMap to a JSON value
            let json_value = json!(json_map);

            // Convert the JSON value to a string
            let json_string = serde_json::to_string(&json_value).unwrap();

            // Print the JSON string with file identifier
            // println!("File {}: {}", csv_files[*i].display(), json_string);
            // println!("prcess row")
        }
        // Do not print anything if the record is None (EOF reached for that file)
    }

    // Separator for each synchronized row set
    // println!("--- End of synchronized row set ---");
}
