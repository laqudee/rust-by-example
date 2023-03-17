use std::fmt::{self, Display, Formatter};

pub struct City {
    pub name: &'static str,
    pub lat: f32,
    pub lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}度{} {:.3}度{}分",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)?;
        write!(f, " 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        // if self.red < 10 {
        //     write!(f, " 0x{:02}", self.red)?;
        // } else {
        //     write!(f, " 0x{:X}", self.red)?;
        // }

        // if self.green < 10 {
        //     write!(f, "{:02}", self.green)?;
        // } else {
        //     write!(f, "{:X}", self.green)?;
        // }

        // if self.blue < 10 {
        //     write!(f, "{:02}", self.blue)
        // } else {
        //     write!(f, "{:X}", self.blue)
        // }
    }
}
