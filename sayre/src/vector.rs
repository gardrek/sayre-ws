// Generic Vector class, with any number of elements
// This is overkill since we only really use 2D vectors

use crate::gfx;

use std::ops::*;

pub trait BasicMath {}

impl<T> BasicMath for T where T: Add + Sub + Mul + Div {}

/*
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize, T>(pub [T; N]) where T: BasicMath;
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>(pub [f64; N]);

pub fn new_2d(x: f64, y: f64) -> Vector<2> {
    Vector::<2>([x, y])
}

pub fn new_3d(x: f64, y: f64, z: f64) -> Vector<3> {
    Vector::<3>([x, y, z])
}

impl Vector<2> {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn rotate_by_angle(&self, angle: f64) -> Vector<2> {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let new_x = self.x() * cos_a - self.y() * sin_a;
        let new_y = self.x() * sin_a + self.y() * cos_a;
        Vector::<2>([new_x, new_y])
    }

    pub fn rotate_by_vector(&self, other: Vector<2>) -> Vector<2> {
        let new_x = self.x() * other.x() - self.y() * other.y();
        let new_y = self.x() * other.y() + self.y() * other.x();
        Vector::<2>([new_x, new_y])
    }

    pub fn draw<T: gfx::RenderTarget>(
        &self,
        target: &mut T,
        x: f64,
        y: f64,
        scale: f64,
        arrow_length: f64,
    ) {
        let scale = if scale <= 0.0 { 16.0 } else { scale };
        let arrow_length = if arrow_length < 0.0 {
            4.0
        } else {
            arrow_length
        };

        let (buffer, run) = target.get_buffer();

        let t = *self * scale;

        if self.mag() != 0.0 {
            let angle = std::f64::consts::PI / 6.0;

            if arrow_length > 0.0 {
                // draw arrow head
                let m = t.mag() / arrow_length;
                let a = t.rotate_by_angle(angle).norm() * -m;
                let b = t.rotate_by_angle(-angle).norm() * -m;
                gfx::line(
                    buffer,
                    run,
                    t.x() + x,
                    t.y() + y,
                    t.x() + x + a.x(),
                    t.y() + y + a.y(),
                );
                gfx::line(
                    buffer,
                    run,
                    t.x() + x,
                    t.y() + y,
                    t.x() + x + b.x(),
                    t.y() + y + b.y(),
                );
            }

            gfx::line(buffer, run, x, y, t.x() + x, t.y() + y);
        }
    }
}

impl Vector<3> {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Vector::zero()
    }
}

impl<const N: usize> Vector<N> {
    pub fn zero() -> Vector<N> {
        Vector([0.0; N])
    }

    pub fn mag(&self) -> f64 {
        self.magsqr().sqrt()
    }

    pub fn magsqr(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, x| acc + x * x)
    }

    pub fn dot(&self, other: &Vector<N>) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |acc, (x, y)| acc + x * y)
    }

    pub fn project2d(&self, other: &Vector<N>) -> Vector<N> {
        let n = self.dot(other);
        let n = n / other.magsqr();
        *other * n
    }

    pub fn norm(&self) -> Vector<N> {
        *self / self.mag()
    }

    pub fn abs(&self) -> Vector<N> {
        let a = self.0.iter().map(|x| x.abs());

        a.collect::<Vector<N>>()
    }
}

// Vector unary op(s)

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let a = self.0.iter().map(|x| -x);

        a.collect::<Vector<N>>()
    }
}

// Vector-Vector binary op(s)

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let a = self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b);

        a.collect::<Vector<N>>()
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let a = self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b);

        a.collect::<Vector<N>>()
    }
}

// Vector-float binary op(s)

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        let a = self.0.iter().map(|a| a * other);

        a.collect::<Vector<N>>()
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        let a = self.0.iter().map(|a| a / other);

        a.collect::<Vector<N>>()
    }
}

// misc

impl<const N: usize> std::fmt::Display for Vector<N> {
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

impl<const N: usize> FromIterator<f64> for Vector<N> {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Vector::<N>(iter.into_iter().collect::<Vec<_>>().try_into().unwrap())
    }
}

impl<const N: usize> IntoIterator for Vector<N> {
    type Item = f64;
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
