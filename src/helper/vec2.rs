use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> Vec2<T>
where
    T: Neg<Output = T> + Copy,
{
    pub fn invert(&self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2<i32> {
    pub const NORTH: Self = Vec2 { x: 0, y: -1 };
    pub const SOUTH: Self = Vec2 { x: 0, y: 1 };
    pub const EAST: Self = Vec2 { x: 1, y: 0 };
    pub const WEST: Self = Vec2 { x: -1, y: 0 };
    pub const ALL_CARDINAL: &'static [Self] = &[Self::NORTH, Self::SOUTH, Self::EAST, Self::WEST];
}

#[allow(dead_code)]
impl Vec2<i64> {
    pub const NORTH: Self = Vec2 { x: 0, y: -1 };
    pub const SOUTH: Self = Vec2 { x: 0, y: 1 };
    pub const EAST: Self = Vec2 { x: 1, y: 0 };
    pub const WEST: Self = Vec2 { x: -1, y: 0 };
    pub const ALL_CARDINAL: &'static [Self] = &[Self::NORTH, Self::SOUTH, Self::EAST, Self::WEST];
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Sum for Vec2<T>
where
    T: Default + Add<Output = T>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2::new(T::default(), T::default()), |a, v| a + v)
    }
}
