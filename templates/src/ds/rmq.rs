use crate::template::*;

#[allow(clippy::module_inception)]
pub mod rmq {
    use super::CommonNumExt;

    pub struct SparseTable<T> {
        table: Vec<Vec<T>>,
        size: usize,
        op: fn(T, T) -> T,
    }

    impl<T: Copy> SparseTable<T> {
        pub fn from_vec(mut v: Vec<T>, op: fn(T, T) -> T) -> Self {
            let size = v.len();
            if size == 0 {
                return SparseTable {
                    table: vec![],
                    size: 0,
                    op,
                };
            }
            let mut table = vec![];
            let mut w = 1;
            while w < v.len() {
                let next = v
                    .iter()
                    .zip(v[w..].iter())
                    .map(|p| op(*p.0, *p.1))
                    .collect::<Vec<_>>();
                table.push(v);
                v = next;
                w <<= 1;
            }
            table.push(v);

            SparseTable { table, size, op }
        }

        pub fn from_fn(size: usize, f: fn(usize) -> T, op: fn(T, T) -> T) -> Self {
            if size == 0 {
                return SparseTable {
                    table: vec![],
                    size: 0,
                    op,
                };
            }
            let level = (size.bit_length()) as usize;
            let mut table: Vec<Vec<T>> = vec![vec![f(0); size]; level];
            for i in 0..size {
                table[0][i] = f(i);
            }

            for i in 1..level {
                let step = 1usize << (i - 1);
                for j in 0..size {
                    let k = j + step;
                    if k < size {
                        table[i][j] = op(table[i - 1][j], table[i - 1][k]);
                    } else {
                        table[i][j] = table[i - 1][j];
                    }
                }
            }

            SparseTable { table, size, op }
        }

        pub fn query(&self, start: usize, end: usize) -> T {
            assert!(start < end && end <= self.size);
            let k = (end - start + 1).next_power_of_two().trailing_zeros() as usize - 1;
            let table = &self.table[k];
            (self.op)(table[start], table[end - (1 << k)])
        }
    }

    pub struct LinearRmq<T> {
        data: Vec<T>,
        table: SparseTable<T>,
        bit: Vec<usize>,
        op: fn(T, T) -> T,
    }

    impl<T: Copy + Ord> LinearRmq<T> {
        pub fn from_vec(data: Vec<T>, op: fn(T, T) -> T) -> Self {
            if data.is_empty() {
                return LinearRmq {
                    data: vec![],
                    table: SparseTable::from_vec(vec![], op),
                    bit: vec![],
                    op,
                };
            }

            let mut bit = vec![0; data.len()];
            let w = 8 * std::mem::size_of_val(&bit[0]);
            let mut stack: Vec<usize> = vec![];
            let mut table_ini = Vec::with_capacity((data.len() + w - 1) / w);
            for (bit, data) in bit.chunks_mut(w).zip(data.chunks(w)) {
                stack.clear();
                let mut b = 0;
                for (i, (bit, d)) in bit.iter_mut().zip(data.iter()).enumerate() {
                    while stack.last().map_or(false, |x| data[*x] > *d) {
                        b ^= 1 << stack.pop().unwrap();
                    }
                    b |= 1 << i;
                    *bit = b;
                    stack.push(i);
                }
                table_ini.push(data[stack[0]]);
            }
            let table = SparseTable::from_vec(table_ini, op);

            LinearRmq {
                data,
                table,
                bit,
                op,
            }
        }

        pub fn query(&self, start: usize, end: usize) -> T {
            assert!(start < end && end <= self.data.len());
            let w = 8 * std::mem::size_of_val(&self.bit[0]);
            let end = end - 1;
            let p = start / w;
            let q = end / w;
            if p == q {
                let pos = start + (self.bit[end] >> (start % w)).trailing_zeros() as usize;
                self.data[pos]
            } else {
                let pos = start
                    + (self.bit[start / w * w + w - 1] >> (start % w)).trailing_zeros() as usize;
                let mut res = self.data[pos];
                let pos = end / w * w + self.bit[end].trailing_zeros() as usize;
                res = (self.op)(res, self.data[pos]);
                if p + 1 < q {
                    res = (self.op)(res, self.table.query(p + 1, q));
                }
                res
            }
        }
    }
}

#[allow(unused)]
fn main() {
    use rmq::LinearRmq;
    let (n, q) = read!(usize, usize);
    let nums = read![i32; n];
    let st = LinearRmq::from_vec(nums, std::cmp::min);
    for _ in 0..q {
        let (l, r) = read!(usize, usize);
        let res = st.query(l, r);
        outln!(res);
    }

    out_flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
}
