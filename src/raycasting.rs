use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze;
use crate::color::Color;
use crate::maze::Maze;

pub struct Raycasting;

impl Raycasting {
    pub fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        // Limpia el framebuffer antes de dibujar
        framebuffer.clear();

        // Dibuja el laberinto
        maze::render_framebuffer(framebuffer, maze);

        // Dibuja los rayos en el modo 2D
        let num_rays = 5; // Ajusta este valor según lo que necesites
        for i in 0..num_rays {
            let current_ray = i as f32 / num_rays as f32; // Rayo actual dividido por el total de rayos
            let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
            Raycasting::cast_ray(framebuffer, maze, player, a, block_size);
        }

        // Dibuja al jugador en la posición actual
        framebuffer.set_current_color(Color::new(255, 0, 0)); // Color rojo para el jugador
        framebuffer.point(player.pos.x as isize, player.pos.y as isize);
    }

    pub fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
        // Implementación del renderizado en 3D
        // Aquí implementarás la lógica para dibujar el entorno 3D según los rayos que se lanzan
    }

    fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, a: f32, block_size: usize) {
        let mut d = 0.0;
        framebuffer.set_current_color(Color::new(255, 0, 0));

        loop {
            let cos = d * a.cos();
            let sin = d * a.sin();
            let x = (player.pos.x + cos) as usize;
            let y = (player.pos.y + sin) as usize;

            let i = x / block_size;
            let j = y / block_size;

            if i >= maze.width || j >= maze.height {
                return;
            }

            if maze.render()[j].chars().nth(i).unwrap() != ' ' {
                return;
            }

            framebuffer.point(x as isize, y as isize);
            d += 0.5; // Ajusta según sea necesario
        }
    }
}
