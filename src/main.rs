use std::io::prelude::*;
mod strsep;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let sep = &args[1];
    let blockn: usize = args[2].parse()?;
    let stdin = std::io::stdin();
    let mut input = std::io::BufReader::new(stdin.lock());
    let mut src = String::new();
    loop {
        let mut line = String::new();
        let bytes = input.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        src.push_str(&line);
        line.clear();
    }
    if let Some(block) = strsep::blockcutr(&src, sep, blockn) {
        let out = std::io::stdout();
        let mut out = std::io::BufWriter::new(out.lock());
        write!(out, "{}", block)?;
        out.flush()?;
        Ok(())
    } else {
        Err(From::from("Nothing to cut."))
    }
}
