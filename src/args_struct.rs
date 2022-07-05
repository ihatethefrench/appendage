use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(name=env!("CARGO_PKG_NAME"), version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    /// File to append to
    #[clap(name = "File", index = 1)]
    pub file: String,

    /// Text to append to the file
    #[clap(name = "Text", index = 2)]
    pub text: String,

    /// Whether to append with a newline
    #[clap(name = "No Newline", short = 'n', long = "no-newline")]
    pub no_newline: bool,

    /// Whether to print debug messages
    #[clap(name = "Verbose", short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Whether to print anything to stdout
    #[clap(name = "Quiet", short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Whether to create the file if it doesn't already exist
    #[clap(name = "Don't Create", short = 'c', long = "no-create")]
    pub no_create: bool,
}
