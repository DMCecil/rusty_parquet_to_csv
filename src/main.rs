use std::path::PathBuf;

use polars::prelude::LazyFrame;

fn parquet_to_csv(input_path: &str, output_path: &str) {
    let output_path = PathBuf::from(&output_path);

    let lf = LazyFrame::scan_parquet( &input_path, Default::default()).unwrap();
        
    lf.sink_csv(output_path, Default::default()).unwrap();
    println!("Parquet file successfully converted!");
}

fn main() {
    let input_path_string = "input/pandas_dataframe_test.parquet.gzip";
    let output_path_string = "output/output.csv";
    
    parquet_to_csv(&input_path_string, &output_path_string);
}
