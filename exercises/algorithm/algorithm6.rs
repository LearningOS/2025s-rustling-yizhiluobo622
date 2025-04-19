use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); // 无向图
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 如果当前节点已经访问过，直接返回
        if visited.contains(&v) {
            return;
        }

        // 标记当前节点为已访问
        visited.insert(v);
        // 将当前节点加入访问顺序
        visit_order.push(v);

        // 遍历当前节点的所有邻居
        for &neighbor in &self.adj[v] {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }

    // 对图执行深度优先搜索，返回访问节点的顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); // DFS 顺序可能因邻接表顺序不同而变化，但这里假定顺序一致
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); // 自环

        let visit_order = graph.dfs(0);
        // DFS 顺序可能为 [0, 1, 2, 3] 或其他合法顺序，具体取决于邻接表顺序
        let expected = vec![0, 1, 2, 3];
        assert!(visit_order.windows(expected.len()).any(|w| w == expected.as_slice()));
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); // 访问第一个连通分量

        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); // 访问第二个连通分量
    }
}