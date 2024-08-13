use std::fs::File;
use std::io::{Write, BufWriter};
use crate::Color;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24; // Usaremos 24 bits por pixel para un archivo BMP de 24 bits

pub trait BmpWritable {
    fn write_bmp_file(&self, file_path: &str) -> std::io::Result<()>;
}

impl BmpWritable for crate::Framebuffer {
    fn write_bmp_file(&self, file_path: &str) -> std::io::Result<()> {
        let file = File::create(file_path)?;
        let mut file = BufWriter::new(file);

        write_bmp_header(&mut file, self.get_width(), self.get_height())?;
        write_pixel_data(&mut file, self.get_buffer(), self.get_width(), self.get_height())?;

        file.flush()?;
        Ok(())
    }
}

fn write_bmp_header(
    file: &mut BufWriter<File>,
    width: usize,
    height: usize
) -> std::io::Result<()> {
    let file_size = (BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8)) as u32;
    let reserved: u32 = 0;
    let pixel_data_offset: u32 = BMP_PIXEL_OFFSET as u32;
    let header_size: u32 = 40;
    let planes: u16 = 1;
    let bits_per_pixel: u16 = BMP_BITS_PER_PIXEL as u16;
    let compression: u32 = 0;
    let image_size: u32 = (width * height * BMP_BITS_PER_PIXEL / 8) as u32;
    let x_pixels_per_meter: u32 = 2835;
    let y_pixels_per_meter: u32 = 2835;
    let colors_used: u32 = 0;
    let important_colors: u32 = 0;

    file.write_all(b"BM")?; // Firma del archivo
    file.write_all(&file_size.to_le_bytes())?; // Tamaño del archivo
    file.write_all(&reserved.to_le_bytes())?; // Reservado
    file.write_all(&pixel_data_offset.to_le_bytes())?; // Offset de datos de píxeles

    file.write_all(&header_size.to_le_bytes())?; // Tamaño del encabezado
    file.write_all(&(width as u32).to_le_bytes())?; // Ancho
    file.write_all(&(height as u32).to_le_bytes())?; // Alto
    file.write_all(&planes.to_le_bytes())?; // Planos de color
    file.write_all(&bits_per_pixel.to_le_bytes())?; // Bits por píxel
    file.write_all(&compression.to_le_bytes())?; // Compresión
    file.write_all(&image_size.to_le_bytes())?; // Tamaño de la imagen
    file.write_all(&x_pixels_per_meter.to_le_bytes())?; // Resolución horizontal
    file.write_all(&y_pixels_per_meter.to_le_bytes())?; // Resolución vertical
    file.write_all(&colors_used.to_le_bytes())?; // Colores usados
    file.write_all(&important_colors.to_le_bytes())?; // Colores importantes

    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>,
    buffer: &[Color],
    width: usize,
    height: usize
) -> std::io::Result<()> {
    let row_padding = (4 - (width * 3) % 4) % 4;

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = &buffer[y * width + x];
            file.write_all(&[pixel.blue(), pixel.green(), pixel.red()])?;
        }
        for _ in 0..row_padding {
            file.write_all(&[0])?;
        }
    }

    Ok(())
}




