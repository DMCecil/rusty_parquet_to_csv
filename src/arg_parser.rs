use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "Duarte Cecílio", version = "0.2.0", about = "Converts Parquet files to CSV with progress bar", long_about = "This script concurrently converts multiple Parquet files to CSV while providing a user-friendly progress bar.")] 
pub struct Args {
    /// Input directory containing Parquet files
    #[arg(short = 'i', long = "input_dir")] 
    pub input_dir: String,

    /// Output directory for converted CSV files
    #[arg(short = 'o', long = "output_dir")]
    pub output_dir: String,

    /// Number of jobs to be spun up concurrently
    #[arg(short = 'j', long = "jobs", default_value_t = num_cpus::get())]
    pub jobs: usize,
}