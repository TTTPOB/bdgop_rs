pub mod bedlike_record {
    use csv;
    use std::fmt;

    pub struct BedLikeRecord<'a> {
        chrom: &'a str,
        start: &'a str,
        end: &'a str,
        other_fields: Vec<&'a str>,
    }

    // construct a new BedLikeRecord from a CSV row
    pub fn new(row: &csv::StringRecord) -> BedLikeRecord {
        BedLikeRecord {
            chrom: &row[0],
            start: &row[1],
            end: &row[2],
            other_fields: row.iter().skip(3).collect(),
        }
    }
    impl<'a> fmt::Display for BedLikeRecord<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}\t{}\t{}", self.chrom, self.start, self.end)?;
            for field in &self.other_fields {
                write!(f, "\t{}", field)?;
            }
            Ok(())
        }
    }
}

pub mod io {

    use csv::{Reader, StringRecord};

    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Read};

    pub fn read_from_in_stream<T: io::BufRead>(
        in_stream: T,
        delim: Option<char>,
    ) -> csv::Reader<T> {
        let delim = delim.unwrap_or('\t');

        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delim as u8)
            .from_reader(in_stream);
        reader
    }

    pub fn read(
        infile: Option<String>,
        delim: Option<char>,
    ) -> csv::Reader<Box<dyn BufRead>> {
        // if infile is none, read from stdin
        let stream_reader: Box<dyn BufRead> = match infile {
            Some(infile) => {
                let file = File::open(infile).unwrap();
                let buf_reader = io::BufReader::new(file);
                Box::new(buf_reader)
            }
            None => {
                let buf_reader = io::BufReader::new(io::stdin());
                Box::new(buf_reader)
            }
        };
        let csv_reader = read_from_in_stream(stream_reader, delim);
        csv_reader
    }
}

pub mod ops;
