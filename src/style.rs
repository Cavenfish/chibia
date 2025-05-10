use core::fmt;

pub trait TibiaStyle {
    fn tibia(self) -> impl fmt::Display;
}

impl TibiaStyle for f64 {
    fn tibia(self) -> impl fmt::Display {
        if self.abs() > 1e6 {
            format!("{:.1}kk", self / 1e6)
        } else if self.abs() > 1e3 {
            format!("{:.1}k", self / 1e3)
        } else {
            format!("{self}")
        }
    }
}
