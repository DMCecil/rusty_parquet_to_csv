use std::path::PathBuf;

use polars::prelude::LazyFrame;

fn main() {
    let output_path_string = "output/output.csv";
    let output_path = PathBuf::from(output_path_string);

    let lf = LazyFrame::scan_parquet( "input/pandas_dataframe_test.parquet.gzip", Default::default()).unwrap();
        
    lf.sink_csv(output_path, Default::default()).unwrap();
    println!("Parquet file successfully converted!");
}
