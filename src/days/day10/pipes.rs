use crate::helper::vec2::Vec2;
use std::ops::Add;

use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Pipes: u32 {
        const NORTH = 0x01;
        const SOUTH = 0x02;
        const EAST = 0x04;
        const WEST = 0x08;
        const START = 0x10;
    }
}

impl Vec2<i32> {
    pub fn as_pipe(&self) -> Pipes {
        match *self {
            Self::NORTH => Pipes::NORTH,
            Self::SOUTH => Pipes::SOUTH,
            Self::EAST => Pipes::EAST,
            Self::WEST => Pipes::WEST,
            _ => panic!(),
        }
    }
}

impl Add<Pipes> for Vec2<i32> {
    type Output = Self;

    fn add(self, rhs: Pipes) -> Self::Output {
        let mut out = self;
        for pipe in rhs.iter() {
            match pipe {
                Pipes::NORTH => out += Self::NORTH,
                Pipes::SOUTH => out += Self::SOUTH,
                Pipes::EAST => out += Self::EAST,
                Pipes::WEST => out += Self::WEST,
                _ => {}
            };
        }
        return out;
    }
}

impl Add<Vec2<i32>> for Pipes {
    type Output = Vec2<i32>;

    fn add(self, rhs: crate::helper::vec2::Vec2<i32>) -> Self::Output {
        return rhs + self;
    }
}
