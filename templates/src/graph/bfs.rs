use std::collections::VecDeque;

pub fn bfs(graph: &[Vec<usize>], start: usize) -> Vec<u32> {
    let mut dist = vec![std::u32::MAX; graph.len()];
    dist[start] = 0;
    let mut queue = VecDeque::from(vec![start]);
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if dist[next] == std::u32::MAX {
                dist[next] = dist[cur] + 1;
                queue.push_back(next);
            }
        }
    }
    dist
}

/// Returns the distance and the previous node of each node from the start node.
pub fn bfs_restore(graph: &[Vec<usize>], start: usize) -> (Vec<u32>, Vec<usize>) {
    let mut dist = vec![std::u32::MAX; graph.len()];
    let mut prev = vec![std::usize::MAX; graph.len()];
    dist[start] = 0;
    prev[start] = start;
    let mut queue = VecDeque::from(vec![start]);
    while let Some(cur) = queue.pop_front() {
        for &next in &graph[cur] {
            if dist[next] == std::u32::MAX {
                dist[next] = dist[cur] + 1;
                prev[next] = cur;
                queue.push_back(next);
            }
        }
    }
    (dist, prev)
}

/// Returns the shortest path from the start node to the target node.
pub fn bfs_find_path(graph: &[Vec<usize>], start: usize, target: usize) -> Option<Vec<usize>> {
    let (dist, prev) = bfs_restore(graph, start);
    if dist[target] == std::u32::MAX {
        return None;
    }
    let mut res = vec![target];
    loop {
        let x = *res.last().unwrap();
        if x == start {
            res.reverse();
            return Some(res);
        }
        res.push(prev[x]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = vec![
            vec![1, 2],
            vec![0, 2, 3],
            vec![0, 1, 4],
            vec![1, 4],
            vec![2, 3],
        ];
        let dist = bfs(&graph, 0);
        assert_eq!(dist, vec![0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_bfs_restore() {
        let graph = vec![
            vec![1, 2],
            vec![0, 2, 3],
            vec![0, 1, 4],
            vec![1, 4],
            vec![2, 3],
        ];
        let (dist, prev) = bfs_restore(&graph, 0);
        assert_eq!(dist, vec![0, 1, 1, 2, 2]);
        assert_eq!(prev, vec![0, 0, 0, 1, 2]);
    }

    #[test]
    fn test_bfs_find_path() {
        // 0 -- 1 -- 3
        //  \   |    |
        //   \  |    |
        //      2 -- 4
        let graph = vec![
            vec![1, 2],
            vec![0, 2, 3],
            vec![0, 1, 4],
            vec![1, 4],
            vec![2, 3],
        ];
        assert_eq!(bfs_find_path(&graph, 0, 3), Some(vec![0, 1, 3]));
        assert_eq!(bfs_find_path(&graph, 0, 4), Some(vec![0, 2, 4]));
        assert_eq!(bfs_find_path(&graph, 0, 0), Some(vec![0]));
    }
}
