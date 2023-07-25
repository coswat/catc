use std::env;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/**
/// Configures and executes a file syntax highlighter.
///
/// This program reads a file and performs syntax highlighting on its contents using the Syntect library.
/// The highlighted contents are then printed to the console.
///
/// Like cat cli tool clone
///
/// Example usage:
/// ```
/// $ cargon run test.html
/// ```
///
/// Author: coswat
/// Date: 2023-07-06
 */

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Create a new Config instance or print an error and exit if invalid arguments are provided
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Read the file contents
    let contents: String = fs::read_to_string(&config.filename).unwrap_or_else(|err: Error| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // Output the highlighted contents
    config.output(&contents);
    process::exit(0);
}

struct Config {
    filename: PathBuf,
    extension: String,
}

impl Config {
    // Create a new Config instance from command line arguments
    fn new(args: &[String]) -> Result<Self, &str> {
        // Check if a file name is provided
        if args.len() < 2 {
            return Err("Missing arg <filename>");
        }

        // Get the current directory path
        let mut filename: PathBuf = env::current_dir().expect("unable to find path");

        // Append the file name to the directory path
        filename.push(&args[1]);

        // Get the file extension or use "txt" if no extension is provided
        let extension: String = match args[1].split('.').last() {
            Some(ext) => ext.to_string(),
            None => "txt".to_string(),
        };

        // Return a new Config instance
        Ok(Self {
            filename,
            extension,
        })
    }

    // Highlight and output the contents
    fn output(&self, contents: &str) {
        // Load default syntaxes
        let ps = SyntaxSet::load_defaults_newlines();

        // Load default themes
        let ts = ThemeSet::load_defaults();

        // Find the syntax for the provided file extension
        let syntax = ps
            .find_syntax_by_extension(&self.extension)
            .unwrap_or_else(|| {
                eprintln!("Error : File type not supported");
                process::exit(1);
            });

        // Create a HighlightLines instance
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        // Highlight and print each line of the contents
        for line in LinesWithEndings::from(contents) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            print!("{}", escaped);
        }
    }
}
