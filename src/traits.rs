use core;
use num_traits::{Num, Bounded};
use num_integer::Integer;

pub trait GeneralFloat: Num {
    type Exponent: Integer;
    fn radix() -> Self;
    fn bit_size() -> Self::Exponent;
    fn epsilon() -> Self;
    fn radix_powi(p: Self::Exponent) -> Self;
}

pub trait BinaryFloat: Num {
    type Exponent: Integer;
    fn bit_size() -> Self::Exponent;
    fn epsilon() -> Self;
    fn radix_powi(p: Self::Exponent) -> Self;
}

impl<T: BinaryFloat> GeneralFloat for T {
    type Exponent = T::Exponent;
    fn radix() -> T {
        T::one() + T::one()
    }
    fn bit_size() -> T::Exponent {
        T::bit_size()
    }
    fn epsilon() -> T {
        T::epsilon()
    }
    fn radix_powi(p: T::Exponent) -> T {
        T::radix_powi(p)
    }
}

pub trait BoundedFloat: Bounded + GeneralFloat {
    fn max_exponent() -> Self::Exponent;
    fn min_exponent() -> Self::Exponent;
    fn min_positive() -> Self;
}

pub trait Overflow: Num {
    fn overflow() -> Self;
    fn neg_overflow() -> Self;
}

pub trait Infinite: Num {
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
}

impl<T: Infinite> Overflow for T {
    fn overflow() -> Self {
        T::infinity()
    }
    fn neg_overflow() -> Self {
        T::neg_infinity()
    }
}

pub trait Underflow: Num {
    fn unit_underflow() -> Self;
}

pub trait IEEE754Float: BinaryFloat + BoundedFloat + Infinite + Underflow {
    fn nan() -> Self;
}

macro_rules! impl_fxx {
    ($fxx:ident, $expo:ty, [$emin:expr,$emax:expr], $bits:expr) => (
        impl BinaryFloat for $fxx {
            type Exponent = $expo;
            fn bit_size() -> Self::Exponent{
                $bits
            }
            fn epsilon() -> Self{
                core::$fxx::EPSILON
            }
            fn radix_powi(p: Self::Exponent) -> Self{
                $fxx::radix().powi(p)
            }
        }
        impl BoundedFloat for $fxx {
            fn max_exponent() -> Self::Exponent{
                $emax
            }
            fn min_exponent() -> Self::Exponent{
                $emin
            }
            fn min_positive() -> Self{
                core::$fxx::MIN_POSITIVE
            }
        }
        impl Infinite for $fxx {
            fn infinity() -> Self{
                core::$fxx::INFINITY
            }
            fn neg_infinity() -> Self{
                core::$fxx::NEG_INFINITY
            }
        }
        impl Underflow for $fxx {
            fn unit_underflow() -> Self{
                core::$fxx::MIN_POSITIVE * core::$fxx::EPSILON
            }
        }
        impl IEEE754Float for $fxx {
            fn nan() -> Self {
                core::$fxx::NAN
            }
        }
    )
}

impl_fxx!(f32, i32, [-126, 127], 24);
impl_fxx!(f64, i32, [-1022, 1023], 53);
