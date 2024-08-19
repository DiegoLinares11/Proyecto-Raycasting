use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze;
use crate::color::Color;
use crate::maze::Maze;
use crate::texture::Texture;
use minifb::MouseMode;

pub struct Raycasting;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

impl Raycasting {
    pub fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        framebuffer.clear();

        maze::render_framebuffer(framebuffer, maze);

        let num_rays = 5; // Ajusta este valor según lo que necesites
        for i in 0..num_rays {
            let current_ray = i as f32 / num_rays as f32; // Rayo actual dividido por el total de rayos
            let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
            Raycasting::cast_ray(framebuffer, maze, player, a, block_size, true);
        }

        framebuffer.set_current_color(Color::new(255, 0, 0)); // Color rojo para el jugador
        framebuffer.point(player.pos.x as isize, player.pos.y as isize);
    }

    pub fn render3d(
        framebuffer: &mut Framebuffer,
        player: &Player,
        maze: &Maze,
        block_size: usize,
        wall_texture: &Texture,
        floor_texture: &Texture,
        ceiling_texture: &Texture, // Nueva textura para el cielo
    ) {
        let hw = framebuffer.get_height() as f32 / 2.0; // Altura media del viewport
        let num_rays = framebuffer.get_width(); // Lanza un rayo por cada columna en el framebuffer
    
        // Campo de visión más común
        let fov = std::f32::consts::PI / 3.0; // 60 grados
        let distance_to_projection_plane = (framebuffer.get_width() as f32 / 2.0) / (fov / 2.0).tan();
    
        for i in 0..num_rays {
            let current_ray = i as f32 / num_rays as f32;
            let a = player.a - (fov / 2.0) + (fov * current_ray);
    
            let intersect = Raycasting::cast_ray(framebuffer, maze, player, a, block_size, false);
            let distance_to_wall = intersect.distance;
    
            // Corregir el efecto de "fish-eye" dividiendo la distancia por el coseno del ángulo del rayo
            let corrected_distance_to_wall = distance_to_wall * (player.a - a).cos();
    
            // Calcular la altura de la stake
            let stake_height = (block_size as f32 * distance_to_projection_plane) / corrected_distance_to_wall;
            let stake_top = (hw - (stake_height / 2.0)).max(0.0) as usize;
            let stake_bottom = (hw + (stake_height / 2.0)).min(framebuffer.get_height() as f32) as usize;
    
            // Renderizar el cielo (por encima de la pared)
            for y in 0..stake_top {
                let texture_x = (i as f32 / num_rays as f32 * ceiling_texture.width() as f32) as u32;
                let texture_y = (y as f32 / stake_top as f32 * ceiling_texture.height() as f32) as u32;
                let color = ceiling_texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(Color::new(color.0, color.1, color.2));
                framebuffer.point(i as isize, y as isize);
            }
    
            // Renderizar la textura de la pared
            let texture_x = (i as f32 / num_rays as f32 * wall_texture.width() as f32) as u32;
            for y in stake_top..stake_bottom {
                let texture_y = ((y as f32 - stake_top as f32) / (stake_bottom - stake_top) as f32 * wall_texture.height() as f32) as u32;
                let color = wall_texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(Color::new(color.0, color.1, color.2));
                framebuffer.point(i as isize, y as isize);
            }
    
            // Renderizar el suelo (por debajo de la pared)
            for y in stake_bottom..framebuffer.get_height() {
                let texture_x = (i as f32 / num_rays as f32 * floor_texture.width() as f32) as u32;
                let texture_y = ((y as f32 - stake_bottom as f32) / (framebuffer.get_height() as f32 - stake_bottom as f32) * floor_texture.height() as f32) as u32;
                let color = floor_texture.get_color(texture_x, texture_y);
                framebuffer.set_current_color(Color::new(color.0, color.1, color.2));
                framebuffer.point(i as isize, y as isize);
            }
        }
    }
    
    
    
    

    pub fn render_minimap(
        framebuffer: &mut Framebuffer,
        player: &Player,
        maze: &Maze,
        block_size: usize
    ) {
        let mini_scale = 4; // Escala del minimapa (ajusta según el tamaño que quieras)
        let offset_x = framebuffer.get_width() as isize - (maze.width * mini_scale) as isize - 10;  // Offset para colocar el minimapa a la derecha
        let offset_y = 10;  // Offset del minimapa en la pantalla
    
        for j in 0..maze.height {
            for i in 0..maze.width {
                let cell = maze.render()[j * 2 + 1].chars().nth(i * 3 + 1).unwrap();
                let color = match cell {
                    ' ' => (200, 200, 200), // Gris para caminos sin textura
                    '+' | '-' | '|' => (255, 255, 0), // Negro para las paredes
                    'p' => (0, 255, 0), // Verde para el inicio
                    'g' => (255, 0, 0), // Rojo para el objetivo
                    _ => (255, 255, 255), // Blanco para cualquier otro
                };
    
                framebuffer.set_current_color(Color::new(color.0, color.1, color.2));
                let mini_x = offset_x + (i * mini_scale) as isize;
                let mini_y = offset_y + (j * mini_scale) as isize;
                for y in mini_y..(mini_y + mini_scale as isize) {
                    for x in mini_x..(mini_x + mini_scale as isize) {
                        framebuffer.point(x, y);
                    }
                }
            }
        }
    
        // Dibujar al jugador en el minimapa
        framebuffer.set_current_color(Color::new(0, 0, 255));
        let player_mini_x = offset_x + ((player.pos.x / (block_size * 3) as f32) * mini_scale as f32) as isize;
        let player_mini_y = offset_y + ((player.pos.y / (block_size * 2) as f32) * mini_scale as f32) as isize;
        framebuffer.point(player_mini_x, player_mini_y);
    }
    
    pub fn cast_ray(
        framebuffer: &mut Framebuffer,
        maze: &Maze,
        player: &Player,
        a: f32,
        block_size: usize,
        draw_line: bool,
    ) -> Intersect {
        let mut d = 0.0;
        framebuffer.set_current_color(Color::new(255, 0, 0));

        loop {
            let cos = d * a.cos();
            let sin = d * a.sin();
            let x = player.pos.x + cos;
            let y = player.pos.y + sin;

            let i = (x / block_size as f32).floor() as usize;
            let j = (y / block_size as f32).floor() as usize;

            if i >= (maze.width * 3 + 1) || j >= (maze.height * 2 + 1) {
                return Intersect {
                    distance: d,
                    impact: ' ',
                };
            }

            let impact_char = maze.render()[j].chars().nth(i).unwrap();

            if impact_char != ' ' {
                return Intersect {
                    distance: d,
                    impact: impact_char,
                };
            }

            if draw_line {
                framebuffer.point(x as isize, y as isize);
            }

            d += 1.0; // Ajusta según sea necesario
        }
    }
}
