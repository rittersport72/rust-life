use graphics::{clear, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

// Window dimensions
pub const CELL_SIZE: u32 = 15;
pub const GRID_X_COUNT: u32 = 25;
pub const GRID_Y_COUNT: u32 = 20;

const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // black

// Cell
const ALIVE_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 1.0]; // magenta
const DEAD_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0]; // grey
const MOUSE_COLOR: [f32; 4] = [0.0, 1.0, 1.0, 1.0]; // blue

pub struct Application {
    gl: GlGraphics,
    grid: [[bool; GRID_X_COUNT as usize]; GRID_Y_COUNT as usize],
    timer: f64,
    mouse_posit: Option<[usize; 2]>, // as grid cell
    mouse_pressed: bool,
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let mut app = Application {
            gl: GlGraphics::new(opengl),
            grid: [[false; GRID_X_COUNT as usize]; GRID_Y_COUNT as usize],
            timer: 0.0,
            mouse_posit: None,
            mouse_pressed: false,
        };
        // app.grid[7][13] = true;
        // app.grid[7][14] = true;
        // app.grid[7][15] = true;
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);

            // Iterate grid
            for y in 0..GRID_Y_COUNT as usize {
                for x in 0..GRID_X_COUNT as usize {
                    // Select color
                    let mut color = DEAD_COLOR;
                    if self.grid[y][x] {
                        color = ALIVE_COLOR;
                    }
                    // Draw cell
                    let rect = [
                        (x as u32 * CELL_SIZE) as f64,
                        (y as u32 * CELL_SIZE) as f64,
                        (CELL_SIZE - 1) as f64,
                        (CELL_SIZE - 1) as f64,
                    ];
                    rectangle(color, rect, c.transform, gl);
                }
            }

            // Draw mouse position
            if self.mouse_posit.is_some() {
                let position = self.mouse_posit.unwrap();
                let rect = [
                    (position[0] as u32 * CELL_SIZE) as f64,
                    (position[1] as u32 * CELL_SIZE) as f64,
                    (CELL_SIZE - 1) as f64,
                    (CELL_SIZE - 1) as f64,
                ];
                rectangle(MOUSE_COLOR, rect, c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer = self.timer + args.dt;
        if self.timer >= 1.0 {
            //println!("timer hit {}", self.timer);
            self.timer = 0.0;

            self.check_neighbours();
        }
    }

    pub fn mouse_pressed(&mut self) {
        self.mouse_pressed = true;

        if self.mouse_posit.is_some() {
            let posit = self.mouse_posit.unwrap();
            self.grid[posit[1]][posit[0]] = true;
        }

        println!("mouse pressed");
    }

    pub fn mouse_released(&mut self) {
        self.mouse_pressed = false;
        self.mouse_posit = None;

        println!("mouse released");
    }

    pub fn mouse_moved(&mut self, position: [f64; 2]) {
        let mut x = (position[0] / CELL_SIZE as f64).floor() as u32;
        let mut y = (position[1] / CELL_SIZE as f64).floor() as u32;

        // Check bounds
        if x >= GRID_X_COUNT {
            x = GRID_X_COUNT - 1;
        }
        if y >= GRID_Y_COUNT {
            y = GRID_Y_COUNT - 1;
        }

        self.mouse_posit = Some([x as usize, y as usize]);
        println!("mouse moved x{} y{}", x, y);

        if self.mouse_pressed {
            let posit = self.mouse_posit.unwrap();
            self.grid[posit[1]][posit[0]] = true;
        }
    }

    fn check_neighbours(&mut self) {
        // nw | n | ne
        // -----------
        //  w | c | e
        // -----------
        // sw | s | se
        //
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

        // Copy grid
        let mut new_grid = self.grid.clone();

        // Iterate grid
        for y in 0..GRID_Y_COUNT as usize {
            for x in 0..GRID_X_COUNT as usize {
                let mut neighbours = 0u8;

                if x > 0 {
                    let w = self.grid[y][x - 1];
                    if w {
                        neighbours += 1;
                    }
                    if y > 0 {
                        let nw = self.grid[y - 1][x - 1];
                        if nw {
                            neighbours += 1;
                        }
                    }
                }
                if y > 0 {
                    let n = self.grid[y - 1][x];
                    if n {
                        neighbours += 1;
                    }
                    if x < (GRID_X_COUNT - 1) as usize {
                        let ne = self.grid[y - 1][x + 1];
                        if ne {
                            neighbours += 1;
                        }
                    }
                }
                if x < (GRID_X_COUNT - 1) as usize {
                    let e = self.grid[y][x + 1];
                    if e {
                        neighbours += 1;
                    }
                    if y < (GRID_Y_COUNT - 1) as usize {
                        let se = self.grid[y + 1][x + 1];
                        if se {
                            neighbours += 1;
                        }
                    }
                }
                if y < (GRID_Y_COUNT - 1) as usize {
                    let s = self.grid[y + 1][x];
                    if s {
                        neighbours += 1;
                    }
                    if x > 0 {
                        let sw = self.grid[y + 1][x - 1];
                        if sw {
                            neighbours += 1;
                        }
                    }
                }

                // Update cell in new grid
                if self.grid[y][x] {
                    // Cell alive
                    if neighbours < 2 {
                        new_grid[y][x] = false;
                    } else if neighbours > 3 {
                        new_grid[y][x] = false;
                    }
                } else {
                    // Cell dead
                    if neighbours == 3 {
                        new_grid[y][x] = true;
                    }
                }
            }
        }

        // Update grid
        self.grid = new_grid;
    }
}
