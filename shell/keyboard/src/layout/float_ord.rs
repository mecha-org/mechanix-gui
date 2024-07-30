extern crate core;

use crate::layout::float_ord::core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use crate::layout::float_ord::core::hash::{Hash, Hasher};
use crate::layout::float_ord::core::mem::transmute;

#[derive(Clone, Copy, Debug)]
pub struct FloatOrd<T>(pub T);

macro_rules! float_ord_impl {
    ($f:ident, $i:ident, $n:expr) => {
        impl FloatOrd<$f> {
            fn convert(self) -> $i {
                let u = unsafe { transmute::<$f, $i>(self.0) };
                let bit = 1 << ($n - 1);
                if u & bit == 0 {
                    u | bit
                } else {
                    !u
                }
            }
        }
        impl PartialEq for FloatOrd<$f> {
            fn eq(&self, other: &Self) -> bool {
                self.convert() == other.convert()
            }
        }
        impl Eq for FloatOrd<$f> {}
        impl PartialOrd for FloatOrd<$f> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.convert().partial_cmp(&other.convert())
            }
        }
        impl Ord for FloatOrd<$f> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.convert().cmp(&other.convert())
            }
        }
        impl Hash for FloatOrd<$f> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.convert().hash(state);
            }
        }
    };
}

float_ord_impl!(f32, u32, 32);
float_ord_impl!(f64, u64, 64);

/// Sort a slice of floats.
///
/// # Allocation behavior
///
/// This routine uses a quicksort implementation that does not heap allocate.
///
/// # Example
///
/// ```
/// use rs::float_ord;
/// let mut v = [-5.0, 4.0, 1.0, -3.0, 2.0];
///
/// float_ord::sort(&mut v);
/// assert!(v == [-5.0, -3.0, 1.0, 2.0, 4.0]);
/// ```
#[allow(dead_code)]
pub fn sort<T>(v: &mut [T])
where
    FloatOrd<T>: Ord,
{
    let v_: &mut [FloatOrd<T>] = unsafe { transmute(v) };
    v_.sort_unstable();
}

#[cfg(test)]
mod tests {
    extern crate std;

    use self::std::collections::hash_map::DefaultHasher;
    use self::std::hash::{Hash, Hasher};
    use super::FloatOrd;

    #[test]
    fn test_ord() {
        assert!(FloatOrd(1.0f64) < FloatOrd(2.0f64));
        assert!(FloatOrd(2.0f32) > FloatOrd(1.0f32));
        assert!(FloatOrd(1.0f64) == FloatOrd(1.0f64));
        assert!(FloatOrd(1.0f32) == FloatOrd(1.0f32));
        assert!(FloatOrd(0.0f64) > FloatOrd(-0.0f64));
        assert!(FloatOrd(0.0f32) > FloatOrd(-0.0f32));
        assert!(
            FloatOrd(crate::layout::float_ord::core::f64::NAN)
                == FloatOrd(crate::layout::float_ord::core::f64::NAN)
        );
        assert!(
            FloatOrd(crate::layout::float_ord::core::f32::NAN)
                == FloatOrd(crate::layout::float_ord::core::f32::NAN)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f64::NAN)
                < FloatOrd(crate::layout::float_ord::core::f64::NAN)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f32::NAN)
                < FloatOrd(crate::layout::float_ord::core::f32::NAN)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f64::INFINITY)
                < FloatOrd(crate::layout::float_ord::core::f64::INFINITY)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f32::INFINITY)
                < FloatOrd(crate::layout::float_ord::core::f32::INFINITY)
        );
        assert!(
            FloatOrd(crate::layout::float_ord::core::f64::INFINITY)
                < FloatOrd(crate::layout::float_ord::core::f64::NAN)
        );
        assert!(
            FloatOrd(crate::layout::float_ord::core::f32::INFINITY)
                < FloatOrd(crate::layout::float_ord::core::f32::NAN)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f64::NAN)
                < FloatOrd(crate::layout::float_ord::core::f64::INFINITY)
        );
        assert!(
            FloatOrd(-crate::layout::float_ord::core::f32::NAN)
                < FloatOrd(crate::layout::float_ord::core::f32::INFINITY)
        );
    }

    fn hash<F: Hash>(f: F) -> u64 {
        let mut hasher = DefaultHasher::new();
        f.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_hash() {
        assert_ne!(hash(FloatOrd(0.0f64)), hash(FloatOrd(-0.0f64)));
        assert_ne!(hash(FloatOrd(0.0f32)), hash(FloatOrd(-0.0f32)));
        assert_eq!(hash(FloatOrd(-0.0f64)), hash(FloatOrd(-0.0f64)));
        assert_eq!(hash(FloatOrd(0.0f32)), hash(FloatOrd(0.0f32)));
        assert_ne!(
            hash(FloatOrd(crate::layout::float_ord::core::f64::NAN)),
            hash(FloatOrd(-crate::layout::float_ord::core::f64::NAN))
        );
        assert_ne!(
            hash(FloatOrd(crate::layout::float_ord::core::f32::NAN)),
            hash(FloatOrd(-crate::layout::float_ord::core::f32::NAN))
        );
        assert_eq!(
            hash(FloatOrd(crate::layout::float_ord::core::f64::NAN)),
            hash(FloatOrd(crate::layout::float_ord::core::f64::NAN))
        );
        assert_eq!(
            hash(FloatOrd(-crate::layout::float_ord::core::f32::NAN)),
            hash(FloatOrd(-crate::layout::float_ord::core::f32::NAN))
        );
    }

    #[test]
    fn test_sort_nan() {
        let nan = crate::layout::float_ord::core::f64::NAN;
        let mut v = [-1.0, 5.0, 0.0, -0.0, nan, 1.5, nan, 3.7];
        super::sort(&mut v);
        assert!(v[0] == -1.0);
        assert!(v[1] == 0.0 && v[1].is_sign_negative());
        assert!(v[2] == 0.0 && !v[2].is_sign_negative());
        assert!(v[3] == 1.5);
        assert!(v[4] == 3.7);
        assert!(v[5] == 5.0);
        assert!(v[6].is_nan());
        assert!(v[7].is_nan());
    }
}
