use data::Grid;
use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::draw_filled_rect_mut;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style {
    pub cell_size: u32,
    pub wall_thickness: u32,
    pub background_color: Rgb<u8>,
    pub wall_color: Rgb<u8>,
}

pub struct StyleBuilder {
    pub cell_size: u32,
    pub wall_thickness: u32,
    pub background_color: Rgb<u8>,
    pub wall_color: Rgb<u8>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        StyleBuilder {
            cell_size: 30,
            wall_thickness: 5,
            background_color: Rgb([255, 255, 255]),
            wall_color: Rgb([0, 0, 0]),
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

    pub fn build(&self) -> Style {
        Style {
            cell_size: self.cell_size,
            wall_thickness: self.wall_thickness,
            background_color: self.background_color,
            wall_color: self.wall_color,
        }
    }
}

pub fn png(grid: &Grid, style: &Style, name: &str) {
    let width = grid.width as u32 * style.cell_size + (grid.width as u32 + 1) * style.wall_thickness;
    let height = grid.height as u32 * style.cell_size + (grid.height as u32 + 1) * style.wall_thickness;

    let mut img = RgbImage::new(width, height);

    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(width, height), style.background_color);
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(width, style.wall_thickness), style.wall_color);
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(style.wall_thickness, height), style.wall_color);

    for col in 0..grid.width {
        for row in 0..grid.height {
            let cell = grid.get(row, col).unwrap(); // this is technically safe
            if let Some(ref east) = cell.east {
                if !cell.is_linked_pos(east) {
                    let x = (col + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
                    let y = row as i32 * (style.cell_size + style.wall_thickness) as i32;
                    let w = style.wall_thickness;
                    let h = style.cell_size + 2 * style.wall_thickness;
                    draw_filled_rect_mut(&mut img, Rect::at(x, y).of_size(w, h), style.wall_color);
                }
            }

            if let Some(ref south) = cell.south {
                if !cell.is_linked_pos(south) {
                    let x = col as i32 * (style.cell_size + style.wall_thickness) as i32;
                    let y = (row + 1) as i32 * (style.cell_size + style.wall_thickness) as i32;
                    let w = style.cell_size + 2 * style.wall_thickness;
                    let h = style.wall_thickness;
                    draw_filled_rect_mut(&mut img, Rect::at(x, y).of_size(w, h), style.wall_color);
                }
            }
        }
    }

    img.save(name).unwrap()
}

#[cfg(test)]
mod test_style {
    use super::*;

    #[test]
    fn bulding() {
        let a = StyleBuilder::new().build();

        let expected = Style {
            cell_size: 30,
            wall_thickness: 5,
            background_color: Rgb([255, 255, 255]),
            wall_color: Rgb([0, 0, 0]),
        };

        assert_eq!(a, expected);

        let a = StyleBuilder::new()
            .cell_size(10)
            .wall_thickness(2)
            .background_color(&[2, 2, 2])
            .wall_color(&[4, 4, 4])
            .build();

        let expected = Style {
            cell_size: 10,
            wall_thickness: 2,
            background_color: Rgb([2, 2, 2]),
            wall_color: Rgb([4, 4, 4]),
        };

        assert_eq!(a, expected);
    }
}
