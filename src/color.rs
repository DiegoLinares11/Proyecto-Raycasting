use std::ops::{Add, Mul};  

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    // Constructor para crear un nuevo color con valores RGB
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    // Constructor para crear un nuevo color desde un valor hexadecimal
    pub fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xFF) as u8,
            green: ((hex >> 8) & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        }
    }


    // Método para obtener el valor hexadecimal del color
    pub fn to_hex(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    // Métodos para obtener los valores de los canales
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    // Métodos para establecer los valores de los canales
    pub fn set_red(&mut self, red: u8) {
        self.red = red;
    }

    pub fn set_green(&mut self, green: u8) {
        self.green = green;
    }

    pub fn set_blue(&mut self, blue: u8) {
        self.blue = blue;
    }
}

impl Add for Color{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self{
            red: self.red.saturating_add(other.red),
            green: self.green.saturating_add(other.green),
            blue: self.blue.saturating_add(other.blue),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            red: (self.red as f32 * factor).min(255.0).max(0.0) as u8,
            green: (self.green as f32 * factor).min(255.0).max(0.0) as u8,
            blue: (self.blue as f32 * factor).min(255.0).max(0.0) as u8,
        }
    }
}

// Pruebas unitarias
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_by_negative() {
        let color = Color { red: 100, green: 150, blue: 200 };
        let factor = -1.5;

        let multiplied_color = color * factor;

        assert_eq!(multiplied_color.red(), 0);
        assert_eq!(multiplied_color.green(), 0);
        assert_eq!(multiplied_color.blue(), 0);
    }

    #[test]
    fn test_multiply_by_large_number() {
        let color = Color { red: 100, green: 150, blue: 200 };
        let factor = 10.0;

        let multiplied_color = color * factor;

        assert_eq!(multiplied_color.red(), 255);
        assert_eq!(multiplied_color.green(), 255);
        assert_eq!(multiplied_color.blue(), 255);
    }
}
impl Color {
    pub fn to_u32(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}

