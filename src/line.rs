use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub trait Line {
    fn line(&mut self, start: Vertex, end: Vertex);
    fn draw_polygon(&mut self, points: &[Vertex]);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vertex, end: Vertex) {
        let Vertex { position: start_pos } = start;
        let Vertex { position: end_pos } = end;

        let (mut x, mut y) = (start_pos.x.round() as isize, start_pos.y.round() as isize);
        let (x2, y2) = (end_pos.x.round() as isize, end_pos.y.round() as isize);
        let dx = (x2 - x).abs();
        let dy = (y2 - y).abs();
        let sx = if x < x2 { 1 } else { -1 };
        let sy = if y < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            // Verificar si el punto está dentro del framebuffer antes de dibujar
            if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
                self.point(x, y);
            }

            if x == x2 && y == y2 {
                break;
            }

            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, points: &[Vertex]) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0] // Cerrar el polígono
            } else {
                points[i + 1]
            };
            self.line(start, end);
        }
    }
}
