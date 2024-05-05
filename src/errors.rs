use std::fmt;

#[derive(Debug)]
pub struct NoParquetFilesError;

impl fmt::Display for NoParquetFilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No Parquet files found in the specified directory")
    }
}

impl std::error::Error for NoParquetFilesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}