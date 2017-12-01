use core;
use core::ops::Neg;
use core::cmp::PartialOrd;
use core::clone::Clone;
use num_traits::{Num, Bounded};
use num_integer::Integer;

pub trait Abs {
    type Output: Num;
    fn abs(self) -> Self::Output;
}

macro_rules! impl_abs {
    ($($num:ty)*) => ($(
        impl Abs for $num {
            type Output = $num;
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
        }
        impl<'a> Abs for &'a $num {
            type Output = $num;
            #[inline]
            fn abs(self) -> $num {
                self.clone().abs()
            }
        }
    )*)
}

impl_abs!(f32 f64);

pub trait Sqrt {
    type Output: Num;
    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_sqrt {
    ($($num:ty)*) => ($(
        impl Sqrt for $num {
            type Output = $num;
            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
        }
        impl<'a> Sqrt for &'a $num {
            type Output = $num;
            #[inline]
            fn sqrt(self) -> $num {
                self.clone().sqrt()
            }
        }
    )*)
}

impl_sqrt!(f32 f64);

pub trait GeneralFloat: Num + PartialOrd {
    type Exponent: Integer;
    fn radix() -> Self;
    fn bit_size() -> Self::Exponent;
    fn epsilon() -> Self;
    fn radix_powi(p: Self::Exponent) -> Self;
}

pub trait BinaryFloat: Num + PartialOrd {
    type Expo: Integer;
    fn bits() -> Self::Expo;
    fn eps() -> Self;
    fn two_powi(p: Self::Expo) -> Self;
}

impl<T: BinaryFloat> GeneralFloat for T {
    type Exponent = T::Expo;
    #[inline]
    fn radix() -> T {
        T::one() + T::one()
    }
    #[inline]
    fn bit_size() -> T::Expo {
        T::bits()
    }
    #[inline]
    fn epsilon() -> T {
        T::eps()
    }
    #[inline]
    fn radix_powi(p: T::Expo) -> T {
        T::two_powi(p)
    }
}

pub trait BoundedFloat: Bounded + GeneralFloat {
    fn max_exponent() -> Self::Exponent;
    fn min_exponent() -> Self::Exponent;
}

pub trait Overflow: Num + PartialOrd {
    fn overflow() -> Self;
    fn neg_overflow() -> Self;
}

pub trait Infinite: Num + PartialOrd {
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_infinite(&self) -> bool;
}

impl<T: Infinite> Overflow for T {
    #[inline]
    fn overflow() -> Self {
        T::infinity()
    }
    #[inline]
    fn neg_overflow() -> Self {
        T::neg_infinity()
    }
}

pub trait Underflow: Num {
    fn min_positive() -> Self;
    fn unit_underflow() -> Self;
}

pub trait IEEE754Float
    : Abs<Output=Self> + Sqrt<Output = Self> + Neg<Output = Self> +
    BinaryFloat + BoundedFloat + Infinite + Underflow
    {
    fn nan() -> Self;
}

macro_rules! impl_fxx {
    ($fxx:ident, $expo:ty, [$emin:expr,$emax:expr], $bits:expr) => (
        impl BinaryFloat for $fxx {
            type Expo = $expo;
            #[inline]
            fn bits() -> Self::Expo{
                $bits
            }
            #[inline]
            fn eps() -> Self{
                core::$fxx::EPSILON
            }
            #[inline]
            fn two_powi(p: Self::Expo) -> Self{
                $fxx::radix().powi(p)
            }
        }
        impl BoundedFloat for $fxx {
            #[inline]
            fn max_exponent() -> Self::Exponent{
                $emax
            }
            #[inline]
            fn min_exponent() -> Self::Exponent{
                $emin
            }
        }
        impl Infinite for $fxx {
            #[inline]
            fn infinity() -> Self{
                core::$fxx::INFINITY
            }
            #[inline]
            fn neg_infinity() -> Self{
                core::$fxx::NEG_INFINITY
            }
            #[inline]
            fn is_infinite(&self) -> bool{
                $fxx::is_infinite(*self)
            }
        }
        impl Underflow for $fxx {
            #[inline]
            fn min_positive() -> Self{
                core::$fxx::MIN_POSITIVE
            }
            #[inline]
            fn unit_underflow() -> Self{
                core::$fxx::MIN_POSITIVE * core::$fxx::EPSILON
            }
        }
        impl IEEE754Float for $fxx {
            #[inline]
            fn nan() -> Self {
                core::$fxx::NAN
            }
        }
    )
}

impl_fxx!(f32, i32, [-126, 127], 24);
impl_fxx!(f64, i32, [-1022, 1023], 53);

mod tests {
    #[test]
    fn f64abs() {
        use super::Abs;
        assert!((-1.0).abs() == <&f64 as Abs>::abs(&-1.0f64));
        assert!((-1.0).abs() == <f64 as Abs>::abs(-1.0f64));
    }
}
