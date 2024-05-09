use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn invert(&self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }

    pub const NORTH: Vec2 = Vec2 { x: 0, y: -1 };
    pub const SOUTH: Vec2 = Vec2 { x: 0, y: 1 };
    pub const EAST: Vec2 = Vec2 { x: 1, y: 0 };
    pub const WEST: Vec2 = Vec2 { x: -1, y: 0 };
    pub const ALL_CARDINAL: &'static [Vec2] = &[Vec2::NORTH, Vec2::SOUTH, Vec2::EAST, Vec2::WEST];
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
