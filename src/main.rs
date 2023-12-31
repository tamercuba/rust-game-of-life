mod domain;
mod tests;

use std::thread::sleep;
use std::time::Duration;

use domain::grid::Grid;
use domain::cells::finite_plane_cell::FinitePlaneCell;

fn main() {
  const GRID_SIZE: i16 = 20;
  const FPS: i16 = 1;

  let mut grid = Grid::new(GRID_SIZE);
  grid.update_field(FinitePlaneCell::new(10, 10, GRID_SIZE).unwrap());
  grid.update_field(FinitePlaneCell::new(10, 11, GRID_SIZE).unwrap());
  grid.update_field(FinitePlaneCell::new(10, 12, GRID_SIZE).unwrap());
  grid.update_field(FinitePlaneCell::new(11, 11, GRID_SIZE).unwrap());

  loop {
    sleep(Duration::from_millis(1000 / (FPS as u64)));
    print!("{esc}c", esc = 27 as char);
    println!("Generation: {}", grid.get_age());
    println!("{}", grid);
    grid.mutate::<FinitePlaneCell>();
  }
}
