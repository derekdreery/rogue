//! A module to hold all types/logic for grids of tiles (called frames).
use crate::TileSet;
use mint::Vector2;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    buf: Vec<T>,
    dims: Vector2<usize>,
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let area = width * height;
        let mut buf = Vec::with_capacity(area);
        for _ in 0..area {
            buf.push(<T as Default>::default());
        }
        Self {
            buf,
            dims: Vector2 {
                x: width,
                y: height,
            },
        }
    }
}

impl<T> Grid<T> {
    pub fn from_fn(width: usize, height: usize, f: impl Fn(usize, usize) -> T) -> Self {
        let mut buf = Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                buf.push(f(width, height));
            }
        }
        Self {
            buf,
            dims: Vector2 {
                x: width,
                y: height,
            },
        }
    }
}

impl<T> Default for Grid<T>
where
    T: Default,
{
    /// Assumes a 80x25 frame size.
    fn default() -> Self {
        Grid::new(80, 25)
    }
}

impl<T> Grid<T> {
    /// Get an item from the grid by location.
    ///
    /// You can also use the implementation of `Index` like so: `frame[(1, 2)]`.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &T {
        let idx = self.idx(x, y);
        &self.buf[idx]
    }

    /// Get a mutable ref to and item from the grid by location.
    ///
    /// You can also use the implementation of `IndexMut` like so: `frame[(1, 2)] = 2`.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let idx = self.idx(x, y);
        &mut self.buf[idx]
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x * self.dims.y + y
    }
}

impl<T> Grid<T>
where
    T: TileSet,
{
    pub fn debug_print(&self) {
        println!("Grid:");
        for row in 0..self.dims.y {
            for col in 0..self.dims.x {
                print!("{}", self.get(row, col).as_char());
            }
            println!()
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get(x, y)
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_mut(x, y)
    }
}
