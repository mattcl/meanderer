use data::cell::{Cell, MazeCell, PolarCell};
use data::grid::{Grid, MazeGrid, PolarGrid};
use data::pos::Position;
use image::{Rgb, RgbImage};
use imageproc::drawing::{
    draw_antialiased_line_segment_mut,
    draw_convex_polygon_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    Point
};
use imageproc::pixelops::interpolate;
use imageproc::rect::Rect;
use std::f32;
use std::f32::consts::PI;

const DEFAULT_CELL_SIZE: u32 = 30;
const DEFAULT_WALL_THICKNESS: u32 = 5;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    pub cell_size: u32,
    pub wall_thickness: u32,
    pub background_color: Rgb<u8>,
    pub wall_color: Rgb<u8>,
    pub color_fn: Option<fn(weight: u32, max_weight: u32) -> Rgb<u8>>,
    pub draw_solution: bool,
    pub solution_color: Rgb<u8>,
}

pub struct StyleBuilder {
    pub cell_size: u32,
    pub wall_thickness: u32,
    pub background_color: Rgb<u8>,
    pub wall_color: Rgb<u8>,
    pub color_fn: Option<fn(weight: u32, max_weight: u32) -> Rgb<u8>>,
    pub draw_solution: bool,
    pub solution_color: Rgb<u8>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        StyleBuilder {
            cell_size: DEFAULT_CELL_SIZE,
            wall_thickness: DEFAULT_WALL_THICKNESS,
            background_color: Rgb([255, 255, 255]),
            wall_color: Rgb([0, 0, 0]),
            color_fn: None,
            draw_solution: false,
            solution_color: Rgb([200, 0, 0]),
        }
    }

    pub fn cell_size(mut self, size: u32) -> Self {
        self.cell_size = size;
        self
    }

    pub fn wall_thickness(mut self, size: u32) -> Self {
        self.wall_thickness = size;
        self
    }

    pub fn background_color(mut self, color: &[u8; 3]) -> Self {
        self.background_color = Rgb(*color);
        self
    }

    pub fn wall_color(mut self, color: &[u8; 3]) -> Self {
        self.wall_color = Rgb(*color);
        self
    }

    pub fn color_fn(mut self, color_fn: fn(weight: u32, max_weight: u32) -> Rgb<u8>) -> Self {
        self.color_fn = Some(color_fn);
        self
    }

    pub fn draw_solution(mut self) -> Self {
        self.draw_solution = true;
        self
    }

    pub fn solution_color(mut self, color: &[u8; 3]) -> Self {
        self.solution_color = Rgb(*color);
        self
    }

    pub fn build(&self) -> Style {
        Style {
            cell_size: self.cell_size,
            wall_thickness: self.wall_thickness,
            background_color: self.background_color,
            wall_color: self.wall_color,
            color_fn: self.color_fn,
            draw_solution: self.draw_solution,
            solution_color: self.solution_color,
        }
    }
}

pub fn default_color_fn(weight: u32, max_weight: u32) -> Rgb<u8> {
    let intensity = (max_weight - weight) as f32 / max_weight as f32;
    let dark = (255.0 * intensity).round() as u8;
    let bright = 128 + (127.0 * intensity).round() as u8;
    Rgb([dark, bright, dark])
}

pub fn png(grid: &Grid, style: &Style, name: &str) {
    let width =
        grid.width as u32 * style.cell_size + (grid.width as u32 + 1) * style.wall_thickness;
    let height =
        grid.height as u32 * style.cell_size + (grid.height as u32 + 1) * style.wall_thickness;
    let max_weight = grid.cells
        .iter()
        .max_by_key(|c| c.weight())
        .unwrap_or(&Cell::new(0, 0))
        .weight();

    let mut img = RgbImage::new(width, height);

    // background
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(width, height),
        style.background_color,
    );

    // top
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(width, style.wall_thickness),
        style.wall_color,
    );

    // left
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(style.wall_thickness, height),
        style.wall_color,
    );

    for col in 0..grid.width {
        for row in 0..grid.height {
            let cur = Position::new(row, col);
            let cell = grid.get(&cur).unwrap(); // this is technically safe

            let x = (col as i32 * style.cell_size as i32)
                + (col + 1) as i32 * style.wall_thickness as i32;
            let y = (row as i32 * style.cell_size as i32)
                + (row + 1) as i32 * style.wall_thickness as i32;
            let w = style.cell_size + style.wall_thickness;
            let h = style.cell_size + style.wall_thickness;

            if style.draw_solution && cell.in_solution() {
                draw_filled_rect_mut(&mut img, Rect::at(x, y).of_size(w, h), style.solution_color);
            } else if let Some(f) = style.color_fn {
                draw_filled_rect_mut(
                    &mut img,
                    Rect::at(x, y).of_size(w, h),
                    f(cell.weight(), max_weight),
                );
            }

            if let Some(ref east) = cell.east {
                _east_wall(&mut img, grid, style, cell, east, max_weight);
            }

            if let Some(ref south) = cell.south {
                _south_wall(&mut img, grid, style, cell, south, max_weight);
            }
        }
    }

    // right
    draw_filled_rect_mut(
        &mut img,
        Rect::at((width - style.wall_thickness) as i32, 0).of_size(style.wall_thickness, height),
        style.wall_color,
    );

    // bot
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, (height - style.wall_thickness) as i32).of_size(width, style.wall_thickness),
        style.wall_color,
    );

    img.save(name).unwrap()
}

fn _east_wall(
    img: &mut RgbImage,
    grid: &Grid,
    style: &Style,
    cell: &Cell,
    east: &Position,
    max_weight: u32,
) {
    if !cell.is_linked_pos(east) {
        let x = (cell.pos.col + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
        let y = cell.pos.row as i32 * (style.cell_size + style.wall_thickness) as i32;
        let w = style.wall_thickness;
        let h = style.cell_size + 2 * style.wall_thickness;

        draw_filled_rect_mut(img, Rect::at(x, y).of_size(w, h), style.wall_color);
    } else if style.draw_solution && cell.in_solution() {
        if let Some(ref east_cell) = grid.get(east) {
            if !east_cell.in_solution() {
                let x = (cell.pos.col + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
                let y = cell.pos.row as i32 * (style.cell_size + style.wall_thickness) as i32
                    + style.wall_thickness as i32;
                let w = style.wall_thickness;
                let h = style.cell_size;

                let color = match style.color_fn {
                    Some(f) => f(cell.weight(), max_weight),
                    None => style.background_color,
                };

                draw_filled_rect_mut(img, Rect::at(x, y).of_size(w, h), color);
            }
        }
    }
}

fn _south_wall(
    img: &mut RgbImage,
    grid: &Grid,
    style: &Style,
    cell: &Cell,
    south: &Position,
    max_weight: u32,
) {
    if !cell.is_linked_pos(south) {
        let x = cell.pos.col as i32 * (style.cell_size + style.wall_thickness) as i32;
        let y = (cell.pos.row + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
        let w = style.cell_size + 2 * style.wall_thickness;
        let h = style.wall_thickness;

        draw_filled_rect_mut(img, Rect::at(x, y).of_size(w, h), style.wall_color);
    } else if style.draw_solution && cell.in_solution() {
        if let Some(ref south_cell) = grid.get(south) {
            if !south_cell.in_solution() {
                let x = cell.pos.col as i32 * (style.cell_size + style.wall_thickness) as i32
                    + style.wall_thickness as i32;
                let y = (cell.pos.row + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
                let w = style.cell_size;
                let h = style.wall_thickness;

                let color = match style.color_fn {
                    Some(f) => f(cell.weight(), max_weight),
                    None => style.background_color,
                };

                draw_filled_rect_mut(img, Rect::at(x, y).of_size(w, h), color);
            }
        }
    }
}

pub fn polar_png(grid: &PolarGrid, style: &Style, name: &str) {
    let offset = 5;
    let size = (grid.rows * 2) as u32 * style.cell_size + offset * 2;
    let center = (size as f32 / 2.0).round() as i32;
    let max_weight = grid.cells
        .iter()
        .max_by_key(|c| c.weight())
        .unwrap_or(&PolarCell::new(0, 0))
        .weight();

    let mut img = RgbImage::new(size, size);

    let mut walls = Vec::new();

    // background
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(size, size),
        style.background_color,
    );

    for cell in &grid.cells {
        let pos = cell.pos();
        let th = 2.0 * PI / grid.column_counts[pos.row] as f32;
        let inner_radius = (pos.row as u32 * style.cell_size) as f32;
        let outer_radius = ((pos.row + 1) as u32 * style.cell_size) as f32;
        let th_ccw = pos.col as f32 * th;
        let th_cw = (pos.col + 1) as f32 * th;

        // counter clockwise inner corner
        let ax = (center as f32 + (inner_radius * th_ccw.cos())) as i32;
        let ay = (center as f32 + (inner_radius * th_ccw.sin())) as i32;

        // counter clockwise outer corner
        let bx = (center as f32 + (outer_radius * th_ccw.cos())) as i32;
        let by = (center as f32 + (outer_radius * th_ccw.sin())) as i32;

        // clockwise inner corner
        let cx = (center as f32 + (inner_radius * th_cw.cos())) as i32;
        let cy = (center as f32 + (inner_radius * th_cw.sin())) as i32;

        // clockwise outer corner
        let dx = (center as f32 + (outer_radius * th_cw.cos())) as i32;
        let dy = (center as f32 + (outer_radius * th_cw.sin())) as i32;

        if style.draw_solution || style.color_fn.is_some() {
            let mut bounds = Vec::new();

            if cell.pos() != &Position::new(0, 0) {
                bounds.push(Point::new(ax, ay));
                bounds.push(Point::new(bx, by));

                if cell.outward.len() > 1 {
                    // midpoint outer corner
                    let th_mid = (pos.col as f32 + 0.5) * th;
                    let mx = (center as f32 + (outer_radius * th_mid.cos())) as i32;
                    let my = (center as f32 + (outer_radius * th_mid.sin())) as i32;

                    bounds.push(Point::new(mx, my));
                }

                bounds.push(Point::new(dx, dy));
                bounds.push(Point::new(cx, cy));

            } else {
                let th = 2.0 * PI / grid.column_counts[pos.row + 1] as f32;
                for i in 0..6 {
                    let th_r = (pos.col + i) as f32 * th;
                    let rx = (center as f32 + (outer_radius * th_r.cos())) as i32;
                    let ry = (center as f32 + (outer_radius * th_r.sin())) as i32;
                    bounds.push(Point::new(rx, ry));
                }
            }

            if cell.in_solution() {
                draw_convex_polygon_mut(&mut img, bounds.as_slice(), style.solution_color);
            } else {
                let color = match style.color_fn {
                    Some(f) => f(cell.weight(), max_weight),
                    None => style.background_color,
                };
                draw_convex_polygon_mut(&mut img, bounds.as_slice(), color);
            }
        }

        if let Some(ref inward) = cell.inward {
            if !cell.is_linked_pos(inward) {
                walls.push(((ax, ay), (cx, cy)));
            }
        }

        if let Some(ref cw) = cell.cw {
            if !cell.is_linked_pos(cw) {
                walls.push(((cx, cy), (dx, dy)));
            }
        }
    }

    for ((ax, ay), (bx, by)) in walls {
        draw_antialiased_line_segment_mut(&mut img, (ax, ay), (bx, by), style.wall_color, interpolate);
    }

    draw_hollow_circle_mut(&mut img, (center, center), ((size - offset * 2) / 2) as i32, style.wall_color);

    img.save(name).unwrap()
}

#[cfg(test)]
mod test_style {
    use super::*;

    fn _test_color_fn(_: u32, _: u32) -> Rgb<u8> {
        Rgb([0, 0, 0])
    }

    #[test]
    fn bulding() {
        let a = StyleBuilder::new().build();

        let expected = Style {
            cell_size: 30,
            wall_thickness: 5,
            background_color: Rgb([255, 255, 255]),
            wall_color: Rgb([0, 0, 0]),
            color_fn: None,
            draw_solution: false,
            solution_color: Rgb([200, 0, 0]),
        };

        assert_eq!(a, expected);

        let a = StyleBuilder::new()
            .cell_size(10)
            .wall_thickness(2)
            .background_color(&[2, 2, 2])
            .wall_color(&[4, 4, 4])
            .color_fn(_test_color_fn)
            .draw_solution()
            .solution_color(&[11, 11, 11])
            .build();

        let expected = Style {
            cell_size: 10,
            wall_thickness: 2,
            background_color: Rgb([2, 2, 2]),
            wall_color: Rgb([4, 4, 4]),
            color_fn: Some(_test_color_fn),
            draw_solution: true,
            solution_color: Rgb([11, 11, 11]),
        };

        assert_eq!(a, expected);
    }
}
