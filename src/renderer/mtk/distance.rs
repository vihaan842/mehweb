#[derive(Debug, Copy, Clone)]
pub enum Distance {
    Absolute(f64),
    Relative(f64),
    Auto
}

impl Distance {
    pub const ZERO: Distance = Distance::Absolute(0.0);
    pub fn to_absolute(&self, side: f64) -> f64 {
	match self {
	    Distance::Absolute(d) => *d,
	    Distance::Relative(p) => side * p,
	    Distance::Auto => side,
	}
    }
}

// get distance from string
impl From<String> for Distance {
    fn from(s: String) -> Distance {
	if s.ends_with("em") {
	    // TODO: account for dpi
	    Distance::Absolute(s.trim_end_matches("em").trim().parse::<f64>().unwrap()*crate::rules::DEFAULT_FONT_SIZE as f64)
	} else if s.ends_with("%") {
	    Distance::Relative(s.trim_end_matches('%').trim().parse::<f64>().unwrap()/100.)
	} else if s.ends_with("px") {
	    Distance::Absolute(s.trim_end_matches("px").trim().parse::<f64>().unwrap())
	} else if s == "auto" {
	    Distance::Auto
	} else if s == "0" {
	    Distance::Absolute(0.)
	} else {
	    panic!("This unit is not implemented yet!: {}", s);
	}
    }
}

impl From<&str> for Distance {
    fn from(s: &str) -> Distance {
	Distance::from(s.to_string())
    }
}
