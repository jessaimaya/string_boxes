extern crate nannou;
use to_binary::{BinaryString};
use nannou::{prelude::*, rand::Rng, ui::color::TRANSPARENT};
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug, Clone)]
struct Cell {
    size: f32,
    pos: (f32, f32),
    cells: Option<Vec<Cell>>,
    col: bool,
    letter: Option<char>,
    fill : bool,
}

#[derive(Debug, Clone)]
struct Model {
    size: f32,
    gutter: f32,
    grid: Vec<Cell>,
    seed: String,
}

fn model(_app: &App) -> Model {
    let size = 60.0;
    let gutter = 4.0;
    let seed = "empowerglobant".to_string();
    //let seed = "abcdefghijklmnopqrstuvwxyz".to_string();
    let grid = gen_grid(size, gutter, &seed);

    Model {
        size,
        gutter,
        grid, 
        seed,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let mut draw = app.draw();

    draw_boxes(&mut draw, model);

    draw.background().color(Rgb::new(33u32,33u32,33u32));


    draw.to_frame(app, &frame).unwrap();
}

fn draw_boxes(draw: &mut Draw, model: &Model) {
    let letters: Vec<char> = model.seed.chars().collect();
    let letters_half = letters.len() / 4;
    
    for parent in model.grid.iter() {
        let cha = parent.letter.unwrap();
        let hex = format!("{:x}", cha as i32);
        let byt =  BinaryString::from_hex(hex.to_owned());
        let byt_str = byt.unwrap().to_string();
        let bits: Vec<bool> = byt_str
                                .chars()
                                .map(|b| !(b == '0'))
                                .collect();

        if let Some(cells) = &parent.cells {
            for (ind,cell) in cells.iter().enumerate() {
                cells_iter(cell, draw, &bits, ind, letters_half);
            }
        }
    }

    fn cells_iter(cell: &Cell, draw: &mut Draw, indx: &Vec<bool>, curr: usize, half: usize) {
        if let Some(cell) = &cell.cells {
            for (ind, cel) in cell.iter().enumerate() {
                cells_iter(cel, draw, indx, curr + ind + 1, half);
            }
        } else {

            if indx[curr] {
                let c = if cell.col {
                    // Green Globant
                    Rgb::new(194u8, 214u8, 71u8)
                } else {
                    // Blue Empower
                    Rgb::new(0u8, 65u8, 133u8)
                    //Rgb::new(219u8, 24u8, 48u8)
                };


            if cell.col {
                if cell.fill {
                     draw.ellipse()
                    .x_y(cell.pos.0, cell.pos.1)
                    .w_h(cell.size, cell.size)
                    .stroke(BLACK)
                    .stroke_weight(2.0)
                    .color(c);

                } else {
                    draw.ellipse()
                    .x_y(cell.pos.0, cell.pos.1)
                    .w_h(cell.size, cell.size)
                    .no_fill()
                    .stroke(c)
                    .stroke_weight(2.0);
                }

            } else {
                if cell.fill {
                    draw.rect()
                    .x_y(cell.pos.0, cell.pos.1)
                    .w_h(cell.size, cell.size)
                    .stroke(BLACK)
                    .stroke_weight(1.0)
                    .color(c);
                } else {
                    draw.rect()
                    .x_y(cell.pos.0, cell.pos.1)
                    .w_h(cell.size, cell.size)
                    .no_fill()
                    .stroke(c)
                    .stroke_weight(1.0);
                }
            }
            }

        }
    }
}

fn gen_grid(size: f32, gutter: f32, seed: &str) -> Vec<Cell> {
    let cols = 2;
    let chars:Vec<char>= seed.chars().collect();
    let rows = chars.len() - 1;

    let mut rng: Pcg64 = Seeder::from(seed).make_rng();

    let mut base_grid = vec![];
    for i in 0..cols {
        for j in 0..rows {
            let coords = get_coors(i, j as u32, size, gutter);

            let mut parent = Cell {
                size,
                pos: coords,
                cells: None,
                col: i % 2 == 0,
                fill: rng.gen_bool(0.5),
                letter: Some(chars[(i + j as u32) as usize]),
            };

            parent.gen_cells();

            let p_cells = parent.cells.as_mut().unwrap();
            let r = rng.gen_range(0..4);

            p_cells[r.clone() as usize].gen_cells();

            base_grid.push(parent);
        }
    }


    fn get_coors(col: u32, row: u32, size: f32, gutter: f32) -> (f32, f32) {
        let center = size;
        let c = (col + 1) as f32;
        let r = (row + 1) as f32;

        let pos_x = (gutter * c) + (size * c);
        let pos_y = (gutter * (row + 1) as f32) + (center * (row + 1) as f32);

        // TODO: dynamic centering with len of rows and cols
        (pos_x - (size * 2.0), (pos_y - (size * 7.5)))

    }
     

    base_grid
}

impl Cell {
    pub fn gen_cells(&mut self) {
        self.cells = self.gen_boxes();
    }

    pub fn gen_boxes (&mut self) -> Option<Vec<Cell>> {
        let mut rng: Pcg64 = Seeder::from(self.letter).make_rng();

        let half = self.size / 4.0;
        let base = Cell {
            size: (self.size / 2.0) - 2.0,
            pos: (0.0, 0.0),
            cells: None,
            col: self.col,
            fill: rng.gen_bool(0.5),
            letter: None,
        };
        let px = self.pos.0;
        let py = self.pos.1;

        Some(vec![
            Cell{pos: ((px - half), (py + half)), ..base.clone()},
            Cell{pos: ((px - half), (py - half)), ..base.clone()},
            Cell{pos: ((px + half), (py + half)), ..base.clone()},
            Cell{pos: ((px + half), (py - half)), ..base},
        ]) 
    }
}
