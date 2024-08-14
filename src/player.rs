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

    pub fn process_events(&mut self, window: &Window) {
        const MOVE_SPEED: f32 = 10.0;
        const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;

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
            self.pos.x += self.a.cos() * MOVE_SPEED;
            self.pos.y += self.a.sin() * MOVE_SPEED;
        }

        // Mover hacia atrás
        if window.is_key_down(Key::Down) {
            self.pos.x -= self.a.cos() * MOVE_SPEED;
            self.pos.y -= self.a.sin() * MOVE_SPEED;
        }
    }
}
