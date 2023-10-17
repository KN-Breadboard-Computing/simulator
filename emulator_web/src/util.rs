use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use egui::{Pos2, Vec2};

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn equals_with_rounding(&self, other: Pos2, epsilon: f32) -> bool {
        let x = self.x as f32;
        let y = self.y as f32;

        (x - other.x).abs() <= epsilon && (y - other.y).abs() <= epsilon
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct IRect {
    pub pos: IVec2,
    pub size: IVec2,
}

pub fn irect(pos: IVec2, size: IVec2) -> IRect {
    IRect::new(pos, size)
}

impl IRect {
    pub fn new(pos: IVec2, size: IVec2) -> Self {
        Self { pos, size }
    }

    pub fn contains(&self, t: IVec2) -> bool {
        let bot_right = self.pos + self.size;
        let top_left = self.pos;

        (top_left.x..=bot_right.x).contains(&t.x) && (top_left.y..=bot_right.y).contains(&t.y)
    }

    pub fn contains_with_rounding(&self, t: Pos2, epsilon: f32) -> bool {
        let bot_right: Pos2 = (self.pos + self.size).into();
        let top_left: Pos2 = self.pos.into();

        (top_left.x - epsilon..=bot_right.x + epsilon).contains(&t.x)
            && (top_left.y - epsilon..=bot_right.y + epsilon).contains(&t.y)
    }
}

impl From<Pos2> for IVec2 {
    fn from(value: Pos2) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}

impl From<Vec2> for IVec2 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32,
        }
    }
}

impl From<IVec2> for Pos2 {
    fn from(value: IVec2) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<IVec2> for Vec2 {
    fn from(value: IVec2) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl Add for IVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for IVec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for IVec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for IVec2 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs
    }
}

impl Mul<IVec2> for i32 {
    type Output = IVec2;

    fn mul(self, rhs: IVec2) -> Self::Output {
        IVec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<i32> for IVec2 {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs
    }
}

impl Div<IVec2> for i32 {
    type Output = IVec2;

    fn div(self, rhs: IVec2) -> Self::Output {
        IVec2 {
            x: rhs.x / self,
            y: rhs.y / self,
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::{vec2, pos2};

    use super::*;

    #[test]
    fn test_eq_round() {
        let ivec = ivec2(40, -30);

        let fvec = pos2(40.5, -29.2);

        assert!(ivec.equals_with_rounding(fvec, 1.0));
        assert!(ivec.equals_with_rounding(fvec, 0.81));
        assert!(!ivec.equals_with_rounding(fvec, 0.79));
        assert!(!ivec.equals_with_rounding(fvec, 0.2));
    }
}
