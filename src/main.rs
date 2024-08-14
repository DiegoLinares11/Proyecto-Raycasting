mod maze;
mod color;
mod framebuffer;
mod player;
mod raycasting;

use color::Color;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycasting::cast_ray;

fn main() {
    let width: usize = 12;
    let height: usize = 8;
    let block_size: usize = 15;

    let mut maze = maze::Maze::new(width, height);
    maze.generate();

    let framebuffer_width = (width * 3 + 1) * block_size;
    let framebuffer_height = (height * 2 + 1) * block_size;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::new(255, 255, 255));

    maze::render_framebuffer(&mut framebuffer, &maze);

    let mut player = Player::new(3.0 * block_size as f32, 3.0 * block_size as f32, std::f32::consts::PI / 3.0);

    cast_ray(&mut framebuffer, &maze.render(), &player, block_size);

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
        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();

        window.update_with_buffer(&buffer, framebuffer_width, framebuffer_height).unwrap();
    }
}
