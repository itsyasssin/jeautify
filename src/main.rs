use std::io::{self, IsTerminal};

fn beautify(path: String) -> Result<(), Box<dyn std::error::Error>> {
    if !path.ends_with(".js") {
        eprintln!("[\x1b[33m#\x1b[0m] {path:?} is not a Javascript file.");
        return Ok(());
    }

    let input = std::fs::read_to_string(&path)?;
    let (contents, _) = prettify_js::prettyprint(&input);

    std::fs::write(path, contents)?;
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if io::stdin().is_terminal() {
        for path in args {
            beautify(path)?;
        }
        return Ok(());
    }

    // read file names from stdin
    for path in io::stdin().lines() {
        beautify(path?)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("[\x1b[31m!\x1b[0m] {e}");
        std::process::exit(1)
    }
}
