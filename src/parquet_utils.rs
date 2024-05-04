// Standard imports
use std::io::Error;
use std::path::PathBuf;

//External crate imports
use polars::prelude::LazyFrame;


pub fn parquet_to_csv(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), Error> {
    /*
        Converts a Parquet file to a CSV file.

        # Arguments

        * `input_path`: The path to the input Parquet file.
        * `output_path`: The directory where the output CSV file will be saved. 
                    The output filename will be generated from the input filename with a `.csv` extension.

        # Returns

        * `Result<(), Error>`: Returns an empty `Ok(())` on success, or an `Error` if conversion fails.
    */
    let output_path = output_path.join(input_path.file_stem().unwrap()).with_extension("csv");

    // Scan parquet file into dataframe using Lazy API
    let lf = LazyFrame::scan_parquet( &input_path, Default::default()).unwrap();
    
    // Convert the file into csv with direct method
    lf.sink_csv(output_path, Default::default()).unwrap();

    Ok(())
}