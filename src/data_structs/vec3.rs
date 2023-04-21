use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use image::Rgb;
use rand::Rng;

pub use Vec3 as Color;
pub use Vec3 as Point3;

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let point = Point3::random_with_limits(-1.0, 1.0);
        if point.length_squared() < 1.0 {
            return point;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}


#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn as_vector(self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn write_color(&self, samples_per_pixel: usize) -> Rgb<u8> {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        Rgb([
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        ])
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    pub fn random_with_limits(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_disk() -> Point3 {
        loop {
            let mut point = Vec3::random_with_limits(-1.0, 1.0);
            point.z = 0.0;

            if point.length_squared() >= 1.0 {
                continue;
            }

            return point;
        }
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;

        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
        vector - 2.0 * vector.dot(normal) * normal
    }

    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-uv.dot(normal), 1.0);
        let ray_out_perpendicular = etai_over_etat * (uv + cos_theta * normal);
        let ray_out_parallel = -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt() * normal;

        ray_out_perpendicular + ray_out_parallel
    }
}


// Macro rules from: https://github.com/ryankaplan/vec3/blob/master/src/lib.rs

// This macro helps us implement math operators on Vector3
// in such a way that it handles binary operators on any
// combination of Vec3, &Vec3 and f64.
macro_rules! impl_binary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Add`
  // $op_fn is something like `add`
  // $op_symbol is something like `+`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
    // Implement a + b where a and b are both of type &VectorType.
    // Lower down we'll implement cases where either a or b - or both
    // - are values by forwarding through to this implementation.
    impl<'a, 'b> $Operation<&'a $VectorType> for &'b $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        }
      }
    }

    // Implement a + b for the cases...
    //
    //   a: $VectorType,  b: &$VectorType
    //   a: &$VectorType, b: $VectorType
    //   a: $VectorType, b: $VectorType
    //
    // In each case we forward through to the implementation above.
    impl $Operation<$VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &self $op_symbol &other
      }
    }

    impl<'a> $Operation<&'a $VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl<'a> $Operation<$VectorType> for &'a $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        self $op_symbol &other
      }
    }

    // Implement a + b where a is type &$VectorType and b is type f64
    impl<'a> $Operation<f64> for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self, other: f64) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other,
          y: self.y $op_symbol other,
          z: self.z $op_symbol other
        }
      }
    }

    // Implement a + b where...
    //
    // a is $VectorType and b is f64
    // a is f64 and b is $VectorType
    // a is f64 and b is &$VectorType
    //
    // In each case we forward the logic to the implementation
    // above.
    impl $Operation<f64> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: f64) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl $Operation<$VectorType> for f64 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &other $op_symbol self
      }
    }

    impl<'a> $Operation<&'a $VectorType> for f64 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        other $op_symbol self
      }
    }
  };
}

// It also implements unary operators like - a where a is of
// type Vec3 or &Vec3.
macro_rules! impl_unary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Neg`
  // $op_fn is something like `neg`
  // $op_symbol is something like `-`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {

    // Implement the unary operator for references
    impl<'a> $Operation for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self) -> Vec3 {
        $VectorType {
          x: $op_symbol self.x,
          y: $op_symbol self.y,
          z: $op_symbol self.z,
        }
      }
    }

    // Have the operator on values forward through to the implementation
    // above
    impl $Operation for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self) -> Vec3 {
        $op_symbol &self
      }
    }
  };
}

// Implement add-assignment operators like a += b where a and
// b is either &Vec3 or Vec3 (in this case a is always of type
// &mut Vec3).
macro_rules! impl_op_assign {
  // $VectorType is something like `Vec3`
  // $OperationAssign is something like `AddAssign`
  // $op_fn is something like `add_assign`
  // $op_symbol is something like `+=`
  ($VectorType:ident $OperationAssign:ident $op_fn:ident $op_symbol:tt) => {
    // Implement $OperationAssign for RHS &Vec3
    impl<'a> $OperationAssign<&'a $VectorType> for $VectorType {
      fn $op_fn(&mut self, other: &'a $VectorType) {
        *self = $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        };
      }
    }

    // Implement $OperationAssign for RHS Vec3 by forwarding through to the
    // implementation above
    impl $OperationAssign for $VectorType {
      #[inline]
      fn $op_fn(&mut self, other: $VectorType) {
        *self = *self $op_symbol &other
      }
    }
  };
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item=Vec3>
    {
        let mut total = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        for vec in iter {
            total.x += vec.x;
            total.y += vec.y;
            total.z += vec.z;
        }

        total
    }
}



impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_unary_operations!(Vec3 Neg neg -);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);
