use std::{fmt::Display, str::FromStr};

use ndarray::{
    Array2, Dim, ShapeError,
    iter::{Lanes, LanesMut},
};

use crate::util::{
    char::{FromChar, ToStyledChar},
    output::Output,
    style::StyleTracker,
};

#[derive(Clone)]
pub struct Grid<T>(Array2<T>);

impl<T> Grid<T> {
    pub fn new(width: u32, height: u32) -> Self
    where
        T: Default,
    {
        let array = Array2::default(sh(width, height));
        Self(array)
    }

    pub fn from_fn<F>(width: u32, height: u32, mut f: F) -> Self
    where
        F: FnMut(u32, u32) -> T,
    {
        let array = Array2::from_shape_fn(sh(width, height), |(x, y)| f(x as u32, y as u32));
        Self(array)
    }

    pub fn from_elem(width: u32, height: u32, elem: T) -> Self
    where
        T: Clone,
    {
        let array = Array2::from_elem(sh(width, height), elem);
        Self(array)
    }

    pub fn from_vec(width: u32, height: u32, v: Vec<T>) -> Result<Self, ShapeError> {
        let array = Array2::from_shape_vec(sh(width, height), v)?;
        Ok(Self(array))
    }

    pub const fn from_array(array: Array2<T>) -> Self {
        Self(array)
    }

    pub fn width(&self) -> u32 {
        self.0.ncols() as u32
    }

    pub fn height(&self) -> u32 {
        self.0.nrows() as u32
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.0.get(sh(x, y))
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.0.get_mut(sh(x, y))
    }

    pub fn set(&mut self, x: u32, y: u32, value: T) -> Option<T> {
        self.0
            .get_mut(sh(x, y))
            .map(|prev| std::mem::replace(prev, value))
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        self.0.as_slice_memory_order().and_then(|s| s.get(index))
    }

    pub fn get_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0
            .as_slice_memory_order_mut()
            .and_then(|s| s.get_mut(index))
    }

    pub fn set_at_mut(&mut self, index: usize, value: T) -> Option<T> {
        self.0
            .as_slice_memory_order_mut()
            .and_then(|s| s.get_mut(index))
            .map(|prev| std::mem::replace(prev, value))
    }

    pub fn rows<'g>(&'g self) -> Lanes<'g, T, Dim<[usize; 1]>> {
        self.0.rows()
    }

    pub fn rows_mut<'g>(&'g mut self) -> LanesMut<'g, T, Dim<[usize; 1]>> {
        self.0.rows_mut()
    }

    pub fn columns<'g>(&'g self) -> Lanes<'g, T, Dim<[usize; 1]>> {
        self.0.columns()
    }

    pub fn columns_mut<'g>(&'g mut self) -> LanesMut<'g, T, Dim<[usize; 1]>> {
        self.0.columns_mut()
    }

    pub fn transpose(&mut self) {
        self.0.swap_axes(0, 1);
    }
}

#[inline(always)]
const fn sh(width: u32, height: u32) -> (usize, usize) {
    (height as usize, width as usize)
}

impl<T> FromStr for Grid<T>
where
    T: FromChar,
{
    type Err = ParseGridError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut v = Vec::new();

        for (lidx, line) in s.lines().enumerate() {
            height += 1;
            for (cidx, ch) in line.chars().enumerate() {
                if lidx == 0 {
                    width += 1;
                }

                let elem = T::from_char(ch).map_err(|error| ParseGridError::Element {
                    position: (cidx as u32, lidx as u32),
                    error,
                })?;

                v.push(elem);
            }
        }

        let grid = Self::from_vec(width, height, v)?;
        Ok(grid)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseGridError<E> {
    #[error(transparent)]
    Shape(#[from] ShapeError),
    #[error("{error} (at {}, {})", position.0, position.1)]
    Element { position: (u32, u32), error: E },
}

impl<T> Display for Grid<T>
where
    T: ToStyledChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style = StyleTracker::default();
        for (y, row) in self.0.rows().into_iter().enumerate() {
            if y != 0 {
                writeln!(f)?;
            }

            for elem in row {
                let sch = elem.to_styled_char();
                let style = style.style(sch.style);
                write!(f, "{style}{}", sch.ch)?;
            }

            let clear = style.clear();
            write!(f, "{clear}")?;
        }

        Ok(())
    }
}

impl<T> Output for Grid<T>
where
    T: ToStyledChar,
{
    fn is_multiline(&self) -> bool {
        true
    }
}
