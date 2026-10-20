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
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Barycentric(Vec3f);

impl AABoundingBox {
    pub const fn new(min: Vec2i, max: Vec2i) -> Self {
        Self { min, max }
    }

    /// Every pixel coordinate in the box, row by row.
    pub fn points(&self) -> impl Iterator<Item = Vec2i> + use<> {
        let (min, max) = (self.min, self.max);
        (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| Vec2i::new(x, y)))
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

        if triangle.doubled_signed_area() == 0 {
            None
        } else {
            Some(triangle)
        }
    }

    /// The three sides, each running in the triangle's winding order:
    /// a → b, b → c, c → a.
    pub fn edges(&self) -> [Line; 3] {
        [
            Line::new(self.a, self.b),
            Line::new(self.b, self.c),
            Line::new(self.c, self.a),
        ]
    }

    /// Twice the signed area, exact. Positive when a → b → c runs
    /// counter-clockwise (y up). Never zero.
    pub fn doubled_signed_area(&self) -> i32 {
        (self.b - self.a).cross(self.c - self.a)
    }

    /// /// Position of `p` in barycentric coordinates relative to this triangle: the
    /// weights (α, β, γ) with p = αa + βb + γc.
    pub fn barycentric(&self, p: Vec2i) -> Barycentric {
        let abc = self.doubled_signed_area() as f32;
        let pbc = (self.b - p).cross(self.c - self.b) as f32;
        let apc = (p - self.a).cross(self.c - p) as f32;

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

    /// 2D cross product: the z component of the 3D cross product with both
    /// vectors on the z = 0 plane. Equals twice the signed area of the
    /// triangle (0, self, rhs).
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
