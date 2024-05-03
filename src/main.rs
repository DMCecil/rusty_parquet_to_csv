use std::io::Error;
use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use polars::prelude::LazyFrame;
use rayon::prelude::*;
use walkdir::WalkDir;

fn parquet_to_csv(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), Error> {
    let output_path = output_path.join(input_path.file_stem().unwrap()).with_extension("csv");

    // Scan parquet file into dataframe using Lazy API
    let lf = LazyFrame::scan_parquet( &input_path, Default::default()).unwrap();
    
    // Convert the file into csv with direct method
    lf.sink_csv(output_path, Default::default()).unwrap();

    Ok(())
}

fn main() -> Result<(), Error> {
    let input_dir = "input/";
    let output_dir = "output/";
    
    // Find all .parquet.gzip files in the directory
    let parquet_files = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".parquet.gzip"))  
                .unwrap_or(false)
        })
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<_>>();

    // Create a progress bar 
    let pb = ProgressBar::new(parquet_files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Convert files in parallel, updating the progress bar
    let output_dir_path = PathBuf::from(output_dir);
    parquet_files.par_iter().for_each(|parquet_file| {
        if let Err(e) = parquet_to_csv(parquet_file, &output_dir_path) {
            eprintln!("Error converting {}: {}", parquet_file.display(), e)
        }
        pb.inc(1);
    });

    pb.finish_with_message("Conversion Complete!");
    Ok(())
}
