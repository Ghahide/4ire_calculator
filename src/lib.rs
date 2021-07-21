use geometry::{flat, volume, Figure};
use std::error::Error;

pub trait NewFigure {
    fn new(args: &[String]) -> Result<Figure, &str>;
}

impl NewFigure for Figure {
    fn new(args: &[String]) -> Result<Figure, &str> {
        if args.len() < 2 {
            return Err("not enought parameters");
        }

        for arg in args[2..].iter() {
            if !arg.parse::<f64>().is_ok() || arg.parse::<f64>().unwrap() < 0.0 {
                return Err("wrong numeric parameters");
            }
        }

        let figure: Figure = match args[1].to_lowercase().as_str() {
            "rectangle" => {
                if args.len() != 4 {
                    return Err("rectangle should have 2 parameters");
                }
                let height = args[2].parse().unwrap();
                let width = args[3].parse().unwrap();
                Figure::Rectangle { height, width }
            }
            "circle" => {
                if args.len() != 3 {
                    return Err("circle should have 1 parameter");
                }
                let radius = args[2].parse().unwrap();
                Figure::Circle { radius }
            }
            "parallelepiped" => {
                if args.len() != 5 {
                    return Err("parallelepiped should have 3 parameters");
                }
                let height = args[2].parse().unwrap();
                let width = args[3].parse().unwrap();
                let depth = args[4].parse().unwrap();
                Figure::Cuboid {
                    height,
                    width,
                    depth,
                }
            }
            "sphere" => {
                if args.len() != 3 {
                    return Err("sphere should have 1 parameter");
                }
                let radius = args[2].parse().unwrap();
                Figure::Sphere { radius }
            }
            _ => {
                return Err("wrong figure type");
            }
        };
        Ok(figure)
    }
}

pub fn run(figure: Figure) -> Result<(), Box<dyn Error>> {
    let result = match figure {
        Figure::Rectangle { .. } | Figure::Circle { .. } => flat::get_area(&figure),
        Figure::Cuboid { .. } | Figure::Sphere { .. } => volume::get_volume(&figure),
    };

    match result {
        Some(x) => println!("Result: {}", x),
        None => return Err("undefined operation".into()),
    };

    Ok(())
}
