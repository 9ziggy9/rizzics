use std::fmt;

pub struct Matrix3x3 {
    elements: [f64; 9]
}

impl fmt::Display for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self.elements
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, &val)| {
                format!(
                    "{}{:>8.3}{}", acc, val,
                    if (i + 1) % 3 == 0 { "\n" } else { " " }
                )
            })
            .trim_end()
            .to_string();
        write!(f, "{}", result)
    }
}

impl Matrix3x3 {
    pub fn new(
      x00: f64, x01: f64, x02: f64,
      x10: f64, x11: f64, x12: f64,
      x20: f64, x21: f64, x22: f64,
    ) -> Self { Matrix3x3 {
        elements: [ x00, x01, x02,
                    x10, x11, x12,
                    x20, x21, x22 ] }}
}

pub struct Vector3 {x: f64, y: f64, z:f64}
