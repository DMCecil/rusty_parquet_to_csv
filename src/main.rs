// Standard imports
use std::path::PathBuf;
use clap::Parser;

// Internal modules import
mod arg_parser;
mod parquet_utils;

// Struct and function import from internal modules
use arg_parser::Args;
use parquet_utils::parquet_to_csv;

// External crate imports
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use walkdir::WalkDir;

#[derive(Debug)]
struct NoParquetFilesError;

impl std::fmt::Display for NoParquetFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No Parquet files found in the specified directory.")
    }
}

impl std::error::Error for NoParquetFilesError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_dir = &args.input_dir;
    let output_dir = &args.output_dir;
    let num_jobs = args.jobs;
    
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

    if parquet_files.is_empty() {
        return Err(Box::new(NoParquetFilesError));
    }

    // Create a progress bar 
    let pb = ProgressBar::new(parquet_files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Convert files in parallel, updating the progress bar
    let output_dir_path = PathBuf::from(output_dir);
    parquet_files.par_iter().with_max_len(num_jobs).for_each(|parquet_file| {
        if let Err(e) = parquet_to_csv(parquet_file, &output_dir_path) {
            eprintln!("Error converting {}: {}", parquet_file.display(), e)
        }
        pb.inc(1);
    });

    pb.finish_with_message("Conversion Complete!");
    Ok(())
}
