use core::f32::consts::PI;

use micromath::vector::F32x2;
macro_rules! new_kernal {
    ($struct:ident: $r:ident $(, $rs:ident)*) => {
        pub struct $struct {
                    $r: f32,
                    $(
                        $rs: f32,
                    )*
        }
        impl $struct {
            pub const fn new($r: f32) -> Self {
                new_kernal!(@internal $r, $r $(, $rs)*);
                Self {
                    $r,
                    $(
                        $rs,
                    )*
                }

            }
        }
    };
    (@internal $r:ident, $prev:ident, $next:ident $(, $rest:ident)*) => {
        let $next = $prev * $r;
        new_kernal!(@internal $r, $next $(, $rest)*)
    };
    (@internal $r:ident, $final:ident) => {}
}
//new_kernal!(SpikyKernel2: r, r2, r3, r4, r5);
new_kernal!(StdKernel2: _r, r2,_r3,r4);
pub trait Kernel {
    type Vect;
    fn eval(&self, distance: f32) -> f32;
    fn first_derivitive(&self, distance: f32) -> f32;
    fn gradient(&self, distance: f32, direction: Self::Vect) -> Self::Vect;
    fn second_derivitive(&self, distance: f32) -> f32;
}
//impl Kernel for SpikyKernel2 {
//    type Vect = F32x2;
//    #[inline]
//    fn eval(&self, distance: f32) -> f32 {
//        if distance >= self.r2 {
//            0f32
//        } else {
//            let x = 1f32 - distance / self.r2;
//            let x3 = x * x * x;
//            (10f32 * x3) / (PI * self.r2)
//        }
//    }
//    fn first_derivitive(&self, distance: f32) -> f32 {
//        0f32
//    }
//    fn gradient(&self, distance: f32, direction: Self::Vect) -> Self::Vect {
//
//    }
//}
impl Kernel for StdKernel2 {
    type Vect = F32x2;
    #[inline]
    fn eval(&self, distance: f32) -> f32 {
        let d = distance * distance;
        if d >= self.r2 {
            0f32
        } else {
            let x = 1f32 - d / self.r2;
            let x3 = x * x * x;
            (4f32 * x3) / (PI * self.r2)
        }
    }
    #[inline]
    fn first_derivitive(&self, distance: f32) -> f32 {
        let d = distance * distance;
        if d >= self.r2 {
            0f32
        } else {
            let x = 1f32 - d / self.r4;
            let x2 = x * x;
            (6f32 * x2) / (PI * self.r4) * distance * x2
        }
    }
    #[inline]
    fn gradient(&self, distance: f32, direction: F32x2) -> F32x2 {
        direction * self.first_derivitive(distance)
    }
    #[inline]
    fn second_derivitive(&self, distance: f32) -> f32 {
        let d = distance * distance;
        if d >= self.r2 {
            0f32
        } else {
            let x = 1f32 - d / self.r4;
            6f32 / (PI * self.r4) * (1f32 - x) * (3f32 * x - 1f32)
        }
    }
}
