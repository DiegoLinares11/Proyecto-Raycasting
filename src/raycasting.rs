use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::player::Player;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[String],
    player: &Player,
    block_size: usize,
) {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::new(255, 0, 0));

    loop {
        let cos = d * player.a.cos();
        let sin = d * player.a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if i >= maze[0].len() || j >= maze.len() {
            return;
        }

        if maze[j].chars().nth(i).unwrap() != ' ' {
            return;
        }

        framebuffer.point(x as isize, y as isize);
        d += 0.5;
    }
}
