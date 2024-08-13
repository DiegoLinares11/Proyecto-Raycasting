use rand::seq::SliceRandom;
use rand::Rng;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    vis: Vec<Vec<bool>>,
    hor: Vec<Vec<bool>>,
    ver: Vec<Vec<bool>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Maze {
    pub fn new(w: usize, h: usize) -> Self {
        Maze {
            width: w,
            height: h,
            vis: vec![vec![false; w]; h],
            hor: vec![vec![true; w]; h + 1],
            ver: vec![vec![true; w + 1]; h],
            start: (0, 0),
            end: (h - 1, w - 1),
        }
    }

    pub fn walk(&mut self, x: usize, y: usize) {
        self.vis[y][x] = true;

        let mut directions = vec![
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
        ];
        directions.shuffle(&mut rand::thread_rng());

        for &(xx, yy) in &directions {
            if yy >= self.height || xx >= self.width || self.vis[yy][xx] {
                continue;
            }

            if xx == x {
                self.hor[y.max(yy)][x] = false;
            } else if yy == y {
                self.ver[y][x.max(xx)] = false;
            }
            self.walk(xx, yy);
        }
    }

    pub fn generate(&mut self) {
        let start_x = rand::thread_rng().gen_range(0..self.width);
        let start_y = rand::thread_rng().gen_range(0..self.height);
        self.start = (start_y, start_x);
        self.walk(start_x, start_y);
        self.end = (self.height - 1, self.width - 1);
    }

    pub fn render(&self) -> Vec<String> {
        let mut maze = Vec::new();

        for y in 0..self.height {
            let mut line_hor = String::new();
            for x in 0..self.width {
                line_hor.push('+');
                line_hor.push(if self.hor[y][x] { '-' } else { ' ' });
                line_hor.push(if self.hor[y][x] { '-' } else { ' ' });
            }
            line_hor.push('+');
            maze.push(line_hor);

            let mut line_ver = String::new();
            for x in 0..self.width {
                line_ver.push(if self.ver[y][x] { '|' } else { ' ' });
                line_ver.push(' ');
                line_ver.push(' ');
            }
            line_ver.push('|');
            maze.push(line_ver);
        }

        let mut last_line = String::new();
        for _x in 0..self.width {
            last_line.push('+');
            last_line.push('-');
            last_line.push('-');
        }
        last_line.push('+');
        maze.push(last_line);

        // Colocar las letras 'p' y 'g' en las posiciones correspondientes
        if let Some(start_line) = maze.get_mut(self.start.0 * 2 + 1) {
            start_line.replace_range(self.start.1 * 3 + 1..self.start.1 * 3 + 2, "p");
        }
        if let Some(end_line) = maze.get_mut(self.end.0 * 2 + 1) {
            end_line.replace_range(self.end.1 * 3 + 1..self.end.1 * 3 + 2, "g");
        }

        maze
    }
}

pub fn display_maze(maze: Vec<String>) -> String {
    maze.join("\n")
}