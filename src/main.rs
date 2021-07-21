use calc::NewFigure;
use geometry::Figure;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let figure = Figure::new(&args).unwrap_or_else(|err| {
        eprintln!("Error during parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = calc::run(figure) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
