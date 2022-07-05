use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;

mod args_struct;
mod strs;

fn write_to_file(file: &str, text: &str, verbose: &bool) -> std::io::Result<()> {
    if *verbose {
        strs::debug(&format!("Opening file {} for writing", &file));
    };

    let mut f = OpenOptions::new().append(true).open(file)?;

    if *verbose {
        if text == "\n" {
            strs::debug(&format!("Appending newline to file {}", &file));
        } else {
            strs::debug(&format!("Appending \"{}\" to file {}", &text, &file));
        }
    };

    f.write_all(text.as_bytes())?;

    Ok(())
}

fn main() {
    let args = args_struct::Args::parse();

    if args.verbose {
        strs::debug(&format!("{:?}", &args));
    }

    if !args.no_create {
        if !Path::new(&args.file).exists() {
            if args.verbose {
                strs::debug(&format!("Creating file {}", &args.file));
            }
            std::fs::File::create(&args.file).unwrap_or_else(|err| {
                strs::error(&format!("Could not create file, {}", err));
                std::process::exit(3);
            });
        };
    }

    let newline = if std::fs::metadata(&args.file).unwrap_or_else(
        |err| {
            strs::error(&format!("Could not get metadata for file, {}", err));
            std::process::exit(2);
        }
    ).len() == 0 {
        ""
    } else {
        "\n"
    };

    if !args.no_newline {
        write_to_file(&args.file, newline, &args.verbose).unwrap_or_else(|err| {
            strs::error(&format!("Could not write to file, {}", err));
            std::process::exit(1);
        });
    };

    write_to_file(&args.file, &args.text, &args.verbose).unwrap_or_else(|err| {
        strs::error(&format!("Could not write to file, {}", err));
        std::process::exit(1);
    });

    if !args.quiet {
        if args.no_newline {
            strs::info(&format!("Appended \"{}\" to {} without a newline", &args.text, &args.file));
        } else {
            strs::info(&format!("Appended \"{}\" to {}", &args.text, &args.file));
        }
        std::process::exit(0);
    }
}
