use anyhow;
use std::{env, fs, path::Path, process};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

pub fn handle<T: AsRef<Path>>(path: T) -> anyhow::Result<()> {
    let mut full_path = env::current_dir()?;
    full_path.push(path);
    let contents = fs::read_to_string(&full_path)?;
    let binding = full_path.extension().unwrap().to_string_lossy();
    let ext = binding.as_ref();
    output(contents, ext)?;
    Ok(())
}

fn output(contents: String, ext: &str) -> anyhow::Result<()> {
    let ps = SyntaxSet::load_defaults_newlines();

    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(ext).unwrap_or_else(|| {
        eprintln!("Error: Filetype not supported");
        process::exit(1)
    });

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(&contents) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    Ok(())
}
