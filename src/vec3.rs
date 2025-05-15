#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    #[inline]
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn with(v: (f32, f32, f32)) -> Self {
        Self(v.0, v.1, v.2)
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }

    fn length_squared(&self) -> f32 {
        self.0.powf(2f32) + self.1.powf(2f32) + self.2.powf(2f32)
    }

    // sqrt(x**2 + y**2 + z**2)
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0-rhs.0, self.1-rhs.1, self.2-rhs.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

pub type Point3 = Vec3;

// TODO: Tests on Assign Ops
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn vec_neg() {
        assert_eq!(-Vec3(3.0, 4.0, 5.0), Vec3(-3.0, -4.0, -5.0));
    }

    #[test]
    pub fn vec_sub() {
        assert_eq!(Vec3(3.0, 4.0, 5.0) - Vec3(2.0, 3.0, 4.0), Vec3(1.0, 1.0, 1.0))
    }

    #[test]
    pub fn vec_get() {
        let vec = Vec3::with((0.0, 1.0, 2.0));
        
        assert_eq!(vec.x(), 0.0);
        assert_eq!(vec.y(), 1.0);
        assert_eq!(vec.z(), 2.0);
    }
}

