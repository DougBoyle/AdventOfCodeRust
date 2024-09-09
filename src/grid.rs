use std::ops::{Index, IndexMut};

use crate::point::Point;

pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    pub width: i32,
    pub height: i32,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        let width = cells[0].len();
        let height = cells.len();
        Grid { cells, width: width as i32, height: height as i32 }
    }

    pub fn parse<F: FnMut(char) -> T>(lines: impl Iterator<Item=String>, mut f: F) -> Self {
        Self::new(lines.map(|line| line.chars().map(|c| f(c)).collect()).collect())
    }

    pub fn is_in_bounds(&self, &Point { x, y }: &Point) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    pub fn map<U, F: FnMut(Point, T) -> U>(self, mut f: F) -> Grid<U> {
        Grid::new(self.cells.into_iter().enumerate()
            .map(|(y, row)| row.into_iter().enumerate()
                .map(|(x, val)| f(Point { x: x as i32, y: y as i32 }, val)).collect())
            .collect())
    }

    pub fn get(&self, p: &Point) -> Option<&T> {
        self.cells.get(p.y as usize).map(|row| row.get(p.x as usize)).flatten()
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: &Point) -> &Self::Output {
        &self.cells[point.y as usize][point.x as usize]
    }
}

impl<T> IndexMut<&Point> for Grid<T> {
    fn index_mut(&mut self, point: &Point) -> &mut Self::Output {
        &mut self.cells[point.y as usize][point.x as usize]
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<TIter: IntoIterator<Item = Vec<T>>>(iter: TIter) -> Self {
        Grid::new(iter.into_iter().collect())
    }
}