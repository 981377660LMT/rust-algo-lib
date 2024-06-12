// pub mod num_number {
//     use std::fmt::Debug;
//     use std::fmt::Display;
//     use std::ops::Add;
//     use std::ops::Div;
//     use std::ops::Mul;
//     use std::ops::Sub;

//     pub trait Number:
//         Copy
//         + Add<Output = Self>
//         + Sub<Output = Self>
//         + Mul<Output = Self>
//         + Div<Output = Self>
//         + PartialEq
//         + PartialOrd
//         + Display
//         + Debug
//     {
//         const MAX: Self;
//         const MIN: Self;
//         const ZERO: Self;
//         const ONE: Self;

//         type SignedType;

//         fn as_i8(&self) -> i8;
//         fn as_i16(&self) -> i16;
//         fn as_i32(&self) -> i32;
//         fn as_i64(&self) -> i64;
//         fn as_i128(&self) -> i128;
//         fn as_u8(&self) -> u8;
//         fn as_u16(&self) -> u16;
//         fn as_u32(&self) -> u32;
//         fn as_u64(&self) -> u64;
//         fn as_u128(&self) -> u128;
//         fn as_f32(&self) -> f32;
//         fn as_f64(&self) -> f64;
//         fn as_isize(&self) -> isize;
//         fn as_usize(&self) -> usize;
//         fn from_i8(x: i8) -> Self;
//         fn from_i16(x: i16) -> Self;
//         fn from_i32(x: i32) -> Self;
//         fn from_i64(x: i64) -> Self;
//         fn from_i128(x: i128) -> Self;
//         fn from_isize(x: isize) -> Self;
//         fn from_u8(x: u8) -> Self;
//         fn from_u16(x: u16) -> Self;
//         fn from_u32(x: u32) -> Self;
//         fn from_u64(x: u64) -> Self;
//         fn from_u128(x: u128) -> Self;
//         fn from_usize(x: usize) -> Self;
//         fn from_f64(x: f64) -> Self;
//         fn from_f32(x: f32) -> Self;
//         fn sign(&self) -> i8 {
//             if self.is_negative() {
//                 -1
//             } else if self.is_positive() {
//                 1
//             } else {
//                 0
//             }
//         }
//         fn negative(&self) -> Self {
//             Self::ZERO - *self
//         }
//         fn is_negative(&self) -> bool {
//             *self < Self::ZERO
//         }
//         fn is_positive(&self) -> bool {
//             *self > Self::ZERO
//         }
//         fn is_non_negative(&self) -> bool {
//             *self >= Self::ZERO
//         }
//         fn is_non_positive(&self) -> bool {
//             *self <= Self::ZERO
//         }
//         fn absolute(&self) -> Self {
//             if self.is_negative() {
//                 self.negative()
//             } else {
//                 *self
//             }
//         }
//         fn as_signed(&self) -> Self::SignedType;
//     }

//     macro_rules! Generator {
//         ($t: ty) => {
//             Generator!($t, $t);
//         };
//         ($t: ty, $s: ty) => {
//             impl Number for $t {
//                 type SignedType = $s;

//                 const MAX: Self = <$t>::MAX;
//                 const MIN: Self = <$t>::MIN;
//                 const ZERO: Self = 0 as Self;
//                 const ONE: Self = 1 as Self;

//                 fn as_i8(&self) -> i8 {
//                     *self as i8
//                 }

//                 fn as_i16(&self) -> i16 {
//                     *self as i16
//                 }

//                 fn as_i32(&self) -> i32 {
//                     *self as i32
//                 }

//                 fn as_i64(&self) -> i64 {
//                     *self as i64
//                 }

//                 fn as_i128(&self) -> i128 {
//                     *self as i128
//                 }

//                 fn as_f32(&self) -> f32 {
//                     *self as f32
//                 }

//                 fn as_f64(&self) -> f64 {
//                     *self as f64
//                 }

//                 fn as_isize(&self) -> isize {
//                     *self as isize
//                 }

//                 fn absolute(&self) -> Self {
//                     if self.is_negative() {
//                         self.negative()
//                     } else {
//                         *self
//                     }
//                 }

//                 fn from_i8(x: i8) -> Self {
//                     x as Self
//                 }

//                 fn from_i16(x: i16) -> Self {
//                     x as Self
//                 }

//                 fn from_i32(x: i32) -> Self {
//                     x as Self
//                 }

//                 fn from_i64(x: i64) -> Self {
//                     x as Self
//                 }

//                 fn from_i128(x: i128) -> Self {
//                     x as Self
//                 }

//                 fn from_isize(x: isize) -> Self {
//                     x as Self
//                 }

//                 fn from_f64(x: f64) -> Self {
//                     x as Self
//                 }

//                 fn from_f32(x: f32) -> Self {
//                     x as Self
//                 }

//                 fn as_u8(&self) -> u8 {
//                     *self as u8
//                 }

//                 fn as_u16(&self) -> u16 {
//                     *self as u16
//                 }

//                 fn as_u32(&self) -> u32 {
//                     *self as u32
//                 }

//                 fn as_u64(&self) -> u64 {
//                     *self as u64
//                 }

//                 fn as_u128(&self) -> u128 {
//                     *self as u128
//                 }

//                 fn as_usize(&self) -> usize {
//                     *self as usize
//                 }

//                 fn from_u8(x: u8) -> Self {
//                     x as Self
//                 }

//                 fn from_u16(x: u16) -> Self {
//                     x as Self
//                 }

//                 fn from_u32(x: u32) -> Self {
//                     x as Self
//                 }

//                 fn from_u64(x: u64) -> Self {
//                     x as Self
//                 }

//                 fn from_u128(x: u128) -> Self {
//                     x as Self
//                 }

//                 fn from_usize(x: usize) -> Self {
//                     x as Self
//                 }

//                 fn as_signed(&self) -> Self::SignedType {
//                     *self as Self::SignedType
//                 }
//             }
//         };
//     }

//     Generator!(usize);
//     Generator!(isize);

//     Generator!(i8);
//     Generator!(i16);
//     Generator!(i32);
//     Generator!(i64);
//     Generator!(i128);

//     Generator!(u8, i8);
//     Generator!(u16, i16);
//     Generator!(u32, i32);
//     Generator!(u64, i64);
//     Generator!(u128, i128);

//     Generator!(f32);
//     Generator!(f64);
// }

// pub mod num_real {
//     use crate::num_number::Number;

//     pub trait Real: Number {
//         fn average(a: Self, b: Self) -> Self {
//             (a + b) / Self::from_i8(2)
//         }
//     }
// }

// pub mod arithmetic {
//     use crate::num_number::Number;
//     use crate::num_real::Real;
//     use std::fmt::Debug;
//     use std::ops::Add;
//     use std::ops::Div;
//     use std::ops::Mul;
//     use std::ops::Sub;

//     pub trait CommutativeAdd: Add<Output = Self> + Copy + Debug {}
//     pub trait AssociativeAdd: Add<Output = Self> + Copy + Debug {}
//     pub trait IdentityAdd: Add<Output = Self> + Copy + Debug {
//         const ZERO: Self;
//     }
//     pub trait CommutativeMul: Mul<Output = Self> + Copy + Debug {}
//     pub trait AssociativeMul: Mul<Output = Self> + Copy + Debug {}
//     pub trait IdentityMul: Mul<Output = Self> + Copy + Debug {
//         const ONE: Self;
//     }
//     pub trait IdempotentAdd: CommutativeAdd + AssociativeAdd {}
//     pub trait IdempotentMul: CommutativeMul + AssociativeMul {}
//     pub trait IntegralMul: Mul<Output = Self> + Copy + Debug {}
//     macro_rules! AddGenerator {
//         ($t: ty, $zero: expr) => {
//             impl CommutativeAdd for $t {}
//             impl IdentityAdd for $t {
//                 const ZERO: Self = $zero;
//             }
//             impl IdempotentAdd for $t {}
//             impl AssociativeAdd for $t {}
//         };
//     }

//     macro_rules! MulGenerator {
//         ($t: ty, $one: expr) => {
//             impl CommutativeMul for $t {}
//             impl IdentityMul for $t {
//                 const ONE: Self = $one;
//             }
//             impl IdempotentMul for $t {}
//             impl AssociativeMul for $t {}
//             impl IntegralMul for $t {}
//         };
//     }

//     macro_rules! AllGenerator {
//         ($t: ty, $zero: expr, $one: expr) => {
//             AddGenerator!($t, $zero);
//             MulGenerator!($t, $one);
//         };
//     }

//     impl<T> CommutativeAdd for T where T: Number {}
//     impl<T> IdentityAdd for T
//     where
//         T: Number,
//     {
//         const ZERO: Self = <T as Number>::ZERO;
//     }
//     impl<T> AssociativeAdd for T where T: Number {}
//     impl<T> CommutativeMul for T where T: Number {}
//     impl<T> IdentityMul for T
//     where
//         T: Number,
//     {
//         const ONE: Self = <T as Number>::ONE;
//     }
//     impl<T> AssociativeMul for T where T: Number {}
//     impl<T> IntegralMul for T where T: Real {}

//     #[derive(Clone, Copy, Debug)]
//     pub struct Nil;
//     impl Mul for Nil {
//         type Output = Nil;

//         fn mul(self, rhs: Nil) -> Self::Output {
//             Nil
//         }
//     }
//     impl Add for Nil {
//         type Output = Nil;

//         fn add(self, rhs: Nil) -> Self::Output {
//             Nil
//         }
//     }
//     impl Sub for Nil {
//         type Output = Nil;

//         fn sub(self, rhs: Nil) -> Self::Output {
//             Nil
//         }
//     }

//     impl Div for Nil {
//         type Output = Nil;

//         fn div(self, rhs: Nil) -> Self::Output {
//             Nil
//         }
//     }
//     impl PartialEq for Nil {
//         fn eq(&self, other: &Self) -> bool {
//             true
//         }
//     }
//     impl Eq for Nil {}
//     AllGenerator!(Nil, Nil, Nil);
//     pub(crate) use AddGenerator;
//     pub(crate) use AllGenerator;
//     pub(crate) use MulGenerator;
// }
// pub mod num_integer {
//     use crate::num_number::Number;
//     use std::ops::BitAnd;
//     use std::ops::BitOr;
//     use std::ops::BitXor;
//     use std::ops::Not;
//     use std::ops::Rem;
//     use std::ops::Shl;
//     use std::ops::Shr;

//     pub trait Integer:
//         Number
//         + Rem<Output = Self>
//         + Shl<Output = Self>
//         + Shr<Output = Self>
//         + BitAnd<Output = Self>
//         + BitOr<Output = Self>
//         + BitXor<Output = Self>
//         + Not<Output = Self>
//     {
//         type HighPrecisionType;
//         type UnsignedType;
//         const BITS: i32;

//         fn div_floor(mut a: Self, mut b: Self) -> Self {
//             let mut r = a / b;
//             if r * b > a {
//                 if b > Self::ZERO {
//                     r = r - Self::ONE;
//                 } else {
//                     r = r + Self::ONE;
//                 }
//             }
//             r
//         }
//         fn div_ceil(a: Self, b: Self) -> Self {
//             let mut r = a / b;
//             if r * b < a {
//                 if b > Self::ZERO {
//                     r = r + Self::ONE;
//                 } else {
//                     r = r - Self::ONE;
//                 }
//             }
//             r
//         }
//         fn bit_count(&self) -> i32;
//         fn higest_set_bit_offset(&self) -> i32;
//         fn lowest_set_bit(&self) -> Self;
//         fn higest_one_bit(&self) -> Self;
//         fn count_leading_zero(&self) -> i32;
//         fn count_trailing_zero(&self) -> i32;
//         fn modular(a: Self, m: Self) -> Self {
//             let res = a % m;
//             if res.is_negative() {
//                 res + m
//             } else {
//                 res
//             }
//         }
//         fn mul_mod(a: Self, b: Self, m: Self) -> Self;
//         fn pow_mod(a: Self, n: u64, m: Self) -> Self {
//             if n == 0 {
//                 Self::ONE
//             } else {
//                 let ans = Self::pow_mod(a, n >> 1, m);
//                 let ans = Self::mul_mod(ans, ans, m);
//                 if (n & 1) == 1 {
//                     Self::mul_mod(ans, a, m)
//                 } else {
//                     ans
//                 }
//             }
//         }
//         fn pow(a: Self, mut n: u32) -> Self {
//             let mut ans = a;
//             while n > 1 {
//                 n -= 1u32;
//                 ans = ans * a;
//             }
//             ans
//         }
//         fn average_floor(a: Self, b: Self) -> Self {
//             (a & b) + ((a ^ b) >> Self::from_i8(1))
//         }
//         fn average_ceil(a: Self, b: Self) -> Self {
//             (a | b) - ((a ^ b) >> Self::from_i8(1))
//         }
//         fn add_overflow(a: Self, b: Self) -> (Self, bool);
//         fn add_or_default(a: Self, b: Self, def: Self) -> (Self, bool) {
//             let mut res = Self::add_overflow(a, b);
//             if res.1 {
//                 res.0 = def;
//             }
//             res
//         }
//         fn mul_or_default(a: Self, b: Self, def: Self) -> (Self, bool) {
//             let mut res = Self::mul_overflow(a, b);
//             if res.1 {
//                 res.0 = def;
//             }
//             res
//         }
//         fn bit_left_shift(&self, step: i32) -> Self {
//             if step >= Self::BITS {
//                 Self::ZERO
//             } else {
//                 *self << Self::from_i32(step)
//             }
//         }
//         fn bit_signed_right_shift(&self, step: i32) -> Self;
//         fn bit_unsigned_right_shift(&self, step: i32) -> Self;

//         fn mul_overflow(a: Self, b: Self) -> (Self, bool);
//         fn div_and_remainder(a: Self, b: Self) -> (Self, Self);
//     }

//     macro_rules! Generator {
//         ($t: ty, $h: ty, $u: ty) => {
//             impl Integer for $t {
//                 type HighPrecisionType = $h;
//                 type UnsignedType = $u;
//                 const BITS: i32 = <$t>::BITS as i32;
//                 fn count_trailing_zero(&self) -> i32 {
//                     let x = 0;
//                     if *self == <$t as Number>::ZERO {
//                         <Self as Integer>::BITS
//                     } else {
//                         <Self as Integer>::BITS - 1 - self.lowest_set_bit().count_leading_zero()
//                     }
//                 }
//                 fn bit_signed_right_shift(&self, step: i32) -> Self {
//                     if step >= <Self as Integer>::BITS {
//                         if self.is_negative() {
//                             !Self::ZERO
//                         } else {
//                             Self::ZERO
//                         }
//                     } else {
//                         ((*self as Self::SignedType) >> (step as Self::SignedType)) as Self
//                     }
//                 }

//                 fn bit_unsigned_right_shift(&self, step: i32) -> Self {
//                     if step >= <Self as Integer>::BITS {
//                         Self::ZERO
//                     } else {
//                         ((*self as Self::UnsignedType) >> (step as Self::UnsignedType)) as Self
//                     }
//                 }

//                 fn mul_mod(a: Self, b: Self, m: Self) -> Self {
//                     let mut res = ((a as Self::HighPrecisionType * b as Self::HighPrecisionType)
//                         % (m as Self::HighPrecisionType)) as Self;
//                     if res.is_negative() {
//                         res = res + m;
//                     }
//                     res
//                 }

//                 fn add_overflow(a: Self, b: Self) -> (Self, bool) {
//                     Self::overflowing_add(a, b)
//                 }

//                 fn mul_overflow(a: Self, b: Self) -> (Self, bool) {
//                     Self::overflowing_mul(a, b)
//                 }

//                 fn bit_count(&self) -> i32 {
//                     self.count_ones() as i32
//                 }

//                 fn higest_set_bit_offset(&self) -> i32 {
//                     (Self::BITS - 1 - self.leading_zeros()) as i32
//                 }

//                 fn lowest_set_bit(&self) -> Self {
//                     *self & self.negative()
//                 }

//                 fn higest_one_bit(&self) -> Self {
//                     if *self == Self::ZERO {
//                         0
//                     } else {
//                         Self::ONE << Self::from_i32(self.higest_set_bit_offset())
//                     }
//                 }

//                 fn count_leading_zero(&self) -> i32 {
//                     self.leading_zeros() as i32
//                 }

//                 fn div_and_remainder(a: Self, b: Self) -> (Self, Self) {
//                     let d = a / b;
//                     (d, a - d * b)
//                 }
//             }
//         };
//     }

//     Generator!(i8, i16, u8);
//     Generator!(u8, i16, u8);
//     Generator!(i16, i32, u16);
//     Generator!(u16, u32, u16);
//     Generator!(i32, i64, u32);
//     Generator!(u32, u64, u32);
//     Generator!(isize, isize, usize);
//     Generator!(usize, usize, usize);
//     Generator!(i64, i128, u64);
//     Generator!(u64, u128, u64);
//     Generator!(i128, i128, u128);
//     Generator!(u128, u128, u128);
// }

// pub mod binary {
//     use crate::num_integer::Integer;
//     use crate::util::should;

//     pub fn lowest_k_one<T>(k: T) -> T
//     where
//         T: Integer,
//     {
//         should!(k <= T::from_i32(T::BITS));
//         T::from_i32(-1).bit_unsigned_right_shift(T::BITS - k.as_i32())
//     }

//     pub fn highest_k_one<T>(k: T) -> T
//     where
//         T: Integer,
//     {
//         should!(k <= T::from_i32(T::BITS));
//         T::from_i32(-1).bit_left_shift(T::BITS - k.as_i32())
//     }
// }

// pub mod external_addition {
//     use crate::arithmetic::AssociativeAdd;
//     use crate::arithmetic::CommutativeAdd;
//     use crate::arithmetic::IdempotentAdd;
//     use std::cmp::max;
//     use std::cmp::min;
//     use std::fmt::Debug;
//     use std::marker::PhantomData;
//     use std::ops::Add;

//     pub trait Addition<T>: Copy + Debug
//     where
//         T: Copy + Debug,
//     {
//         fn add(a: T, b: T) -> T;
//     }
//     #[derive(Clone, Copy, Debug)]
//     pub struct Wrapper<T, F>
//     where
//         T: Copy + Debug,
//         F: Addition<T>,
//     {
//         v: T,
//         f: PhantomData<F>,
//     }
//     impl<T, F> Wrapper<T, F>
//     where
//         T: Copy + Debug,
//         F: Addition<T>,
//     {
//         pub fn new(v: T) -> Self {
//             Self { v, f: PhantomData }
//         }
//         pub fn value_ref<'a>(&'a self) -> &'a T {
//             &self.v
//         }
//         pub fn value_mut_ref<'a>(&'a mut self) -> &'a mut T {
//             &mut self.v
//         }
//         pub fn value(&self) -> T {
//             self.v
//         }
//     }

//     impl<T, F> Add for Wrapper<T, F>
//     where
//         T: Copy + Debug,
//         F: Addition<T>,
//     {
//         type Output = Self;
//         fn add(self, rhs: Self) -> Self::Output {
//             Wrapper::new(F::add(self.v, rhs.v))
//         }
//     }

//     #[derive(Clone, Copy, Debug)]
//     pub struct MinimumAddition<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         phantom: PhantomData<T>,
//     }

//     impl<T> Addition<T> for MinimumAddition<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         fn add(a: T, b: T) -> T {
//             min(a, b)
//         }
//     }
//     impl<T> IdempotentAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}
//     impl<T> CommutativeAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}
//     impl<T> AssociativeAdd for Wrapper<T, MinimumAddition<T>> where T: Ord + Copy + Debug {}

//     #[derive(Clone, Copy, Debug)]
//     pub struct MaximumAddition<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         phantom: PhantomData<T>,
//     }
//     impl<T> Addition<T> for MaximumAddition<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         fn add(a: T, b: T) -> T {
//             max(a, b)
//         }
//     }
//     impl<T> IdempotentAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
//     impl<T> CommutativeAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
//     impl<T> AssociativeAdd for Wrapper<T, MaximumAddition<T>> where T: Ord + Copy + Debug {}
// }
// pub mod algebraic_structure {
//     use crate::arithmetic::*;
//     use std::ops::Add;
//     use std::ops::Div;
//     use std::ops::Mul;
//     use std::ops::Sub;

//     pub trait Magma: Add<Output = Self> + Copy + PartialEq {}
//     impl<T> Magma for T where T: Add<Output = Self> + Copy + PartialEq {}
//     pub trait Semigroup: Magma + AssociativeAdd {}
//     impl<T> Semigroup for T where T: Magma + AssociativeAdd {}
//     pub trait Monoid: Semigroup + IdentityAdd {}
//     impl<T> Monoid for T where T: Semigroup + IdentityAdd {}
//     pub trait Group: Monoid + Sub<Output = Self> {}
//     impl<T> Group for T where T: Monoid + Sub<Output = Self> {}
//     pub trait AbelianGroup: Group + CommutativeAdd {}
//     impl<T> AbelianGroup for T where T: Group + CommutativeAdd {}
//     pub trait Ring: AbelianGroup + Mul<Output = Self> + IdentityMul {}
//     impl<T> Ring for T where T: AbelianGroup + Mul<Output = Self> + IdentityMul {}
//     pub trait CommutativeRing: Ring + CommutativeMul {}
//     impl<T> CommutativeRing for T where T: Ring + CommutativeMul {}
//     pub trait IntegralDomain: CommutativeRing + IntegralMul {}
//     impl<T> IntegralDomain for T where T: CommutativeRing + IntegralMul {}
//     pub trait Field: IntegralDomain + Div<Output = Self> {}
//     impl<T> Field for T where T: IntegralDomain + Div<Output = Self> {}
// }

// pub mod math {
//     use crate::algebraic_structure::Ring;
//     use crate::arithmetic::IdentityMul;
//     use crate::num_integer::Integer;

//     pub fn pow<T, E>(x: T, n: E) -> T
//     where
//         T: Ring,
//         E: Integer,
//     {
//         if n == E::ZERO {
//             return <T as IdentityMul>::ONE;
//         }
//         let ans = pow(x, n >> E::ONE);
//         let ans = ans * ans;
//         if (n & E::ONE) == E::ONE {
//             ans * x
//         } else {
//             ans
//         }
//     }

//     pub fn log2_floor<T>(x: T) -> i32
//     where
//         T: Integer,
//     {
//         let leading_zero = x.count_leading_zero();
//         T::BITS - leading_zero - 1
//     }
//     pub fn log2_ceil<T>(x: T) -> i32
//     where
//         T: Integer,
//     {
//         let res = log2_floor(x);
//         if res < 0 || (T::ONE << T::from_i32(res)) < x {
//             res + 1
//         } else {
//             res
//         }
//     }
// }
// pub mod sparse_table {
//     use crate::algebraic_structure::Monoid;
//     use crate::arithmetic::CommutativeAdd;
//     use crate::arithmetic::IdempotentAdd;
//     use crate::arithmetic::IdentityAdd;
//     use crate::math::log2_floor;
//     use crate::util::should;
//     use crate::util::should_eq;
//     use std::fmt;
//     use std::fmt::Display;

//     #[derive(Debug)]
//     pub struct SparseTable<T>
//     where
//         T: IdempotentAdd,
//     {
//         data: Vec<Vec<T>>,
//     }

//     impl<T> SparseTable<T>
//     where
//         T: IdempotentAdd,
//     {
//         pub fn new(s: &[T]) -> Self {
//             let n = s.len();
//             if n == 0 {
//                 return Self { data: Vec::new() };
//             }
//             let level = (log2_floor(n) + 1) as usize;
//             let mut data: Vec<Vec<T>> = vec![vec![s[0]; n]; level];
//             for i in 0..n {
//                 data[0][i] = s[i];
//             }

//             for i in 1..level {
//                 let step = 1usize << (i - 1);
//                 for j in 0..n {
//                     let k = j + step;
//                     if k < n {
//                         data[i][j] = data[i - 1][j] + data[i - 1][k];
//                     } else {
//                         data[i][j] = data[i - 1][j];
//                     }
//                 }
//             }

//             Self { data }
//         }

//         pub fn query(&self, l: usize, r: usize) -> T {
//             should!(l <= r);
//             let log = log2_floor(r - l + 1) as usize;
//             self.data[log][l] + self.data[log][r + 1 - (1usize << log)]
//         }
//     }
// }

// pub mod range_minimum_query {
//     use crate::binary::highest_k_one;
//     use crate::external_addition::MinimumAddition;
//     use crate::external_addition::Wrapper;
//     use crate::math::log2_floor;
//     use crate::num_integer::Integer;
//     use crate::sparse_table::SparseTable;
//     use crate::util::should;
//     use std::cmp::min;
//     use std::fmt::Debug;

//     const SHIFT: usize = 5;
//     const BLOCK_SIZE: usize = 1 << SHIFT;
//     const AND_MASK: usize = BLOCK_SIZE - 1;

//     #[derive(Debug)]
//     pub struct RangeMinimumQuery<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         data: Vec<T>,
//         to_left: Vec<usize>,
//         st: SparseTable<Wrapper<T, MinimumAddition<T>>>,
//     }

//     impl<T> RangeMinimumQuery<T>
//     where
//         T: Ord + Copy + Debug,
//     {
//         pub fn new(data: Vec<T>) -> Self {
//             let n = data.len();
//             if n == 0 {
//                 return Self {
//                     data,
//                     to_left: Vec::new(),
//                     st: SparseTable::new(&Vec::new()),
//                 };
//             }
//             let consider_part = ((n - 1) >> SHIFT) + 1;
//             let mut min_elements = Vec::with_capacity(consider_part);
//             for i in 0..n {
//                 let to = i >> SHIFT;
//                 let w: Wrapper<T, MinimumAddition<T>> = Wrapper::new(data[i]);
//                 if min_elements.len() <= to {
//                     min_elements.push(w);
//                 } else {
//                     min_elements[to] = min_elements[to] + w;
//                 }
//             }
//             let mut to_left = Vec::with_capacity(n);
//             let st = SparseTable::new(&min_elements[..]);
//             let mut mask = 0usize;
//             for i in 0..n {
//                 if (i & AND_MASK) == 0 {
//                     mask = 0;
//                 }
//                 let b = i >> SHIFT;
//                 while mask != 0 {
//                     let head = log2_floor(mask) as usize;
//                     if data[i] <= data[(b << SHIFT) | head] {
//                         mask = mask & !(1usize << head);
//                     } else {
//                         break;
//                     }
//                 }
//                 mask = mask | (1usize << (i & AND_MASK));
//                 to_left.push(mask);
//             }

//             Self { data, st, to_left }
//         }

//         pub fn query(&self, l: usize, r: usize) -> T {
//             should!(l <= r);
//             let bl = l >> SHIFT;
//             let br = r >> SHIFT;
//             let to = highest_k_one(32u32 - (l & AND_MASK) as u32) as usize;
//             let bs = bl << SHIFT;
//             if bl == br {
//                 let index = (self.to_left[r] & to).count_trailing_zero() as usize | bs;
//                 self.data[index]
//             } else {
//                 let idx_1 = (self.to_left[(bl << SHIFT) | AND_MASK] & to).count_trailing_zero()
//                     as usize
//                     | bs;
//                 let idx_2 = self.to_left[r].count_trailing_zero() as usize | (br << SHIFT);
//                 let mut best = min(self.data[idx_1], self.data[idx_2]);
//                 if bl + 1 < br {
//                     best = min(best, self.st.query(bl + 1, br - 1).value());
//                 }
//                 best
//             }
//         }
//     }
// }
// pub mod solver {
//     use crate::arithmetic::*;
//     use crate::fast_input::FastInput;
//     use crate::range_minimum_query::RangeMinimumQuery;
//     use crate::util::debug;
//     use std::cmp::min;
//     use std::io::BufWriter;
//     use std::io::StdinLock;
//     use std::io::StdoutLock;
//     use std::io::Write;
//     use std::ops::Add;
//     use std::panic;

//     pub unsafe fn solve_one(
//         test_id: u32,
//         fi: &mut FastInput<StdinLock>,
//         fo: &mut BufWriter<StdoutLock>,
//     ) {
//         let n = fi.ru();
//         let q = fi.ru();
//         let mut a = Vec::with_capacity(n);
//         for i in 0..n {
//             a.push(fi.ri());
//         }
//         let rmq = RangeMinimumQuery::new(a);
//         for i in 0..q {
//             let l = fi.ru();
//             let r = fi.ru() - 1;
//             writeln!(fo, "{}", rmq.query(l, r));
//         }
//     }

//     pub unsafe fn solve_multi(fi: &mut FastInput<StdinLock>, fo: &mut BufWriter<StdoutLock>) {
//         let t: u32 = 1;
//         for test_id in 1..t + 1 {
//             solve_one(test_id, fi, fo);
//         }
//     }
// }
// use util::should;

// use crate::fast_input::FastInput;
// use crate::solver::solve_multi;
// use std::io::BufWriter;
// use std::thread;

// unsafe fn run_in_current_thread() {
//     let stdin = std::io::stdin();
//     let stdout = std::io::stdout();
//     let mut fi = FastInput::new(stdin.lock());
//     let mut fo = BufWriter::new(stdout.lock());
//     solve_multi(&mut fi, &mut fo);
// }
// unsafe fn run_in_new_thread() {
//     thread::Builder::new()
//         .stack_size(256 << 20)
//         .spawn(|| {
//             run_in_current_thread();
//         })
//         .unwrap()
//         .join();
// }

// fn main() {
//     unsafe {
//         run_in_current_thread();
//     }
// }
