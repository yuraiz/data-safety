use num::pow::Pow;
use num::BigInt;

use num::traits::Euclid;
use num::One;
use num::Zero;

#[derive(Debug, Default, Clone)]
pub struct ECPoint {
    pub x: BigInt,
    pub y: BigInt,
    pub a: BigInt,
    pub b: BigInt,
    pub p: BigInt,
}

impl ECPoint {
    pub fn mod_inverse(mut b: BigInt, p: BigInt) -> BigInt {
        let (mut x0, mut x1, mut y0, mut y1, mut n) = (
            BigInt::one(),
            BigInt::zero(),
            BigInt::zero(),
            BigInt::one(),
            p.clone(),
        );

        let mut q;

        while !n.is_zero() {
            (q, b, n) = (b.clone() / n.clone(), n.clone(), b.rem_euclid(&n));
            (x0, x1) = (x1.clone(), x0 - q.clone() * x1);
            (y0, y1) = (y1.clone(), y0 - q * y1);
        }

        x0.rem_euclid(&p)
    }
}

impl std::str::FromStr for ECPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<BigInt> = s
            .split_ascii_whitespace()
            .map(BigInt::from_str)
            .flatten()
            .collect();

        let [x, y, a, b, p] = nums.try_into().map_err(|_| ())?;

        Ok(Self { x, y, a, b, p })
    }
}

impl std::fmt::Display for ECPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, a, b, p } = self;

        write!(f, "{x} {y} {a} {b} {p}")
    }
}
impl std::ops::Add for ECPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = ECPoint {
            a: self.a.clone(),
            b: self.b.clone(),
            p: self.p.clone(),
            ..Default::default()
        };

        let dx = (rhs.x.clone() - self.x.clone()).rem_euclid(&self.p);
        let dy = (rhs.y.clone() - self.y.clone()).rem_euclid(&self.p);

        let l = if self.x == rhs.x && self.y == rhs.y {
            ((3u8 * self.x.clone().pow(2u8) + self.a.clone())
                * Self::mod_inverse(2 * self.y.clone(), self.p.clone()))
            .rem_euclid(&self.p)
        } else {
            if self.x == rhs.x {
                panic!("infinity");
            }
            let dx_inverse = Self::mod_inverse(dx, self.p.clone());
            (dy * dx_inverse) % self.p.clone()
        };
        result.x = (l.clone() * l.clone() - self.x.clone() - rhs.x.clone()).rem_euclid(&self.p);
        result.y = (l.clone() * (self.x.clone() - result.x.clone()) - self.y).rem_euclid(&self.p);

        result
    }
}

impl std::ops::AddAssign for ECPoint {
    fn add_assign(&mut self, rhs: Self) {
        let temp = std::mem::take(self);
        *self = temp + rhs;
    }
}

impl std::ops::Mul<ECPoint> for i32 {
    type Output = ECPoint;

    // Multiplication EC point and integer
    fn mul(self, rhs: ECPoint) -> Self::Output {
        let mut result = rhs.clone();
        let mut temp = rhs.clone();
        let mut x = self - 1;
        while x != 0 {
            if x % 2 != 0 {
                result += temp.clone();
                x -= 1;
            }
            x /= 2;
            temp += temp.clone();
        }
        result
    }
}

impl std::ops::Mul<ECPoint> for BigInt {
    type Output = ECPoint;

    // Multiplication EC point and integer
    fn mul(self, rhs: ECPoint) -> Self::Output {
        let mut result = rhs.clone();
        let mut temp = rhs.clone();
        let mut x = self - BigInt::one();
        while !x.is_zero() {
            if x.bit(0) {
                result += temp.clone();
                x -= 1;
            }
            x /= 2;
            temp += temp.clone();
        }
        result
    }
}
