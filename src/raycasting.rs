use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze;
use crate::color::Color;
use crate::maze::Maze;
use crate::texture::Texture;  

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

    pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        let hw = framebuffer.get_height() as f32 / 2.0; // Altura media del viewport
        let num_rays = framebuffer.get_width(); // Lanza un rayo por cada columna en el framebuffer
    
        framebuffer.set_current_color(Color::new(255, 255, 255)); // Asignar color blanco
    
        for i in 0..num_rays {
            // Calcular el ángulo del rayo actual
            let current_ray = i as f32 / num_rays as f32;
            let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    
            // Lanza el rayo y obtén la intersección
            let intersect = Raycasting::cast_ray(framebuffer, maze, player, a, block_size, false);
    
            // Calcular la distancia a la pared
            let distance_to_wall = intersect.distance;
    
            // Calcula la altura de la stake
            let distance_to_projection_plane = 1.0; // Puedes ajustar esto para modificar la perspectiva
            let stake_height = (hw / distance_to_wall) * distance_to_projection_plane;
    
            // Calcular la posición para dibujar la stake
            let stake_top = (hw - (stake_height / 2.0)).max(0.0) as usize;
            let stake_bottom = (hw + (stake_height / 2.0)).min(framebuffer.get_height() as f32) as usize;
    
            // Dibujar la stake en el framebuffer
            for y in stake_top..stake_bottom {
                framebuffer.point(i as isize, y as isize);
            }
        }
    }
    
    

    pub fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        let mini_scale = 4; // Escala del minimapa (ajusta según el tamaño que quieras)
        let offset_x = 10;  // Offset del minimapa en la pantalla
        let offset_y = 10;
    
        // Dibujar el laberinto en el minimapa
        let rendered_maze = maze.render(); // Renderiza el laberinto
        for j in 0..maze.height {
            for i in 0..maze.width {
                // Obtén los caracteres que representan las paredes y los caminos
                let cell = rendered_maze[j * 2 + 1].chars().nth(i * 3 + 1).unwrap();
                let color = match cell {
                    ' ' => Color::new(200, 200, 200), // Gris para caminos
                    '+' | '-' | '|' => Color::new(0, 0, 0), // Negro para paredes
                    'p' => Color::new(0, 255, 0), // Verde para el inicio
                    'g' => Color::new(255, 0, 0), // Rojo para el objetivo
                    _ => Color::new(255, 255, 255), // Blanco para cualquier otro
                };
                
    
                framebuffer.set_current_color(color);
                let mini_x = offset_x + i * mini_scale;
                let mini_y = offset_y + j * mini_scale;
                for y in mini_y..(mini_y + mini_scale) {
                    for x in mini_x..(mini_x + mini_scale) {
                        framebuffer.point(x as isize, y as isize);
                    }
                }
            }
        }
    
        // Dibujar al jugador en el minimapa
        framebuffer.set_current_color(Color::new(0, 0, 255));
    
        let player_mini_x = offset_x as isize + ((player.pos.x / (block_size * 3) as f32) * mini_scale as f32) as isize;
        let player_mini_y = offset_y as isize + ((player.pos.y / (block_size * 2) as f32) * mini_scale as f32) as isize;
    
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
