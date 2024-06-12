#![allow(dead_code, unused_macros, unused_imports)]
pub use std::{
    cell::{Cell, RefCell, UnsafeCell},
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::{DefaultHasher, RandomState},
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
    },
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::{Debug, Display, Write as FmtWrite},
    hash::{BuildHasher, Hash, Hasher},
    io::{BufWriter, Read, Stdin, Stdout, Write},
    iter::{FromIterator, Peekable},
    marker::PhantomData,
    mem::swap,
    ops::*,
    process::exit,
    rc::Rc,
    str::{from_utf8_unchecked, FromStr},
    time::{Duration, Instant},
};

const IO_BUF_SIZE: usize = 1 << 16;
type Input = Scanner<Stdin>;
type Output = BufWriter<Stdout>;
fn _init_input() -> Input {
    Scanner::new(std::io::stdin())
}
fn _init_output() -> Output {
    BufWriter::with_capacity(IO_BUF_SIZE, std::io::stdout())
}

#[repr(transparent)]
struct Unsync<T>(T);
unsafe impl<T> Sync for Unsync<T> {}

type BadLazy<T> = Unsync<UnsafeCell<Option<T>>>;
impl<T> BadLazy<T> {
    const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }
}

static INPUT: BadLazy<Input> = BadLazy::new();
static OUTPUT: BadLazy<Output> = BadLazy::new();

fn inp<F: FnOnce(&mut Input) -> R, R>(f: F) -> R {
    unsafe { f((*INPUT.0.get()).get_or_insert_with(_init_input)) }
}
pub fn out_fn<F: FnOnce(&mut Output) -> R, R>(f: F) -> R {
    unsafe { f((*OUTPUT.0.get()).get_or_insert_with(_init_output)) }
}

macro_rules! read {
    () => { read_fn() };
    ($t: ty) => { read_fn::<$t>() };
    ($t: ty, $($tt: ty),*) => { (read_fn::<$t>(), $(read_fn::<$tt>(),)*) };
    [$t: ty; $n: expr] => { read_vec::<$t>($n) };
}
pub(crate) use read;

macro_rules! outln {
    () => { out_fn(|x| { let _ = writeln!(x); }) };
    ($exp: expr) => { out_fn(|x| { let _ = writeln!(x, "{}", $exp); }) };
    ($fmt: expr, $($arg : tt )*) => { out_fn(|x| { let _ = writeln!(x, $fmt, $($arg)*); }) }
}
pub(crate) use outln;

macro_rules! out {
    ($exp: expr) => { out_fn(|x| { let _ = write!(x, "{}", $exp); }) };
    ($fmt: expr, $($arg : tt )*) => { out_fn(|x| { let _ = write!(x, $fmt, $($arg)*); }) }
}
pub(crate) use out;

pub fn out_flush() {
    out_fn(|x| {
        let _ = x.flush();
    });
}

pub fn input_is_eof() -> bool {
    inp(|x| x.eof())
}
pub fn read_byte() -> u8 {
    inp(|x| x.byte())
}
pub fn read_bytes_no_skip(n: usize) -> Vec<u8> {
    inp(|x| x.bytes_no_skip(n))
}
pub fn read_bytes(n: usize) -> Vec<u8> {
    inp(|x| x.bytes(n))
}
pub fn read_bytes2(n: usize, m: usize) -> Vec<Vec<u8>> {
    inp(|x| x.bytes2(n, m))
}
pub fn read_token() -> Vec<u8> {
    inp(|x| x.token_bytes())
}
pub fn read_token_str() -> String {
    unsafe { String::from_utf8_unchecked(read_token()) }
}
pub fn read_line() -> Vec<u8> {
    inp(|x| x.line_bytes())
}
pub fn read_line_str() -> String {
    unsafe { String::from_utf8_unchecked(read_line()) }
}
pub fn read_fn<T: FromStr>() -> T {
    read_token_str().parse::<T>().ok().expect("failed parse")
}
pub fn read_vec<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read_fn()).collect()
}
pub fn read_vec2<T: FromStr>(n: usize, m: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec(m)).collect()
}

struct Scanner<R: Read> {
    src: R,
    _buf: Vec<u8>,
    _pt: usize, // pointer
    _rd: usize, // bytes read
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(src: R) -> Scanner<R> {
        Scanner {
            src,
            _buf: vec![0; IO_BUF_SIZE],
            _pt: 1,
            _rd: 1,
        }
    }

    fn _check_buf(&mut self) {
        if self._pt == self._rd {
            self._rd = self.src.read(&mut self._buf).unwrap_or(0);
            self._pt = (self._rd == 0) as usize;
        }
    }

    // returns true if end of file
    fn eof(&mut self) -> bool {
        self._check_buf();
        self._rd == 0
    }

    // filters \r, returns \0 if eof
    fn byte(&mut self) -> u8 {
        loop {
            self._check_buf();
            if self._rd == 0 {
                return 0;
            }
            let res = self._buf[self._pt];
            self._pt += 1;
            if res != b'\r' {
                return res;
            }
        }
    }

    fn bytes_no_skip(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.byte()).collect()
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let res = self.bytes_no_skip(n);
        self.byte();
        res
    }
    fn bytes2(&mut self, n: usize, m: usize) -> Vec<Vec<u8>> {
        (0..n).map(|_| self.bytes(m)).collect()
    }

    fn token_bytes(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut c = self.byte();
        while c <= b' ' {
            if c == b'\0' {
                return res;
            }
            c = self.byte();
        }
        loop {
            res.push(c);
            c = self.byte();
            if c <= b' ' {
                return res;
            }
        }
    }

    fn line_bytes(&mut self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut c = self.byte();
        while c != b'\n' && c != b'\0' {
            res.push(c);
            c = self.byte();
        }
        res
    }
}

pub trait JoinToStr {
    fn join_to_str(self, sep: &str) -> String;
    fn concat_to_str(self) -> String;
}
impl<T: Display, I: Iterator<Item = T>> JoinToStr for I {
    fn join_to_str(mut self, sep: &str) -> String {
        match self.next() {
            Some(first) => {
                let mut res = first.to_string();
                for item in self {
                    res.push_str(sep);
                    res.push_str(&item.to_string());
                }
                res
            }
            None => String::new(),
        }
    }

    fn concat_to_str(self) -> String {
        let mut res = String::new();
        for item in self {
            res.push_str(&item.to_string());
        }
        res
    }
}
pub trait AsStr {
    fn as_str(&self) -> &str;
}
impl AsStr for [u8] {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).expect("attempt to convert non-UTF8 byte string.")
    }
}

macro_rules! veci {
    ($n:expr , $i:ident : $gen:expr) => {{
        let _veci_n = $n;
        let mut _veci_list = Vec::with_capacity(_veci_n);
        for $i in 0.._veci_n {
            _veci_list.push($gen);
        }
        _veci_list
    }};
    ($n:expr , $gen:expr) => { veci!($n, _veci_: $gen) }
}
pub(crate) use veci;

pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

pub trait CommonNumExt {
    fn div_ceil(self, b: Self) -> Self;
    fn div_floor(self, b: Self) -> Self;
    fn gcd(self, b: Self) -> Self;
    fn highest_one(self) -> Self;
    fn lowest_one(self) -> Self;
    fn bit_length(self) -> u32;
    fn bit_count(self) -> u32;
    fn log2_floor(self) -> u32;
    fn log2_ceil(self) -> u32;
}

macro_rules! impl_common_num_ext {
    ($($ix:tt = $ux:tt),*) => {
        $(
            impl CommonNumExt for $ux {
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
                #[inline] fn bit_count(self) -> u32 { self.count_ones() }
                #[inline] fn log2_floor(self) -> u32 { std::mem::size_of::<$ux>() as u32 * 8 - self.leading_zeros() - 1 }
                #[inline] fn log2_ceil(self) -> u32 { std::mem::size_of::<$ux>() as u32 * 8 - self.leading_zeros() - 1 + (self & (self - 1) != 0) as u32 }
            }

            impl CommonNumExt for $ix {
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
                #[inline] fn bit_count(self) -> u32 { self.count_ones() }
                #[inline] fn log2_floor(self) -> u32 { std::mem::size_of::<$ix>() as u32 * 8 - self.leading_zeros() - 1 }
                #[inline] fn log2_ceil(self) -> u32 { std::mem::size_of::<$ix>() as u32 * 8 - self.leading_zeros() - 1 + (self & (self - 1) != 0) as u32 }
            }
        )*
    }
}
impl_common_num_ext!(
    i8 = u8,
    i16 = u16,
    i32 = u32,
    i64 = u64,
    i128 = u128,
    isize = usize
);

pub trait ChMaxMin<T> {
    fn chmax(&mut self, v: T) -> bool;
    fn chmin(&mut self, v: T) -> bool;
}
impl<T: PartialOrd> ChMaxMin<T> for Option<T> {
    fn chmax(&mut self, v: T) -> bool {
        if self.is_none() || v > *self.as_ref().unwrap() {
            *self = Some(v);
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, v: T) -> bool {
        if self.is_none() || v < *self.as_ref().unwrap() {
            *self = Some(v);
            true
        } else {
            false
        }
    }
}
impl<T: PartialOrd> ChMaxMin<T> for T {
    fn chmax(&mut self, v: T) -> bool {
        if v > *self {
            *self = v;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, v: T) -> bool {
        if v < *self {
            *self = v;
            true
        } else {
            false
        }
    }
}

pub fn reduce_mut<T>(a: &mut [T], mut f: impl FnMut(&mut T, &mut T)) {
    if !a.is_empty() {
        for i in 1..a.len() {
            let (left, right) = a.split_at_mut(i);
            f(left.last_mut().unwrap(), right.first_mut().unwrap());
        }
    }
}

pub fn reduce_right_mut<T>(a: &mut [T], mut f: impl FnMut(&mut T, &mut T)) {
    if !a.is_empty() {
        for i in (1..a.len()).rev() {
            let (left, right) = a.split_at_mut(i);
            f(right.first_mut().unwrap(), left.last_mut().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for v in 1..=10 {
            println!(
                "{}: {}:{}:{}",
                v,
                v.bit_length(),
                v.log2_floor(),
                v.log2_ceil()
            );
        }
    }

    #[test]
    fn test_reduce_mut() {
        let mut a = vec![1, 2, 3, 4, 5];
        reduce_mut(&mut a, |&mut pre, cur| *cur += pre);
        assert_eq!(a, vec![1, 3, 6, 10, 15]);

        let mut a = vec![1, 2, 3, 4, 5];
        reduce_right_mut(&mut a, |&mut pre, cur| *cur += pre);
        println!("{:?}", a);
        assert_eq!(a, vec![15, 14, 12, 9, 5]);
    }
}
