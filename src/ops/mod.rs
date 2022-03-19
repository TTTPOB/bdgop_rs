use crate::io::read;
use std::io;

pub fn print_bed(file: Option<String>, delim: Option<char>) {
    let mut csv_reader = read(file, delim);
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    for row in csv_reader.records() {
        let record = row.unwrap();
        // print the row
        writer.write_record(&record).unwrap();
    }
}
