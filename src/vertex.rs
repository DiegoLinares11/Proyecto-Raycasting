use nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: glm::Vec3,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: glm::vec3(x, y, z),
        }
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn z(&self) -> f32 {
        self.position.z
    }
}
