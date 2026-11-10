//! A minimal f64 vector type for the ephemeris.
//!
//! Deliberately not `glam`: the astronomy wants f64 throughout, the whole need
//! is a dot, a cross and a normalize, and the render side converts to f32 at the
//! boundary anyway.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

impl Vec3 {
    pub const ZERO: Vec3 = vec3(0.0, 0.0, 0.0);

    /// Unit vector from ecliptic longitude and latitude, in degrees.
    pub fn from_lon_lat_deg(lon: f64, lat: f64) -> Vec3 {
        let (lo, la) = (lon.to_radians(), lat.to_radians());
        vec3(la.cos() * lo.cos(), la.cos() * lo.sin(), la.sin())
    }

    pub fn dot(self, o: Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(self, o: Vec3) -> Vec3 {
        vec3(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }

    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Unit vector in the same direction; the zero vector maps to itself.
    pub fn normalize(self) -> Vec3 {
        let l = self.len();
        if l > 0.0 { self * (1.0 / l) } else { self }
    }

    /// Ecliptic longitude, degrees in `[0, 360)`.
    pub fn lon_deg(self) -> f64 {
        crate::ephem::wrap360(self.y.atan2(self.x).to_degrees())
    }

    /// Ecliptic latitude, degrees in `[-90, 90]`.
    pub fn lat_deg(self) -> f64 {
        (self.z / self.len()).clamp(-1.0, 1.0).asin().to_degrees()
    }

    pub fn to_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        vec3(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        vec3(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 {
        vec3(self.x * s, self.y * s, self.z * s)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        vec3(-self.x, -self.y, -self.z)
    }
}

/// An orthonormal basis, stored as its three column vectors.
#[derive(Debug, Clone, Copy)]
pub struct Basis {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Basis {
    /// Transform a vector expressed in this basis into the parent frame.
    pub fn apply(&self, v: Vec3) -> Vec3 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Transform a parent-frame vector into this basis (the transpose, valid
    /// because the basis is orthonormal).
    pub fn unapply(&self, v: Vec3) -> Vec3 {
        vec3(self.x.dot(v), self.y.dot(v), self.z.dot(v))
    }
}
