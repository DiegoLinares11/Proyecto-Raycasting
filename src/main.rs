mod maze;
mod color;
mod framebuffer;
mod player;
mod raycasting;
mod texture;
mod text_renderer;

extern crate rusttype;

use color::Color;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use player::Player;
use raycasting::Raycasting;
use texture::Texture;
use text_renderer::TextRenderer;

fn main() {
    let width: usize = 6;
    let height: usize = 4;
    let block_size: usize = 5;

    let mut maze = maze::Maze::new(width, height);
    maze.generate();

    let framebuffer_width = ((width * 3 + 1) * block_size) * 3;
    let framebuffer_height = ((height * 2 + 1) * block_size) * 3;

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

    // Cargar las texturas
    let wall_texture = Texture::new("assets/lavaImage.png");
    let floor_texture = Texture::new("assets/sueloTest.png");
    let ceiling_texture = Texture::new("assets/cieloTest.png");



    // Inicializar el renderizador de texto
    let text_renderer = TextRenderer::new("assets/fuente.ttf");

    let mut window = Window::new(
        "Rust Graphics - Maze Example",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut last_time = std::time::Instant::now();
    let mut frames = 0;
    let mut fps = 0;

    let mut last_mouse_x = None;
    // let mut animation_frame_index = 0;  // Índice del cuadro actual

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((mouse_x, _mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if let Some(last_x) = last_mouse_x {
                let delta_x = mouse_x - last_x; // Diferencia de posición en X
                player.a += delta_x * 0.005; // Ajusta la sensibilidad del mouse
            }
            last_mouse_x = Some(mouse_x);
        }
        frames += 1;

        let current_time = std::time::Instant::now();
        let elapsed = current_time.duration_since(last_time);
        if elapsed.as_secs() >= 1 {
            fps = frames / elapsed.as_secs() as u32;
            frames = 0;
            last_time = current_time;
        }

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
            Raycasting::render3d(
                &mut framebuffer, 
                &player, 
                &maze, 
                block_size, 
                &wall_texture, 
                &floor_texture, 
                &ceiling_texture
            );
        }

        // Renderizar el minimapa en la esquina superior derecha
        Raycasting::render_minimap(
            &mut framebuffer, 
            &player, 
            &maze, 
            block_size
        );

        // Renderizar los FPS en la esquina superior izquierda
        text_renderer.render_fps(&mut framebuffer, fps);

        // // Renderizar la animación en el centro de la pantalla
        // animation_frame_index = (animation_frame_index + 1) % animation_frames.len();
        // let animation_frame = &animation_frames[animation_frame_index];
        // // Supongamos que queremos renderizar la animación en el centro de la pantalla
        // let center_x = framebuffer.get_width() / 2;
        // let center_y = framebuffer.get_height() / 2;
        // framebuffer.render_texture(animation_frame, center_x, center_y);
    
        // Actualizar la ventana con el contenido del framebuffer
        let buffer: Vec<u32> = framebuffer
            .get_buffer()
            .iter()
            .map(|color| color.to_u32())
            .collect();
    
        window.update_with_buffer(&buffer, framebuffer_width, framebuffer_height).unwrap();
    }
}
