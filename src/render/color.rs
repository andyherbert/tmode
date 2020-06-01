pub struct Color {
    pub rgb: [u8; 3],
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            rgb: [red, green, blue],
        }
    }
}
