use calamine::{open_workbook_auto, DataType, Reader};
use chrono::{Duration, NaiveDate};
use csv::WriterBuilder;
use rayon::prelude::*;
use sanitize_filename::sanitize;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn convert_excel_to_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Start measuring time
    let start_time = Instant::now();

    // Specify the input Excel file path
    let excel_file_path = "/run/media/agl-000024/extra/Office/clerk_accounting/media/files/valid_data_large_v_2.xlsx";

    // Specify the output directory for CSV files
    let output_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_csv_files";

    // Ensure the output directory exists
    create_dir_all(&output_dir)?;
    println!("Output directory created or already exists: {}", output_dir);

    // Open the Excel workbook
    let mut workbook = open_workbook_auto(&excel_file_path)?;
    println!("Workbook opened successfully: {}", excel_file_path);

    // Get and print sheet names
    let sheet_names = workbook.sheet_names().to_owned();
    println!("Found sheets: {:?}", sheet_names);

    // Arc and Mutex to share the workbook across threads safely
    let workbook = Arc::new(Mutex::new(workbook));

    // Process sheets in parallel
    sheet_names.par_iter().for_each(|sheet_name| {
        let sheet_start_time = Instant::now();
        println!("Processing sheet: {}", sheet_name);

        let mut workbook = workbook.lock().unwrap();
        match workbook.worksheet_range(sheet_name) {
            Some(Ok(range)) => {
                drop(workbook); // Release the lock as soon as possible

                // Sanitize the sheet name for file naming
                let sanitized_sheet_name = sanitize(sheet_name);

                // Create a CSV file for the current sheet
                let csv_file_name = format!("{}.csv", sanitized_sheet_name);
                let csv_file_path = Path::new(&output_dir).join(csv_file_name);
                let csv_file = match File::create(&csv_file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Failed to create CSV file: {}", e);
                        return;
                    }
                };
                println!("Created CSV file at: {}", csv_file_path.display());

                let buf_writer = BufWriter::new(csv_file);
                let mut csv_writer = WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(buf_writer);

                // Iterate over rows in the sheet and write to CSV
                let mut row_count = 0;
                for row in range.rows() {
                    let record: Vec<String> = row
                        .iter()
                        .map(|cell| cell_to_string(cell))
                        .collect();
                    if let Err(e) = csv_writer.write_record(&record) {
                        eprintln!("Error writing record: {}", e);
                        return;
                    }
                    row_count += 1;
                }

                // Flush the CSV writer to ensure all data is written
                if let Err(e) = csv_writer.flush() {
                    eprintln!("Error flushing CSV writer: {}", e);
                    return;
                }
                println!(
                    "Sheet '{}' has been written to '{}', total rows: {}",
                    sheet_name,
                    csv_file_path.display(),
                    row_count
                );

                // Calculate elapsed time for the sheet
                let sheet_elapsed_time = sheet_start_time.elapsed();
                println!(
                    "Processing of sheet '{}' completed in {:.2?} seconds",
                    sheet_name,
                    sheet_elapsed_time.as_secs_f64()
                );
            }
            Some(Err(e)) => {
                eprintln!("Error reading sheet '{}': {:?}", sheet_name, e);
            }
            None => {
                eprintln!("Sheet '{}' not found.", sheet_name);
            }
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

/// Converts a cell to a string representation.
fn cell_to_string(cell: &DataType) -> String {
    match cell {
        DataType::Empty => String::new(),
        DataType::String(s) => s.clone(),
        DataType::Float(f) => {
            if is_date(*f) {
                excel_serial_number_to_date(*f)
            } else {
                f.to_string()
            }
        }
        DataType::Int(i) => i.to_string(),
        DataType::Bool(b) => b.to_string(),
        DataType::Error(e) => format!("Error: {:?}", e),
        DataType::DateTime(f) => {
            // Handle DateTime variant (represented as a float)
            excel_serial_number_to_date(*f)
        }
        DataType::DateTimeIso(s) => {
            // Already in ISO 8601 format
            s.clone()
        }
        DataType::DurationIso(s) => {
            // Duration in ISO 8601 format
            s.clone()
        }
        _ => {
            // Handle any future variants
            format!("{:?}", cell)
        }
    }
}

/// Determines if a floating-point number represents an Excel date.
fn is_date(value: f64) -> bool {
    // Adjust this logic based on your data
    value > 1.0
}

/// Converts an Excel serial date number to a readable date string.
fn excel_serial_number_to_date(serial: f64) -> String {
    // Excel's base date is 1899-12-30
    let base_date = NaiveDate::from_ymd(1899, 12, 30);
    let days = serial.trunc() as i64;
    let fractional_day = serial - serial.trunc();
    let seconds = (fractional_day * 86400.0).round() as i64;

    if let Some(date) = base_date.checked_add_signed(Duration::days(days)) {
        let date_time = date.and_hms(0, 0, 0) + Duration::seconds(seconds);
        date_time.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        serial.to_string()
    }
}








// let path = "/run/media/agl-000024/extra/Office/clerk_accounting/media/files/valid_data_large_v_2.xlsx";
// let output_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_csv_files";