pub enum Figure {
    Rectangle { height: f64, width: f64 },
    Circle { radius: f64 },
    Cuboid { height: f64, width: f64, depth: f64 },
    Sphere { radius: f64 },
}

pub mod flat {
    use super::Figure;
    pub fn get_area(figure: &Figure) -> Option<f64> {
        match figure {
            Figure::Rectangle { height, width } => Some(height * width),
            Figure::Circle { radius } => Some(radius.powi(2) * std::f64::consts::PI),
            _ => None,
        }
    }
}

pub mod volume {
    use super::Figure;
    pub fn get_volume(figure: &Figure) -> Option<f64> {
        match figure {
            Figure::Cuboid {
                height,
                width,
                depth,
            } => Some(height * width * depth),
            Figure::Sphere { radius } => Some((4.0 / 3.0) * std::f64::consts::PI * radius.powi(3)),
            _ => None,
        }
    }
}
