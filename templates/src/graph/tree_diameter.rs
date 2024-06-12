use crate::graph::bfs::{bfs, bfs_restore};

/// 木の直径を一つ探し、その両端点とその間の距離を返します。
pub fn tree_diamter(tree: &[Vec<usize>]) -> ([usize; 2], u32) {
    let dist = bfs(tree, 0);
    let &diam = dist.iter().max().unwrap();
    let x = dist.iter().position(|&d| d == diam).unwrap();
    let dist = bfs(tree, x);
    let &diam = dist.iter().max().unwrap();
    let y = dist.iter().position(|&d| d == diam).unwrap();
    ([x, y], diam)
}

/// 木の直径を一つ探し、頂点列とパスの長さ（辺の個数）を返します。
pub fn tree_diamter_restore(tree: &[Vec<usize>]) -> (Vec<usize>, u32) {
    let dist = bfs(tree, 0);
    let &diam = dist.iter().max().unwrap();
    let x = dist.iter().position(|&d| d == diam).unwrap();
    let (dist, prv) = bfs_restore(tree, x);
    let &diam = dist.iter().max().unwrap();
    let mut res = vec![dist.iter().position(|&d| d == diam).unwrap()];
    loop {
        let x = *res.last().unwrap();
        if dist[x] == 0 {
            break;
        }
        res.push(prv[x]);
    }
    (res, diam)
}
