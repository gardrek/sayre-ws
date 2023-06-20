// Generic Vector class, with any number of elements
// This is overkill since we only really use 2D vectors

#![allow(dead_code)]

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::*;

pub trait Vectorable:
    Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Copy
{
}

impl<T> Vectorable for T where
    T: Neg<Output = Self>
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Copy
{
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize>(pub [T; N])
where
    T: Vectorable;

//~ #[derive(Debug, Clone, Copy, PartialEq)]
//~ pub struct Vector<const N: usize>(pub [f64; N]);

impl<T: Vectorable> Vector<T, 2> {
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn unpack(&self) -> (T, T) {
        let Vector([x, y]) = self;
        (*x, *y)
    }

    pub fn rotate_by_vector(&self, other: Self) -> Self {
        let new_x = self.x() * other.x() - self.y() * other.y();
        let new_y = self.x() * other.y() + self.y() * other.x();
        Self([new_x, new_y])
    }
}

impl Vector<f64, 2> {
    pub fn rotate_by_angle(&self, angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let new_x = self.x() * cos_a - self.y() * sin_a;
        let new_y = self.x() * sin_a + self.y() * cos_a;
        Vector([new_x, new_y])
    }
}

impl Vector<f64, 3> {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn unpack(&self) -> (f64, f64, f64) {
        let Vector([x, y, z]) = self;
        (*x, *y, *z)
    }
}

impl<const N: usize> Default for Vector<f64, N> {
    fn default() -> Self {
        Vector::zero()
    }
}

impl<const N: usize> Vector<f64, N> {
    pub const fn zero() -> Self {
        Vector([0.0; N])
    }

    pub fn mag(&self) -> f64 {
        self.magsqr().sqrt()
    }

    pub fn magsqr(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, x| acc + x * x)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |acc, (x, y)| acc + x * y)
    }

    pub fn project2d(&self, other: &Self) -> Self {
        let n = self.dot(other);
        let n = n / other.magsqr();
        *other * n
    }

    /// note this function does not return a reasonable answer for (0, 0)
    /// indeed, it will panic for integers, but return an *infinitely long* vector for floats
    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    pub fn abs(&self) -> Self {
        let a = self.0.iter().map(|x| x.abs());

        a.collect::<Self>()
    }
}

// Vector unary op(s)

impl<const N: usize, T: Vectorable> Neg for Vector<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let a = self.0.iter().map(|&x| -x);

        a.collect::<Self>()
    }
}

// Vector-Vector binary op(s)

impl<const N: usize, T: Vectorable> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let a = self.0.iter().zip(other.0.iter()).map(|(&a, &b)| a + b);

        a.collect::<Self>()
    }
}

impl<const N: usize, T: Vectorable> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let a = self.0.iter().zip(other.0.iter()).map(|(&a, &b)| a - b);

        a.collect::<Self>()
    }
}

impl<const N: usize, T: Vectorable> Mul for Vector<T, N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let a = self.0.iter().zip(other.0.iter()).map(|(&a, &b)| a * b);

        a.collect::<Self>()
    }
}

// Vector-float binary op(s)

impl<const N: usize, T: Vectorable> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let a = self.0.iter().map(|&a| a * other);

        a.collect::<Self>()
    }
}

impl<const N: usize, T: Vectorable> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let a = self.0.iter().map(|&a| a / other);

        a.collect::<Self>()
    }
}

// misc

impl<const N: usize, T: Vectorable + Display> Display for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Vector(")?;

        let _inner = self
            .0
            .iter()
            .map(|n| write!(f, "{}, ", n))
            .collect::<Result<(), _>>()?;

        write!(f, ")")
    }
}

impl<const N: usize, T: Vectorable> FromIterator<T> for Vector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vector::<T, N>(
            iter.into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| unreachable!()),
        )
    }
}

impl<const N: usize, T: Vectorable> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v = self.0.iter().cloned().collect::<Vec<_>>();
        v.into_iter()
    }
}
/*
function Vector:unpack()
  local t = {}
  for i = 1, #self do
    t[i] = self[i]
  end
  return unpack(t)
end

function Vector:__add(other)
  local r = Vector:new(#self)
  if type(other) == 'number' then
    for i = 1, #self do
      r[i] = self[i] + other
    end
  else
    if #self ~= #other then error('Attempt to add unlike Vectors.', 2) end
    for i = 1, #self do
      r[i] = self[i] + other[i]
    end
  end
  return r
end

function Vector:__sub(other)
  local r = Vector:new(#self)
  if type(other) == 'number' then
    for i = 1, #self do
      r[i] = self[i] - other
    end
  elseif type(other) == 'table' and other.class == 'Vector' then
    if #self ~= #other then error('Attempt to subtract unlike Vectors.', 2) end
    for i = 1, #self do
      r[i] = self[i] - other[i]
    end
  else
    error(tostring(other), 2)
  end
  return r
end

function Vector:__mul(other)
  local r = Vector:new(#self)
  if type(other) == 'number' then
    for i = 1, #self do
      r[i] = self[i] * other
    end
  elseif type(other) == 'table' and other.class == 'Vector' then
    if #self ~= #other then error('Attempt to multiply unlike Vectors.', 2) end
    for i = 1, #self do
      r[i] = self[i] * other[i]
    end
  end
  return r
end

function Vector:__div(other)
  local r = Vector:new(#self)
  if type(other) == 'number' then
    for i = 1, #self do
      r[i] = self[i] / other
    end
  else
    if #self ~= #other then error('Attempt to divide unlike Vectors.', 2) end
    for i = 1, #self do
      r[i] = self[i] / other[i]
    end
  end
  return r
end

function Vector:__unm()
  local r = Vector:new(#self)
  for i = 1, #self do
    r[i] = -self[i]
  end
  return r
end


return Vector
// */
