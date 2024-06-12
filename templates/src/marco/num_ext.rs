pub trait NumExt {
    fn div_ceil(self, b: Self) -> Self;
    fn div_floor(self, b: Self) -> Self;
    fn gcd(self, b: Self) -> Self;
    fn highest_one(self) -> Self;
    fn lowest_one(self) -> Self;
    fn bit_length(self) -> u32;
}

macro_rules! impl_num_ext {
($($ix:tt = $ux:tt),*) => {
    $(
        impl NumExt for $ux {
            fn div_ceil(self, b: Self) -> Self {
                let q = self / b; let r = self % b;
                if r != 0 { q + 1 } else { q }
            }
            fn div_floor(self, b: Self) -> Self { self / b }
            fn gcd(self, mut b: Self) -> Self {
                let mut a = self;
                if a == 0 || b == 0 { return a | b; }
                let shift = (a | b).trailing_zeros();
                a >>= a.trailing_zeros();
                b >>= b.trailing_zeros();
                while a != b {
                    if a > b { a -= b; a >>= a.trailing_zeros(); }
                    else { b -= a; b >>= b.trailing_zeros(); }
                }
                a << shift
            }
            #[inline] fn highest_one(self) -> Self {
                if self == 0 { 0 } else { const ONE: $ux = 1; ONE << self.bit_length() - 1 }
            }
            #[inline] fn lowest_one(self) -> Self { self & self.wrapping_neg() }
            #[inline] fn bit_length(self) -> u32 { std::mem::size_of::<$ux>() as u32 * 8 - self.leading_zeros() }
        }

        impl NumExt for $ix {
            fn div_ceil(self, b: Self) -> Self {
                let q = self / b; let r = self % b;
                if self ^ b >= 0 && r != 0 { q + 1 } else { q }
            }
            fn div_floor(self, b: Self) -> Self {
                let q = self / b; let r = self % b;
                if self ^ b < 0 && r != 0 { q - 1 } else { q }
            }
            fn gcd(self, b: Self) -> Self {
                fn w_abs(x: $ix) -> $ux { (if x.is_negative() { x.wrapping_neg() } else { x }) as _ }
                w_abs(self).gcd(w_abs(b)) as _
            }
            #[inline] fn highest_one(self) -> Self { (self as $ux).highest_one() as _ }
            #[inline] fn lowest_one(self) -> Self { self & self.wrapping_neg() }
            #[inline] fn bit_length(self) -> u32 { std::mem::size_of::<$ix>() as u32 * 8 - self.leading_zeros() }
        }
    )*
  }
}

impl_num_ext!(
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = 10;
        let y = 3;
        assert_eq!(x.div_ceil(y), 4);
    }
}
