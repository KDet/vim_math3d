use std::{ops::{BitAnd, Add, Not}, process::Output};

pub fn abs<T: num_traits::Float>(value: T) -> T { T::abs(value) }
pub fn acos<T: num_traits::Float>(value: T) -> T { T::acos(value) }
pub fn asin<T: num_traits::Float>(value: T) -> T { T::asin(value) }
pub fn atan<T: num_traits::Float>(value: T) -> T { T::atan(value) }
pub fn cos<T: num_traits::Float>(value: T) -> T { T::cos(value) }
pub fn cosh<T: num_traits::Float>(value: T) -> T { T::cosh(value) }
pub fn exp<T: num_traits::Float>(value: T) -> T { T::exp(value) }
pub fn ln<T: num_traits::Float>(value: T) -> T { T::ln(value) }
pub fn log10<T: num_traits::Float>(value: T) -> T { T::log10(value) }
pub fn sin<T: num_traits::Float>(value: T) -> T { T::sin(value) }
pub fn sinh<T: num_traits::Float>(value: T) -> T { T::sinh(value) }
pub fn sqrt<T: num_traits::Float>(value: T) -> T { T::sqrt(value) }
pub fn tan<T: num_traits::Float>(value: T) -> T { T::tan(value) }
pub fn tanh<T: num_traits::Float>(value: T) -> T { T::tanh(value) }

pub fn inverse<T: num_traits::Float>(value: T) -> T { T::one() / value }
pub fn ceil<T: num_traits::Float>(value: T) -> T { T::ceil(value) }
pub fn floor<T: num_traits::Float>(value: T) -> T { T::floor(value) }
pub fn round<T: num_traits::Float>(value: T) -> T { T::round(value) }
pub fn trunc<T: num_traits::Float>(value: T) -> T { T::trunc(value) }
pub fn sqr<T: std::ops::Mul<Output = T> + Copy>(value: T) -> T { value * value }
pub fn cube<T: std::ops::Mul<Output = T> + Copy>(value: T) -> T {  value * value * value }
pub fn to_radians<T: num_traits::Float>(value: T) -> T { T::to_radians(value) }
pub fn to_degrees<T: num_traits::Float>(value: T) -> T { T::to_degrees(value) }

pub fn sign<T: num_traits::Float>(value: T) -> T { T::signum(value) }
pub fn magnitude<T: num_traits::Float>(value: T) -> T { value }
pub fn magnitude_squared<T: num_traits::Float>(value: T) -> T { value * value }
pub fn distance<T: num_traits::Float>(value: T, other: &T) -> T { (value - *other).abs() }
pub fn is_infinite<T: num_traits::Float>(value: T) -> bool { T::is_infinite(value) }
pub fn is_nan<T: num_traits::Float>(value: T) -> bool { T::is_nan(value) }
pub fn almost_equals<T: num_traits::Float>(value: &T, other: &T, tolerance: &T) -> bool { almost_zero(&(*value - *other), tolerance) }
pub fn almost_zero<T: num_traits::Float>(value: &T, tolerance: &T) -> bool { value.abs() < *tolerance }
pub fn smoothstep<T: num_traits::Float>(value: T) -> T {
    let three: T = T::from(3.0).unwrap();
    let two: T = T::from(2.0).unwrap();
    value * value * (three - two * value)
}

pub fn within<T: PartialOrd<T>>(value: &T, min: &T, max: &T) -> bool { value >= min && value < max }
pub fn min<T: core::cmp::Ord>(value: T, other: T) -> T { value.min(other) }
pub fn max<T: core::cmp::Ord>(value: T, other: T) -> T { value.max(other) }

pub fn add<T: std::ops::Add<Output=T>>(value: T, other: T) -> T { value.add(other) }
pub fn sub<T: std::ops::Sub<Output=T>>(value: T, other: T) -> T { value.sub(other) }
pub fn mul<T: std::ops::Mul<Output=T>>(value: T, other: T) -> T { value.mul(other) }
pub fn div<T: std::ops::Div<Output=T>>(value: T, other: T) -> T { value.div(other) }
pub fn neg<T: std::ops::Neg<Output=T>>(value: T) -> T { value.neg() }

pub fn gt<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.gt(other) }
pub fn lt<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.lt(other) }
pub fn ge<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.ge(other) }
pub fn le<T: PartialOrd<T>>(value: &T, other: &T) -> bool { value.le(other) }
pub fn eq<T: PartialEq<T>>(value: &T, other: &T) -> bool { value.eq(other) }
pub fn ne<T: PartialEq<T>>(value: &T, other: &T) -> bool { value.ne(other) }

pub fn not<T: Not<Output = T>>(value: T) -> T { !value } 

pub fn bitadd<T: std::ops::BitAnd<Output = T>>(value: T, other: T) -> T { value.bitand(other) } 
pub fn bitor<T: std::ops::BitOr<Output = T>>(value: T, other: T) -> T { value.bitor(other) } 
pub fn bitnadd<T: std::ops::BitAnd<Output = impl Not<Output = T>> + Not<Output = T>>(value: T, other: T) -> T { !value.bitand(other) } 
pub fn bitxor<T: std::ops::BitXor<Output = T>>(value: T, other: T) -> T { value.bitxor(other) } 
pub fn bitnor<T: std::ops::BitOr<Output = impl Not<Output = T>> + Not<Output = T>>(value: T, other: T) -> T { !value.bitor(other) } 

// foreach (var t in intTypes) {
//     #>
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> Magnitude(this <#= t #> x) => x;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> MagnitudeSquared(this <#= t #> x) => x * x;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> And (this <#= t #> a, <#= t #> b) => a & b;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> Or (this <#= t #> a, <#= t #> b) => a | b;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> NAnd (this <#= t #> a, <#= t #> b) => ~(a & b);
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> XOr (this <#= t #> a, <#= t #> b) => a | b & ~(a & b);
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> NOr (this <#= t #> a, <#= t #> b) => ~(a | b);
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> Not (this <#= t #> a) => ~a;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> Abs (this <#= t #> a) => Math.Abs(a);
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> DivideRoundUp (this <#= t #> a, <#= t #> b) => a / b + (a % b > 0 ? 1 : 0);
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static bool IsEven (this <#= t #> n) => n % 2 == 0;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static bool IsOdd (this <#= t #> n) => n % 2 == 1;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static bool IsPowerOfTwo (this <#= t #> v) => v > 0 && (v & (v - 1)) == 0;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static bool IsInfinity (this <#= t #> n) => false;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static bool IsNaN (this <#= t #> n) => false;
//             [MethodImpl(MethodImplOptions.AggressiveInlining)] public static <#= t #> Clamp (this <#= t #> v, <#= t #> min, <#= t #> max) => v.Min(max).Max(min);
//     <# } #>