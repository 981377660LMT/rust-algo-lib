use crate::template::*;

pub mod rmq {
    use super::CommonNumExt;

    struct SparseTable<T> {
        table: Vec<Vec<T>>,
        size: usize,
        op: fn(T, T) -> T,
    }

    impl<T: Copy> SparseTable<T> {
        pub fn from_vec(mut v: Vec<T>, op: fn(T, T) -> T) -> Self {
            let n = v.len();
            if n == 0 {
                return SparseTable {
                    table: vec![],
                    size: 0,
                    op,
                };
            }
            let mut table = vec![];
            let mut w = 1;
            while w < n {
                let next = v
                    .iter()
                    .zip(v[w..].iter())
                    .map(|p| op(*p.0, *p.1))
                    .collect::<Vec<_>>();
                table.push(v);
                v = next;
                w <<= 1;
            }

            SparseTable { table, size: n, op }
        }

        pub fn from_fn(n: usize, f: fn(usize) -> T, op: fn(T, T) -> T) -> Self {
            if n == 0 {
                return SparseTable {
                    table: vec![],
                    size: 0,
                    op,
                };
            }
            let level = n.sig_bits();
        }

        pub fn query(&self, start: usize, end: usize) -> T {}
    }
}

#[allow(unused)]
fn main() {
    let (n, q) = read!(u32, u32);
    outln!(n);
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
