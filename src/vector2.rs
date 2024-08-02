use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

impl<T: Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x,
            y
        }
    }
}

impl<T> Vector2<T> where T: Add<Output = T> + Mul<Output = T> + Copy {
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T> where T: Mul<Output = T> + Copy {
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T> Mul<T> for Vector2<T> where T: Mul<Output = T> + Copy {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T> Add<Vector2<T>> for Vector2<T> where T: Add<Output = T> + Copy {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T> where T: Sub<Output = T> + Copy {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Into<eframe::egui::Pos2> for Vector2<f32> {
    fn into(self) -> eframe::egui::Pos2 {
        eframe::egui::Pos2 {
            x: self.x,
            y: self.y
        }
    }
}

impl From<eframe::egui::Pos2> for Vector2<f32> {
    fn from(value: eframe::egui::Pos2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}
