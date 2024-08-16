use nalgebra_glm::Vec2;
use minifb::Window;
use minifb::Key;

pub struct Player {
    pub pos: Vec2,
    pub a: f32, // Angle of view
    pub fov: f32, // Field of view
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32, fov: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            a,
            fov,
        }
    }

    pub fn process_events(&mut self, window: &Window, maze: &[String], block_size: usize) {
        const MOVE_SPEED: f32 = 2.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 30.0;

        let mut new_pos = self.pos; // Copiamos la posición actual del jugador

        // Rotar a la izquierda
        if window.is_key_down(Key::Left) {
            self.a -= ROTATION_SPEED;
        }

        // Rotar a la derecha
        if window.is_key_down(Key::Right) {
            self.a += ROTATION_SPEED;
        }

        // Mover hacia adelante
        if window.is_key_down(Key::Up) {
            new_pos.x += self.a.cos() * MOVE_SPEED;
            new_pos.y += self.a.sin() * MOVE_SPEED;
        }

        // Mover hacia atrás
        if window.is_key_down(Key::Down) {
            new_pos.x -= self.a.cos() * MOVE_SPEED;
            new_pos.y -= self.a.sin() * MOVE_SPEED;
        }

        // Verificar si la nueva posición está dentro de los límites del mapa
        let i = new_pos.x as usize / block_size;
        let j = new_pos.y as usize / block_size;

        if i >= maze[0].len() || j >= maze.len() {
            // No mover al jugador si la nueva posición está fuera de los límites
            return;
        }

        // Verificar si la nueva posición es una pared o un espacio libre
        match maze[j].chars().nth(i).unwrap() {
            ' ' | 'p' | 'g' => {
                // Si la nueva posición es un espacio libre, 'p' o 'g', actualiza la posición del jugador
                self.pos = new_pos;
            }
            _ => {
                // Si es una pared, no actualizar la posición
            }
        }
    }
}
