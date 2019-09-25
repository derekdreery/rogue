//! A module to hold all types/logic for grids of tiles (called frames).
use crate::TileSet;
use mint::Vector2;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Frame<T> {
    buf: Vec<T>,
    dims: Vector2<usize>,
}

impl<T> Frame<T>
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

impl<T> Default for Frame<T>
where
    T: Default,
{
    /// Assumes a 80x25 frame size.
    fn default() -> Self {
        Frame::new(80, 25)
    }
}

impl<T> Frame<T> {
    /// Get an item from the grid by location.
    ///
    /// You can also use the implementation of `Index` like so: `frame[(1, 2)]`.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> &T {
        let idx = self.idx(col, row);
        &self.buf[idx]
    }

    /// Get a mutable ref to and item from the grid by location.
    ///
    /// You can also use the implementation of `IndexMut` like so: `frame[(1, 2)] = 2`.
    #[inline]
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let idx = self.idx(col, row);
        &mut self.buf[idx]
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x * self.dims.y + y
    }
}

impl<T> Frame<T>
where
    T: TileSet,
{
    pub fn debug_print(&self) {
        println!("Frame:");
        for row in 0..self.dims.y {
            for col in 0..self.dims.x {
                print!("{}", self.get(row, col).as_char());
            }
            println!()
        }
    }
}

impl<T> Index<(usize, usize)> for Frame<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col)
    }
}

impl<T> IndexMut<(usize, usize)> for Frame<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col)
    }
}
