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

impl Vec2 {
    pub fn as_pipe(&self) -> Pipes {
        match *self {
            Vec2::NORTH => Pipes::NORTH,
            Vec2::SOUTH => Pipes::SOUTH,
            Vec2::EAST => Pipes::EAST,
            Vec2::WEST => Pipes::WEST,
            _ => panic!(),
        }
    }
}

impl Add<Pipes> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Pipes) -> Self::Output {
        let mut out = self;
        for pipe in rhs.iter() {
            match pipe {
                Pipes::NORTH => out += Vec2::NORTH,
                Pipes::SOUTH => out += Vec2::SOUTH,
                Pipes::EAST => out += Vec2::EAST,
                Pipes::WEST => out += Vec2::WEST,
                _ => {}
            };
        }
        return out;
    }
}

impl Add<Vec2> for Pipes {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        return rhs + self;
    }
}
