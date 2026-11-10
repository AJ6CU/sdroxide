//! f32 vector/matrix helpers for the 3D view.
//!
//! The ephemeris works in f64 (`sdroxide_solar::vec3`); this is the render-side
//! half, where everything is already in gigametres and fits f32 comfortably.
//! Matrices are column-major to match WGSL's `mat4x4<f32>`.

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn v3(x: f32, y: f32, z: f32) -> V3 {
    V3 { x, y, z }
}

impl V3 {
    pub const ZERO: V3 = v3(0.0, 0.0, 0.0);

    pub fn from_f64(v: sdroxide_solar::Vec3) -> V3 {
        v3(v.x as f32, v.y as f32, v.z as f32)
    }

    pub fn dot(self, o: V3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(self, o: V3) -> V3 {
        v3(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
    }

    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> V3 {
        let l = self.len();
        if l > 0.0 { self * (1.0 / l) } else { self }
    }

    pub fn arr(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn arr4(self, w: f32) -> [f32; 4] {
        [self.x, self.y, self.z, w]
    }
}

impl std::ops::Add for V3 {
    type Output = V3;
    fn add(self, o: V3) -> V3 {
        v3(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl std::ops::Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        v3(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl std::ops::Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, s: f32) -> V3 {
        v3(self.x * s, self.y * s, self.z * s)
    }
}
impl std::ops::Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        v3(-self.x, -self.y, -self.z)
    }
}

/// Column-major 4×4 matrix: `cols[i]` is the i-th column.
#[derive(Debug, Clone, Copy)]
pub struct M4 {
    pub cols: [[f32; 4]; 4],
}

impl M4 {
    pub const IDENTITY: M4 = M4 {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// A rigid body placement: an orthonormal basis scaled uniformly by
    /// `scale`, translated to `origin`.
    pub fn from_basis(x: V3, y: V3, z: V3, origin: V3, scale: f32) -> M4 {
        M4 {
            cols: [
                (x * scale).arr4(0.0),
                (y * scale).arr4(0.0),
                (z * scale).arr4(0.0),
                origin.arr4(1.0),
            ],
        }
    }

    pub fn mul(&self, o: &M4) -> M4 {
        let mut cols = [[0.0f32; 4]; 4];
        for (c, out) in cols.iter_mut().enumerate() {
            for r in 0..4 {
                out[r] = (0..4).map(|k| self.cols[k][r] * o.cols[c][k]).sum();
            }
        }
        M4 { cols }
    }

    /// Right-handed look-at: the camera looks down its local −Z.
    pub fn look_at(eye: V3, target: V3, up: V3) -> M4 {
        let f = (target - eye).normalize();
        // Degenerate when the view direction is parallel to `up`; the caller
        // clamps pitch to ±89° so this only guards against numerical drift.
        let mut s = f.cross(up);
        if s.len() < 1e-6 {
            s = f.cross(v3(0.0, 1.0, 0.0));
        }
        let s = s.normalize();
        let u = s.cross(f);
        M4 {
            cols: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
            ],
        }
    }

    /// Reversed-Z perspective for wgpu's `[0, 1]` depth range: the near plane
    /// maps to 1 and the far plane to 0.
    ///
    /// Pair with a depth clear of `0.0` and `CompareFunction::Greater`. This is
    /// what makes a scene spanning 100 km to 240 Gm z-fight-free in a single
    /// `Depth32Float` buffer — float precision bunches near 0, which reversed-Z
    /// puts at the far plane where it is not needed.
    pub fn perspective_reversed_z(fov_y: f32, aspect: f32, near: f32, far: f32) -> M4 {
        let f = 1.0 / (fov_y * 0.5).tan();
        M4 {
            cols: [
                [f / aspect.max(1e-6), 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, near / (far - near), -1.0],
                [0.0, 0.0, far * near / (far - near), 0.0],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform(m: &M4, p: V3, w: f32) -> [f32; 4] {
        let mut o = [0.0f32; 4];
        for (r, out) in o.iter_mut().enumerate() {
            *out = m.cols[0][r] * p.x + m.cols[1][r] * p.y + m.cols[2][r] * p.z + m.cols[3][r] * w;
        }
        o
    }

    #[test]
    fn identity_is_a_multiplicative_unit() {
        let m = M4::from_basis(
            v3(0.0, 1.0, 0.0),
            v3(0.0, 0.0, 1.0),
            v3(1.0, 0.0, 0.0),
            v3(3.0, 4.0, 5.0),
            2.0,
        );
        let p = m.mul(&M4::IDENTITY);
        for c in 0..4 {
            for r in 0..4 {
                assert!((p.cols[c][r] - m.cols[c][r]).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn look_at_puts_the_eye_at_the_origin_looking_down_minus_z() {
        let eye = v3(10.0, 0.0, 0.0);
        let v = M4::look_at(eye, V3::ZERO, v3(0.0, 0.0, 1.0));
        let o = transform(&v, eye, 1.0);
        assert!(o[0].abs() < 1e-5 && o[1].abs() < 1e-5 && o[2].abs() < 1e-5, "eye maps to {o:?}");
        // The target is 10 units ahead, i.e. at z = −10 in view space.
        let t = transform(&v, V3::ZERO, 1.0);
        assert!((t[2] + 10.0).abs() < 1e-4, "target maps to {t:?}");
    }

    /// The whole depth strategy in one test: near must land at 1, far at 0, and
    /// the mapping must stay monotone across five orders of magnitude.
    #[test]
    fn reversed_z_maps_near_to_one_and_far_to_zero() {
        let (near, far) = (1e-4f32, 1e4f32);
        let p = M4::perspective_reversed_z(45f32.to_radians(), 1.5, near, far);
        let depth = |z: f32| {
            let o = transform(&p, v3(0.0, 0.0, -z), 1.0);
            o[2] / o[3]
        };
        assert!((depth(near) - 1.0).abs() < 1e-5, "near → {}", depth(near));
        assert!(depth(far).abs() < 1e-5, "far → {}", depth(far));
        let mut prev = f32::INFINITY;
        for i in 0..64 {
            // Log-spaced samples from near to far.
            let z = near * (far / near).powf(i as f32 / 63.0);
            let d = depth(z);
            assert!(d < prev, "depth not decreasing at z = {z}");
            assert!((0.0..=1.0).contains(&d), "depth {d} out of range at z = {z}");
            prev = d;
        }
    }

    /// Earth-scale detail at 1 AU must stay separable in depth — the exact case
    /// a conventional depth buffer loses.
    ///
    /// Measured in ULPs of the depth value, which is the only meaningful unit:
    /// reversed-Z puts the distant geometry near 0, where f32 steps are ~1e-13,
    /// so an absolute epsilon says nothing about whether z-fighting occurs.
    #[test]
    fn reversed_z_resolves_earth_scale_detail_at_one_au() {
        let ulps_between = |a: f32, b: f32| {
            let ulp = f32::from_bits(a.to_bits() + 1) - a;
            (a - b) / ulp.max(f32::MIN_POSITIVE)
        };
        // Viewpoints that actually occur, each resolving detail at its own
        // viewing distance — the whole-system overhead shot, an Earth-framing
        // shot, and a close pass where 100 km must still be distinguishable.
        for (dist, sep) in [
            (360.0f32, 0.127f32), // 2.4 AU out, across the exaggerated Earth
            (1.0, 0.006_371),     // Earth framed, across its true diameter
            (0.05, 0.000_1),      // 50 000 km out, resolving 100 km
        ] {
            let near = (dist * 0.0015).clamp(1e-5, 0.5);
            let p = M4::perspective_reversed_z(45f32.to_radians(), 1.6, near, 1.0e6);
            let depth = |z: f32| {
                let o = transform(&p, v3(0.0, 0.0, -z), 1.0);
                o[2] / o[3]
            };
            let front = depth(dist);
            let back = depth(dist + sep);
            let ulps = ulps_between(front, back);
            assert!(front > back, "no separation at dist {dist}: {front} vs {back}");
            assert!(ulps > 64.0, "only {ulps} ULPs of separation at dist {dist}");
        }

        // And the case reversed-Z exists for: the Sun 1 AU beyond the focused
        // Earth must not collapse onto it.
        let p = M4::perspective_reversed_z(45f32.to_radians(), 1.6, 0.0015, 1.0e6);
        let d = |z: f32| {
            let o = transform(&p, v3(0.0, 0.0, -z), 1.0);
            o[2] / o[3]
        };
        assert!(d(1.0) > d(149.6) * 4.0, "Earth and Sun collapsed to one depth");
    }
}
