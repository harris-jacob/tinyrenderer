use std::ops::{Add, Mul, Neg, Sub};

/// Integer point, used for anything that addresses pixels.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

pub const fn vec2i(x: i32, y: i32) -> Vec2i {
    Vec2i { x, y }
}

impl Vec2i {
    pub const ZERO: Vec2i = vec2i(0, 0);

    pub fn dot(self, rhs: Vec2i) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }

    // Not a true cross, 2d equivalent if points placed
    // on a plane at z=0;
    pub fn cross(self, rhs: Vec2i) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn min(self, rhs: Vec2i) -> Vec2i {
        vec2i(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    pub fn max(self, rhs: Vec2i) -> Vec2i {
        vec2i(self.x.max(rhs.x), self.y.max(rhs.y))
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, rhs: Vec2i) -> Vec2i {
        vec2i(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;
    fn sub(self, rhs: Vec2i) -> Vec2i {
        vec2i(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Vec2i {
    type Output = Vec2i;
    fn mul(self, rhs: i32) -> Vec2i {
        vec2i(self.x * rhs, self.y * rhs)
    }
}

impl Neg for Vec2i {
    type Output = Vec2i;
    fn neg(self) -> Vec2i {
        vec2i(-self.x, -self.y)
    }
}

/// Float vector, used for model space, barycentric weights and normals.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn vec3f(x: f32, y: f32, z: f32) -> Vec3f {
    Vec3f { x, y, z }
}

impl Vec3f {
    pub const ZERO: Vec3f = vec3f(0.0, 0.0, 0.0);

    pub fn dot(self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3f) -> Vec3f {
        vec3f(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns the zero vector if `self` has no length, rather than NaNs.
    pub fn normalized(self) -> Vec3f {
        let len = self.length();
        if len == 0.0 {
            Vec3f::ZERO
        } else {
            self * (1.0 / len)
        }
    }

    /// Drop z. Rounds rather than truncates, so screen-space positions land on
    /// the nearest pixel.
    pub fn to_vec2i(self) -> Vec2i {
        vec2i(self.x.round() as i32, self.y.round() as i32)
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        vec3f(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        vec3f(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Vec3f {
        vec3f(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        vec3f(-self.x, -self.y, -self.z)
    }
}

impl From<Vec2i> for Vec3f {
    fn from(v: Vec2i) -> Vec3f {
        vec3f(v.x as f32, v.y as f32, 0.0)
    }
}
