## Parquet-to-CSV Converter: A Rusty Adventure 🚣‍♀️

Tired of clunky Parquet files? Let's turn them into beautiful CSV data with ease! This script is your command-line friend that effortlessly converts those Parquet files into CSVs while keeping you updated with a snazzy progress bar.

### Features

* **Speedy Conversion:** Zips through Parquet files and transforms them into CSVs in a flash.
* **Customizable:** Point it to your input and output directories for a tailored experience.
* **Progress Tracking:** No more waiting in the dark! Watch the conversion magic happen with a fancy progress bar.

### Getting Started

1. **Make sure Rust is installed:** Check out the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. **Download or clone this project.**
3. **Build it:** Open your terminal and run `cargo build` in the project directory. 
4. **Let the conversion party begin!**  

   ```bash
   ./your_application --input_dir ./my_parquet_stash --output_dir ./csv_files --jobs n_jobs
   ```

   ```powershell
   .\your_application.exe --input_dir ./my_parquet_stash --output_dir ./csv_files --jobs n_jobs
   ```

### Why Does This Exist?
- Because sometimes you just need CSVs!
- Parquet files can be a bit unapproachable for quick analysis.
- Progress bars make waiting way more fun.

### Built With
- Love for data ❤️
- The power of Rust 🦀
- The awesome Polars library for data manipulation 🐻‍❄️
- The fantastic Clap crate for command-line superpowers ⚡

### Disclaimer
This script won't magically make your data problems disappear, but it might make them a little easier to handle.

### Feedback?
Have a suggestion? Found a bug? Want to share your Parquet-to-CSV success story? Feel free to open an issue or drop me a line!
