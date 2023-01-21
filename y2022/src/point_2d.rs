use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

pub trait Abs<T>
where
    T: Copy,
{
    fn abs(self) -> Self;
}
impl Abs<i32> for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Abs<i64> for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Abs<f32> for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Abs<f64> for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
pub trait Sqrt<T> {
    fn sqrt(self) -> Self;
}
impl Sqrt<f64> for f64 {
    fn sqrt(self: f64) -> Self {
        self.sqrt()
    }
}
impl Sqrt<f32> for f32 {
    fn sqrt(self: f32) -> Self {
        self.sqrt()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Rect<T> {
    pub northwest: Point2d<T>,
    pub southeast: Point2d<T>,
}
impl<T: PartialOrd<T> + Copy + From<i32> + Sub<Output = T> + Add<Output = T> + Mul<Output = T>>
    Rect<T>
{
    pub fn contains(&self, point: Point2d<T>) -> bool {
        (self.northwest.x..=self.southeast.x).contains(&point.x)
            && (self.northwest.y..=self.southeast.y).contains(&point.y)
    }
    pub fn area(&self) -> T {
        let x = self.southeast.x - self.northwest.x + 1.into();
        let y = self.southeast.y - self.northwest.y + 1.into();
        x * y
    }
    pub fn dims(&self) -> (T, T) {
        let xdim = (self.southeast.x - self.northwest.x) + 1.into();
        let ydim = (self.southeast.y - self.northwest.y) + 1.into();
        (xdim, ydim)
    }
    pub fn bound(points: impl Iterator<Item = Point2d<T>>) -> Rect<T> {
        let bounds = points.fold(((0.into(), 0.into()), (0.into(), 0.into())), |b, p| {
            let (xmin, xmax) = b.0;
            let (ymin, ymax) = b.1;

            (
                if p.x < xmin {
                    (p.x, xmax)
                } else if p.x > xmax {
                    (xmin, p.x)
                } else {
                    (xmin, xmax)
                },
                if p.y < ymin {
                    (p.y, ymax)
                } else if p.y > ymax {
                    (ymin, p.y)
                } else {
                    (ymin, ymax)
                },
            )
        });
        Self {
            northwest: (bounds.0 .0, bounds.1 .0).into(),
            southeast: (bounds.0 .1, bounds.1 .1).into(),
        }
    }
}

impl<T> From<(T, T)> for Point2d<T> {
    fn from(t: (T, T)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

pub trait Rem<T> {
    fn rem_euclid(self, divisor: T) -> T;
}
impl Rem<i32> for i32 {
    fn rem_euclid(self, divisor: i32) -> i32 {
        self.rem_euclid(divisor)
    }
}
impl Rem<i64> for i64 {
    fn rem_euclid(self, divisor: i64) -> i64 {
        self.rem_euclid(divisor)
    }
}
impl Rem<usize> for usize {
    fn rem_euclid(self, divisor: usize) -> usize {
        self.rem_euclid(divisor)
    }
}
impl Rem<u32> for u32 {
    fn rem_euclid(self, divisor: u32) -> u32 {
        self.rem_euclid(divisor)
    }
}
impl Rem<u64> for u64 {
    fn rem_euclid(self, divisor: u64) -> u64 {
        self.rem_euclid(divisor)
    }
}

impl<T> Point2d<T>
where
    T: Rem<T>,
{
    pub fn wrap(self, xmax: T, ymax: T) -> Self {
        Self {
            x: self.x.rem_euclid(xmax),
            y: self.y.rem_euclid(ymax),
        }
    }
}

impl<T> Point2d<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Copy
        + Abs<T>
        + Sub<Output = T>
        + Div<Output = T>
        + From<i32>
        + PartialOrd<T>,
{
    pub fn origin() -> Self {
        Self::new(0.into(), 0.into())
    }

    pub fn northwest(self) -> Self {
        Self {
            x: self.x - 1.into(),
            y: self.y - 1.into(),
        }
    }

    pub fn north(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1.into(),
        }
    }
    pub fn northeast(self) -> Self {
        Self {
            x: self.x + 1.into(),
            y: self.y - 1.into(),
        }
    }
    pub fn west(self) -> Self {
        Self {
            x: self.x - 1.into(),
            y: self.y,
        }
    }
    pub fn southwest(self) -> Self {
        Self {
            x: self.x - 1.into(),
            y: self.y + 1.into(),
        }
    }
    pub fn east(self) -> Self {
        Self {
            x: self.x + 1.into(),
            y: self.y,
        }
    }
    pub fn southeast(self) -> Self {
        Self {
            x: self.x + 1.into(),
            y: self.y + 1.into(),
        }
    }
    pub fn south(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1.into(),
        }
    }
    pub fn around(self) -> Vec<Self> {
        vec![
            self.northwest(),
            self.north(),
            self.northeast(),
            self.west(),
            self.east(),
            self.southwest(),
            self.south(),
            self.southeast(),
        ]
    }
    pub fn shift(&mut self, offset: &Self) {
        *self = *self + *offset;
    }

    pub fn clamp(self, xbound: (T, T), ybound: (T, T)) -> Self {
        Self {
            x: if self.x < xbound.0 {
                xbound.0
            } else if self.x > xbound.1 {
                xbound.1
            } else {
                self.x
            },
            y: if self.y < ybound.0 {
                ybound.0
            } else if self.y > ybound.1 {
                ybound.1
            } else {
                self.y
            },
        }
    }

    pub fn new(x: T, y: T) -> Self {
        Point2d { x, y }
    }
    pub fn scale(&self, factor: T) -> Self {
        *self * Self::new(factor, factor)
    }
    pub fn invscale(&self, divisor: T) -> Self {
        Self::new(self.x / divisor, self.y / divisor)
    }
    pub fn abs(self) -> Point2d<T> {
        Self::new(self.x.abs(), self.y.abs())
    }
    pub fn sum(self) -> T {
        self.x + self.y
    }
    pub fn manhattan(self) -> T {
        self.abs().sum()
    }
    pub fn square(self) -> Self {
        self * self
    }
}

impl<T: Sqrt<T> + Add<Output = T>> Point2d<T> {
    pub fn euclid(self) -> T {
        (self.x + self.y).sqrt()
    }
}

impl<T: Add<Output = T>> Add for Point2d<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point2d<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point2d<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Point2d<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
