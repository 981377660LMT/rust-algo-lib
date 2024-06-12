use crate::template::*;

pub mod fast_set {
    use std::fmt::Debug;

    use super::*;

    #[derive(Debug, Clone, Default)]
    struct SimpleBitSet(Vec<u64>);

    impl SimpleBitSet {
        pub fn new(n: usize) -> Self {
            Self(vec![0; n.div_ceil(64)])
        }

        pub fn new_with_filled(n: usize, filled: bool) -> Self {
            if filled {
                let mut res = Self(vec![!0; n.div_ceil(64)]);
                let div = n >> 6;
                let mod_ = n & 63;
                if mod_ != 0 {
                    res.0[div] >>= 64 - mod_;
                }
                res
            } else {
                Self::new(n)
            }
        }

        pub fn insert(&mut self, i: usize) -> bool {
            let (div, mod_) = (i >> 6, i & 63);
            let pre = self.0[div];
            self.0[div] |= 1 << mod_;
            pre != self.0[div]
        }

        pub fn remove(&mut self, i: usize) -> bool {
            let (div, mod_) = (i >> 6, i & 63);
            let pre = self.0[div];
            self.0[div] &= !(1 << mod_);
            pre != self.0[div]
        }

        pub fn contains(&self, i: usize) -> bool {
            let (div, mod_) = (i >> 6, i & 63);
            self.0[div] >> mod_ & 1 == 1
        }

        pub fn set(&mut self, i: usize, b: bool) -> bool {
            if b {
                self.insert(i)
            } else {
                self.remove(i)
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct FastSet {
        n: usize,
        bs: Vec<SimpleBitSet>,
    }

    impl FastSet {
        /// Creates a new `FastSet` with `n` elements.
        /// [0, n) is the range of elements.
        pub fn new_with_filled(n: usize, filled: bool) -> Self {
            if n == 0 {
                Self::default()
            } else {
                let mut bs = Vec::with_capacity(max(1, (n - 1).sig_bits().div_ceil(6) as usize));
                let mut m = n;
                loop {
                    bs.push(SimpleBitSet::new_with_filled(m, filled));
                    m = m.div_ceil(64);
                    if m == 1 {
                        break;
                    }
                }
                Self { n, bs }
            }
        }

        pub fn set(&mut self, mut i: usize, mut b: bool) -> bool {
            for j in 0..self.bs.len() {
                if !self.bs[j].set(i, b) {
                    return j != 0;
                }
                i >>= 6;
                b = self.bs[j].0[i] != 0;
            }
            true
        }

        pub fn insert(&mut self, i: usize) -> bool {
            self.set(i, true)
        }

        pub fn remove(&mut self, i: usize) -> bool {
            self.set(i, false)
        }

        pub fn contains(&self, i: usize) -> bool {
            self.bs[0].contains(i)
        }

        /// Returns the smallest element in the set greater than or equal to `i`.
        pub fn ceil(&self, mut i: usize) -> Option<usize> {
            if i >= self.n {
                return None;
            }
            let mut j = 0;
            while j < self.bs.len() {
                let div = i >> 6;
                let mod_ = i & 63;
                if div >= self.bs[j].0.len() {
                    return None;
                }
                let mask = self.bs[j].0[div] >> mod_;
                if mask == 0 {
                    i = (i >> 6) + 1;
                    j += 1;
                } else {
                    i += mask.trailing_zeros() as usize;
                    break;
                }
            }
            if j == self.bs.len() {
                return None;
            }
            while j > 0 {
                j -= 1;
                let mask = self.bs[j].0[i];
                i = (i << 6) + mask.trailing_zeros() as usize;
            }
            Some(i)
        }

        /// Returns the largest element in the set less than or equal to `i`.
        pub fn floor(&self, mut i: usize) -> Option<usize> {
            let mut j = 0;
            while j < self.bs.len() {
                let div = i >> 6;
                let mod_ = i & 63;
                let mask = self.bs[j].0[div] << (63 - mod_);
                if mask == 0 {
                    i >>= 6;
                    if i == 0 {
                        return None;
                    }
                    i -= 1;
                    j += 1;
                } else {
                    i -= mask.leading_zeros() as usize;
                    break;
                }
            }
            if j == self.bs.len() {
                return None;
            }
            while j > 0 {
                j -= 1;
                let mask = self.bs[j].0[i];
                i = (i << 6) + (63 - mask.leading_zeros()) as usize;
            }
            Some(i)
        }

        pub fn iter(&self) -> FastSetIter<'_> {
            self.iter_with_start(0)
        }

        pub fn iter_with_start(&self, start: usize) -> FastSetIter<'_> {
            FastSetIter(self, start)
        }
    }

    impl Display for FastSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let res: Vec<usize> = self.iter().collect();
            res.fmt(f)
        }
    }

    pub struct FastSetIter<'a>(&'a FastSet, usize);
    impl<'a> Iterator for FastSetIter<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0.ceil(self.1) {
                Some(x) => {
                    self.1 = x + 1;
                    Some(x)
                }
                None => None,
            }
        }
    }
}

#[allow(unused)]
// https://judge.yosupo.jp/problem/predecessor_problem
fn main() {
    use fast_set::FastSet;
    let (n, q) = read!(usize, usize);
    let mut set = FastSet::new_with_filled(n, false);
    for i in 0..n {
        set.set(i, read_byte() == b'1');
    }
    for _ in 0..q {
        let (op, v) = read!(u8, usize);
        match op {
            0 => {
                set.insert(v);
            }
            1 => {
                set.remove(v);
            }
            2 => {
                outln!(set.contains(v) as u8);
            }
            3 => {
                outln!(set.ceil(v).map_or(-1, |x| x as i32));
            }
            4 => {
                outln!(set.floor(v).map_or(-1, |x| x as i32));
            }
            _ => unreachable!(),
        }
    }

    out_flush();
}

#[cfg(test)]
mod tests {
    use fast_set::FastSet;

    use super::*;

    #[test]
    fn test() {
        let set = FastSet::new_with_filled(10, true);
        assert_eq!(set.iter().collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
        let iter = set.iter();
        for x in iter {
            println!("{}", x);
        }
    }
}
