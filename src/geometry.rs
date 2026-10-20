use std::ops::{Add, Mul, Neg, Sub};

/// Integer point, used for anything that addresses pixels.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

/// Axis-aligned box in pixel coordinates. Both `min` and `max` lie inside the box.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AABoundingBox {
    pub min: Vec2i,
    pub max: Vec2i,
}

/// Barycentric coordinates of a point relative to a triangle ABC: the weights
/// in P = αA + βB + γC. They always sum to 1, and are only built by
/// [`Triangle::barycentric`].
///
/// Stored as (α, β, γ) in x, y, z. The vector is private because its
/// arithmetic would not preserve the sum.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Barycentric(Vec3f);

impl AABoundingBox {
    pub const fn new(min: Vec2i, max: Vec2i) -> Self {
        Self { min, max }
    }
}

impl Barycentric {
    pub fn is_inside(&self) -> bool {
        let Barycentric(w) = self;
        w.x >= 0.0 && w.y >= 0.0 && w.z >= 0.0
    }
}

/// Segment between two pixel coordinates, both endpoints included.
#[allow(dead_code)]
pub struct Line {
    pub a: Vec2i,
    pub b: Vec2i,
}

/// Triangle in pixel coordinates with non-zero area.
pub struct Triangle {
    a: Vec2i,
    b: Vec2i,
    c: Vec2i,
}

impl Triangle {
    /// Returns `None` when the vertices are collinear (zero area).
    pub fn new(a: Vec2i, b: Vec2i, c: Vec2i) -> Option<Self> {
        let triangle = Self { a, b, c };

        if triangle.area() == 0.0 {
            None
        } else {
            Some(triangle)
        }
    }

    /// Positive when a → b → c runs counter-clockwise (y up). Never zero.
    pub fn signed_area(&self) -> f32 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;

        ab.cross(ac) as f32 / 2.0
    }

    pub fn area(&self) -> f32 {
        self.signed_area().abs()
    }

    /// Each weight is the signed area of the triangle with `p` in place of that
    /// weight's vertex, over the signed area of the whole. Both areas share a
    /// winding, so the result does not depend on the order of a, b, c.
    pub fn barycentric(&self, p: Vec2i) -> Barycentric {
        // Sub-areas cross consecutive edges, equal to (b − p) × (c − p) and
        // (p − a) × (c − a).
        let abc = self.signed_area();
        let pbc = (self.b - p).cross(self.c - self.b) as f32 / 2.0;

        let apc = (p - self.a).cross(self.c - p) as f32 / 2.0;

        let alpha = pbc / abc;
        let beta = apc / abc;
        let gamma = 1.0 - alpha - beta;

        Barycentric(Vec3f::new(alpha, beta, gamma))
    }

    pub fn bounding_box(&self) -> AABoundingBox {
        AABoundingBox::new(
            self.a.min(self.b).min(self.c),
            self.a.max(self.b).max(self.c),
        )
    }
}

impl Line {
    #[allow(dead_code)]
    pub const fn new(a: Vec2i, b: Vec2i) -> Self {
        Self { a, b }
    }
}

impl Vec2i {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Not a true cross, 2d equivalent if points placed
    // on a plane at z=0;
    pub fn cross(self, rhs: Vec2i) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn min(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    pub fn max(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;
    fn sub(self, rhs: Vec2i) -> Vec2i {
        Vec2i::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Vec2i {
    type Output = Vec2i;
    fn mul(self, rhs: i32) -> Vec2i {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl Neg for Vec2i {
    type Output = Vec2i;
    fn neg(self) -> Vec2i {
        Vec2i::new(-self.x, -self.y)
    }
}

/// Float vector, used for model space, barycentric weights and normals.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3f {
    pub const ZERO: Vec3f = Vec3f::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(
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
        Vec2i::new(self.x.round() as i32, self.y.round() as i32)
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }
}

impl From<Vec2i> for Vec3f {
    fn from(v: Vec2i) -> Vec3f {
        Vec3f::new(v.x as f32, v.y as f32, 0.0)
    }
}
