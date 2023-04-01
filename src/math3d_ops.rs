use num_traits::{Float, PrimInt};
use std::ops::{BitOr, BitXor, Add, Sub, Mul, Div, Neg, Not, BitAnd, Rem};

pub fn abs<T: Float>(value: T) -> T { T::abs(value) }
pub fn acos<T: Float>(value: T) -> T { T::acos(value) }
pub fn asin<T: Float>(value: T) -> T { T::asin(value) }
pub fn atan<T: Float>(value: T) -> T { T::atan(value) }
pub fn cos<T: Float>(value: T) -> T { T::cos(value) }
pub fn cosh<T: Float>(value: T) -> T { T::cosh(value) }
pub fn exp<T: Float>(value: T) -> T { T::exp(value) }
pub fn ln<T: Float>(value: T) -> T { T::ln(value) }
pub fn log10<T: Float>(value: T) -> T { T::log10(value) }
pub fn sin<T: Float>(value: T) -> T { T::sin(value) }
pub fn sinh<T: Float>(value: T) -> T { T::sinh(value) }
pub fn sqrt<T: Float>(value: T) -> T { T::sqrt(value) }
pub fn tan<T: Float>(value: T) -> T { T::tan(value) }
pub fn tanh<T: Float>(value: T) -> T { T::tanh(value) }

pub fn inverse<T: Float>(value: T) -> T { T::one() / value }
pub fn ceil<T: Float>(value: T) -> T { T::ceil(value) }
pub fn floor<T: Float>(value: T) -> T { T::floor(value) }
pub fn round<T: Float>(value: T) -> T { T::round(value) }
pub fn trunc<T: Float>(value: T) -> T { T::trunc(value) }
pub fn sqr<T: Mul<Output = T> + Copy>(value: T) -> T { value * value }
pub fn cube<T: Mul<Output = T> + Copy>(value: T) -> T {  value * value * value }
pub fn to_radians<T: Float>(value: T) -> T { T::to_radians(value) }
pub fn to_degrees<T: Float>(value: T) -> T { T::to_degrees(value) }

pub fn sign<T: Float>(value: T) -> T { T::signum(value) }
pub fn magnitude<T: Copy>(value: T) -> T { value }
pub fn magnitude_squared<T: Mul<Output = T> + Copy>(value: T) -> T { value.mul(value) }
pub fn distance<T: Float>(value: T, other: &T) -> T { (value - *other).abs() }
pub fn is_infinite<T: Float>(value: T) -> bool { T::is_infinite(value) }
pub fn is_nan<T: Float>(value: T) -> bool { T::is_nan(value) }
pub fn almost_equals<T: Float>(value: &T, other: &T, tolerance: &T) -> bool { almost_zero(&(*value - *other), tolerance) }
pub fn almost_zero<T: Float>(value: &T, tolerance: &T) -> bool { value.abs() < *tolerance }
pub fn smoothstep<T: Float>(value: T) -> T {
    let three: T = T::from(3.0).unwrap();
    let two: T = T::from(2.0).unwrap();
    value * value * (three - two * value)
}

pub fn within<T: PartialOrd<T>>(value: &T, min: &T, max: &T) -> bool { value >= min && value < max }
pub fn min<T: core::cmp::Ord>(value: T, other: T) -> T { value.min(other) }
pub fn max<T: core::cmp::Ord>(value: T, other: T) -> T { value.max(other) }

pub fn add<T: Add<Output=T>>(value: T, other: T) -> T { value.add(other) }
pub fn sub<T: Sub<Output=T>>(value: T, other: T) -> T { value.sub(other) }
pub fn mul<T: Mul<Output=T>>(value: T, other: T) -> T { value.mul(other) }
pub fn div<T: Div<Output=T>>(value: T, other: T) -> T { value.div(other) }
pub fn neg<T: Neg<Output=T>>(value: T) -> T { value.neg() }

pub fn gt<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.gt(other) }
pub fn lt<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.lt(other) }
pub fn ge<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.ge(other) }
pub fn le<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.le(other) }
pub fn eq<T: PartialEq<T>>(value: &T, other: &T) -> bool { value.eq(other) }
pub fn ne<T: PartialEq<T>>(value: &T, other: &T) -> bool { value.ne(other) }

pub fn and<T: BitAnd<Output = T>>(value: T, other: T) -> T { value.bitand(other) } 
pub fn or<T: BitOr<Output = T>>(value: T, other: T) -> T { value.bitor(other) } 
pub fn nand<T: BitAnd<Output = impl Not<Output = T>> + Not<Output = T>>(value: T, other: T) -> T { !value.bitand(other) } 
pub fn xor<T: BitXor<Output = T>>(value: T, other: T) -> T { value.bitxor(other) } 
pub fn nor<T: BitOr<Output = impl Not<Output = T>> + Not<Output = T>>(value: T, other: T) -> T { !value.bitor(other) } 
pub fn not<T: Not<Output = T>>(value: T) -> T { !value } 

pub fn div_round_up<T: PrimInt>(a: T, b: T) -> T { a / b + (if a % b > T::zero() { T::one() } else { T::zero() }) }
pub fn is_even<T: PrimInt>(n: T) -> bool { n % T::from(2).unwrap() == T::zero() }
pub fn is_odd<T: PrimInt>(n: T) -> bool { n % T::from(2).unwrap() == T::one() }
pub fn is_power_of_two<T: PrimInt>(v: T) -> bool { v > T::zero() && (v & (v - T::one())) == T::zero() }
//pub fn clamp<T: PrimInt>(v: T, min: T, max: T) -> T { v.min(max).max(min) }

pub fn lerp<T: Float>(v1: T, v2: T, t: T) -> T { v1 + (v2 - v1) * t }
pub fn inverse_lerp<T: Float>(v: T, a: T, b: T) -> T { (v - a) / (b - a) }
pub fn lerp_precise<T: Float>(v1: T, v2: T, t: T) -> T { ((T::one() - t) * v1) + (v2 * t) }
pub fn clamp_lower<T: Float>(v: T, min: T) -> T { v.max(min) }
pub fn clamp_upper<T: Float>(v: T, max: T) -> T { v.min(max) }
pub fn clamp<T: Float>(v: T, min: T, max: T) -> T { v.min(max).max(min) }
pub fn average<T: Float>(v1: T, v2: T) -> T { lerp(v1, v2, T::from(0.5f32).unwrap() ) }
pub fn barycentric<T: Float>(v1: T, v2: T, v3: T, u: T, v: T) -> T { v1 + (v2 - v1) * u + (v3 - v1) * v }
