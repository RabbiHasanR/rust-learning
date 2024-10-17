use csv::ReaderBuilder;
use rust_xlsxwriter::{Workbook};
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::time::Instant;

pub fn convert_csv_to_excel() -> Result<(), Box<dyn Error>> {
    // Start measuring time
    let start_time = Instant::now();
    // Specify the input directory containing CSV files
    let csv_files_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_csv_files";

    // Specify the output directory for Excel files
    let output_excel_dir = "/run/media/agl-000024/extra/Personal_work/rust-learning/excel-file-processor/output_excel_files";

    // Ensure the output directory exists
    std::fs::create_dir_all(output_excel_dir)?;

    // Collect CSV file paths
    let mut csv_files: Vec<PathBuf> = WalkDir::new(csv_files_dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file()
                && entry.path().extension()?.to_ascii_lowercase() == "csv"
            {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        })
        .collect();

    // Sort the files to ensure consistent order
    csv_files.sort();

    // Process each CSV file individually
    for csv_file_path in &csv_files {
        // Get the file name without extension to use as base for Excel file
        let file_stem = csv_file_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        // Create the output Excel file path
        let output_excel_file = Path::new(output_excel_dir)
            .join(format!("{}.xlsx", file_stem));

        // Open the CSV file
        let file = File::open(csv_file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        // Create a new Excel workbook
        let mut workbook = Workbook::new();

        // Add a new worksheet
        let worksheet = workbook.add_worksheet();

        // Read and write each record
        let mut row_num = 0;

        // Write the headers if they exist
        if rdr.has_headers() {
            let headers = rdr.headers()?;
            for (col_num, header) in headers.iter().enumerate() {
                worksheet.write_string(row_num, col_num as u16, header)?;
            }
            row_num += 1;
        }

        // Read and process each record
        for result in rdr.records() {
            let record = result?;
            for (col_num, field) in record.iter().enumerate() {
                worksheet.write_string(row_num, col_num as u16, field)?;
            }
            row_num += 1;
        }

        // Save the workbook to a file
        workbook.save(&output_excel_file)?;

        println!(
            "Excel file saved to {}",
            output_excel_file.to_string_lossy()
        );
    }

    // Calculate total elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "Completed processing in {:.2?} seconds",
        elapsed_time.as_secs_f64()
    );

    Ok(())
}
