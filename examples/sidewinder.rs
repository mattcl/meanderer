extern crate meanderer;
extern crate rand;


use meanderer::data::{Grid, Position};
use rand::Rng;


fn sidewinder(grid: &mut Grid) {
    let mut links = Vec::new();

    for row in 0..grid.height {
        let mut run = Vec::new();
        let mut rng = rand::thread_rng();

        for col in 0..grid.width {
            let cell = grid.get(row, col).unwrap(); // this is safe
            run.push(cell);

            let east_bound = cell.east.is_none();
            let south_bound = cell.south.is_none();

            let close = east_bound || (!south_bound && rng.gen_range(0, 3) == 0);

            if close {
                if let Some(choice) = rng.choose(&run) {
                    if let Some(ref pos) = choice.south {
                        links.push((choice.pos.clone(), pos.clone()));
                    }
                }
                run.clear();
            } else if let Some(ref pos) = cell.east {
                links.push((cell.pos.clone(), pos.clone()));
            }

        }
    }
    links.iter().map(|(p1, p2)| grid.link(p1, p2)).collect::<()>();
}

fn main() {
    let mut grid = Grid::new(6, 6);
    sidewinder(&mut grid);
    println!("{}", grid.to_string(false));
}
