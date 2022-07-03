use std::fs::OpenOptions;
use std::io::prelude::*;

use clap::Parser;

mod args_struct;
mod strs;

fn write_to_file(file: &str, text: &str, verbose: &bool) -> std::io::Result<()> {
    if *verbose {
        strs::debug(&format!("Opening file {} for reading", &file));
    };

    let mut f = OpenOptions::new().append(true).open(file)?;

    if *verbose {
        strs::debug(&format!("Writing \"{}\" to file {}", &text, &file));
    };

    f.write_all(text.as_bytes())?;

    Ok(())
}

fn main() {
    let args = args_struct::Args::parse();

    if args.verbose {
        strs::debug(&format!("{:?}", &args));
    }

    if !args.no_newline {
        write_to_file(&args.file, "\n", &args.verbose).unwrap_or_else(|err| {
            strs::error(&format!("Could not write newline to file, {}", err));
            std::process::exit(1);
        });
    };

    write_to_file(&args.file, &args.text, &args.verbose).unwrap_or_else(|err| {
        strs::error(&format!("Could not write text to file, {}", err));
        std::process::exit(1);
    });

    if !args.quiet {
        strs::info(&format!("Appended \"{}\" to {}", &args.text, &args.file));
        std::process::exit(0);
    }
}
