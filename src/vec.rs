use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::ops::{Add, Mul};

use crate::core::Direction;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16,
}

impl Vec2 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn from_direction(d: Direction) -> Vec2 {
        match d {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
        .into()
    }
}

impl Mul<i16> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i16) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vec2 {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Vec2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state)
    }
}

impl From<(i16, i16)> for Vec2 {
    fn from(t: (i16, i16)) -> Self {
        Vec2 { x: t.0, y: t.1 }
    }
}
