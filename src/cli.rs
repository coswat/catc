use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Filename
    #[arg(value_name("FILE_NAME"))]
    pub name: String,
}
