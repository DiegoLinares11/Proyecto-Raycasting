mod maze;
mod color;
mod framebuffer;
mod player;
mod raycasting;
mod texture;

use color::Color;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycasting::Raycasting;
use texture::Texture;

fn main() {
    let width: usize = 8;
    let height: usize = 4;
    let block_size: usize = 20;

    let mut maze = maze::Maze::new(width, height);
    maze.generate();

    let framebuffer_width = (width * 3 + 1) * block_size;
    let framebuffer_height = (height * 2 + 1) * block_size;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::new(255, 255, 255));

    maze::render_framebuffer(&mut framebuffer, &maze);

    let start_pos = maze.get_start_position();

    let mut player = Player::new(
        (start_pos.1 as f32 * 3.0 + 1.0) * block_size as f32, 
        (start_pos.0 as f32 * 2.0 + 1.0) * block_size as f32, 
        std::f32::consts::PI / 3.0,
        std::f32::consts::PI / 3.0,
    );

    let mut mode = "2D";  // Modo inicial

    // Cargar la textura para las paredes
    let wall_texture = Texture::new("assets/lavaImage.png");

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        player.process_events(&window, &maze.render(), block_size);
    
        // Cambiar de modo si se presiona la tecla M
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }
    
        // Limpiar el framebuffer
        framebuffer.clear();
    
        // Dibujar según el modo actual
        if mode == "2D" {
            Raycasting::render2d(&mut framebuffer, &player, &maze, block_size);
        } else {
            Raycasting::render3d(&mut framebuffer, &player, &maze, block_size, &wall_texture);
        }

        // Renderizar el minimapa en la esquina superior izquierda
        Raycasting::render_minimap(&mut framebuffer, &player, &maze, block_size);
    
        // Actualizar la ventana con el contenido del framebuffer
        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();
    
        window.update_with_buffer(&buffer, framebuffer_width, framebuffer_height).unwrap();
    }
}