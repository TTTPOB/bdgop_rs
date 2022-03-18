use bdgop_rs::{
    bdg_record::{self, BdgRecord},
    bed_record::{self, BedLikeRecord},
};
use csv::{Reader, ReaderBuilder, StringRecord, WriterBuilder};
use serde::Serialize;
use std::error::Error;
use std::result::Result;

use clap::{self, arg, Command};

#[derive(clap::Subcommand)]
enum Action {
    print_bdg,
    print_bed,
}

#[derive(clap::Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn print_bdg(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(file_path)?;

    let mut write = WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(std::io::stdout());

    for result in reader.deserialize() {
        let record: BdgRecord = result.unwrap();
        write.serialize(record).unwrap();
    }

    Ok(())
}

fn print_bed(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(file_path)?;

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(std::io::stdout());

    for result in reader.deserialize() {
        let record: BedLikeRecord = result.unwrap();
        writer.serialize(record).unwrap();
    }

    Ok(())
}

fn main() {
    let matches = Command::new("bdgop")
        .about("bedgraph operations")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("print_bdg")
                .about("print bdg records")
                .arg(arg!( <FILE> "the input file")),
        )
        .subcommand(
            Command::new("print_bed")
                .about("print bed records")
                .arg(arg!( <FILE> "the input file")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("print_bdg", sub_matches)) => {
            let file = sub_matches.value_of("FILE").unwrap();
            print_bdg(file).unwrap();
        }

        Some(("print_bed", sub_matches)) => {
            let file = sub_matches.value_of("FILE").unwrap();
            print_bed(file).unwrap();
        }

        _ => {
            println!("{}", String::from("please see the help"));
        }
    }
}
