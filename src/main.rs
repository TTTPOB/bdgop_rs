use bdgop_rs::ops::print_bed;
use bdgop_rs::{self, bedlike_record};
use clap::{self, Parser, Subcommand};
use csv;
use std::env;
use std::io;

#[derive(Parser)]
#[clap(name = "bdgops", about = "Bed(graph) like file operations")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    PrintBed {
        file: Option<String>,
        delim: Option<char>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::PrintBed { file, delim } => {
            print_bed(file, delim);
        }
    }
}
