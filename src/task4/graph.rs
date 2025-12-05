use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::task4::task_results::{DistancesMatrix, GraphResults, PathsMatrix};

type Vertex = usize;
type Weight = f64;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub directed: bool,
    pub adj: Vec<Vec<(Vertex, Weight)>>,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        Self {
            n,
            directed,
            adj: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, u: Vertex, v: Vertex, w: Weight) {
        self.adj[u].push((v, w));
        if !self.directed {
            self.adj[v].push((u, w));
        }
    }

    pub fn parse_adjust_from_file<P: AsRef<Path>>(path: P) -> io::Result<Graph> {
        let file = File::open(path)?;

        let mut lines = io::BufReader::new(file).lines();

        let n: usize = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing n"))??
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid n"))?;

        let mut temp_edges: Vec<(usize, usize, f64)> = Vec::new();

        for (u, line) in lines.enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            for pair in line.split_whitespace() {
                let mut it = pair.split(":");
                let v = it
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad vertex"))?
                    .parse::<Vertex>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad vertex"))?
                    - 1; // 0 based indexing

                let w = it
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad weight"))?
                    .parse::<Weight>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad weight"))?;

                temp_edges.push((u, v, w));
            }
        }

        let mut is_directed = false;

        for &(u, v, w) in &temp_edges {
            let mut found_rev = false;
            for &(x, y, w2) in &temp_edges {
                if x == v && y == u && (w2 - w).abs() < 1e-12 {
                    found_rev = true;
                    break;
                }
            }
            if !found_rev {
                is_directed = true;
                break;
            }
        }

        let mut g = Graph::new(n, is_directed);
        for (u, v, w) in temp_edges {
            g.add_edge(u, v, w);
        }

        Ok(g)
    }

    pub fn parse_edgelist_from_file<P: AsRef<Path>>(path: P) -> io::Result<Graph> {
        let file = File::open(path)?;
        let mut lines = io::BufReader::new(file).lines();

        let n: usize = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing n"))??
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid n"))?;

        let mut temp_edges: Vec<(usize, usize, f64)> = Vec::new();

        for line in lines {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split_whitespace();

            let u = parts
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing u"))?
                .parse::<Vertex>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad u"))?
                - 1;

            let v = parts
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing v"))?
                .parse::<Vertex>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad v"))?
                - 1;

            let w = parts
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing w"))?
                .parse::<Weight>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad w"))?;

            temp_edges.push((u, v, w));
        }

        let mut is_directed = false;

        for &(u, v, w) in &temp_edges {
            let mut found_rev = false;
            for &(x, y, w2) in &temp_edges {
                if x == v && y == u && (w2 - w).abs() < 1e-12 {
                    found_rev = true;
                    break;
                }
            }
            if !found_rev {
                is_directed = true;
                break;
            }
        }

        let mut g = Graph::new(n, is_directed);
        for (u, v, w) in temp_edges {
            g.add_edge(u, v, w);
        }

        Ok(g)
    }

    pub fn parse_matrix_from_file<P: AsRef<Path>>(path: P) -> io::Result<Graph> {
        let file = File::open(path)?;
        let mut lines = io::BufReader::new(file).lines();

        let n: usize = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing n"))??
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid n"))?;

        let mut temp_edges: Vec<(usize, usize, f64)> = Vec::new();

        for (i, line) in lines.enumerate() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().map(|s| s.trim()).collect();

            if parts.len() != n {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "wrong number of columns",
                ));
            }

            for (j, token) in parts.iter().enumerate() {
                let w: f64 = token.parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "bad weight in matrix")
                })?;

                if w != 0.0 {
                    temp_edges.push((i, j, w));
                }
            }
        }

        let mut is_directed = false;

        for &(u, v, w) in &temp_edges {
            let mut found_rev = false;

            for &(x, y, w2) in &temp_edges {
                if x == v && y == u && (w2 - w).abs() < 1e-12 {
                    found_rev = true;
                    break;
                }
            }

            if !found_rev {
                is_directed = true;
                break;
            }
        }

        let mut g = Graph::new(n, is_directed);

        for (u, v, w) in temp_edges {
            g.add_edge(u, v, w);
        }

        Ok(g)
    }
}

impl Graph {
    pub fn analyze(&self) -> GraphResults {
        let n = self.n;

        let mut deg_out = vec![0; n];
        let mut deg_in = vec![0; n];
        (0..n).for_each(|u| {
            deg_out[u] = self.adj[u].len();
            for &(v, _) in &self.adj[u] {
                deg_in[v] += 1;
            }
        });

        let mut weak_components = self.weakly_connected_components();
        for c in &mut weak_components {
            c.sort();
        }
        weak_components.sort_by_key(|c| c[0]);

        let mut strong_components = self.strongly_connected_components();
        for c in &mut strong_components {
            c.sort();
        }
        strong_components.sort_by_key(|c| c[0]);

        //TODO: redo
        let start = 0;
        let end = n - 1;

        let mut distances: DistancesMatrix = vec![vec![None; n]; n];
        let mut paths: PathsMatrix = vec![vec![None; n]; n];
        let mut selected_pairs = Vec::new();

        for u in start..=end {
            let (dist_u, prev_u) = self.dijkstra(u);
            distances[u] = dist_u.clone();

            for v in 0..n {
                if u != v && distances[u][v].is_some() {
                    // Reconstruct path
                    let mut path = Vec::new();
                    let mut current = v;
                    while let Some(p) = prev_u[current] {
                        path.push(current);
                        current = p;
                    }
                    path.push(u);
                    path.reverse();
                    paths[u][v] = Some(path);
                    selected_pairs.push((u, v));
                }
            }
        }

        GraphResults {
            directed: self.directed,
            deg_in,
            deg_out,
            weak_components,
            strong_components,
            distances,
            paths,
            selected_pairs,
            diameter: None,
            radius: None,
            centers: Vec::new(),
            periphery: Vec::new(),
        }
    }

    pub fn weakly_connected_components(&self) -> Vec<Vec<usize>> {
        let n = self.n;
        let mut visited = vec![false; n];
        let mut components = Vec::new();

        for v in 0..n {
            if !visited[v] {
                let mut comp = Vec::new();
                self.dfs_undirected(v, &mut visited, &mut comp);
                components.push(comp);
            }
        }
        components
    }

    fn dfs_undirected(&self, u: usize, visited: &mut [bool], comp: &mut Vec<usize>) {
        visited[u] = true;
        comp.push(u);
        for &(v, _) in &self.adj[u] {
            if !visited[v] {
                self.dfs_undirected(v, visited, comp);
            }
        }
        if self.directed {
            for i in 0..self.n {
                if self.adj[i].iter().any(|&(v, _)| v == u) && !visited[i] {
                    self.dfs_undirected(i, visited, comp);
                }
            }
        }
    }

    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        if !self.directed {
            return self.weakly_connected_components();
        }

        let n = self.n;
        let mut visited = vec![false; n];
        let mut order = Vec::new();

        for v in 0..n {
            if !visited[v] {
                self.dfs_for_order(v, &mut visited, &mut order);
            }
        }

        let mut rev_adj = vec![Vec::new(); n];
        for u in 0..n {
            for &(v, w) in &self.adj[u] {
                rev_adj[v].push((u, w));
            }
        }

        visited.fill(false);
        let mut components = Vec::new();
        while let Some(u) = order.pop() {
            if !visited[u] {
                let mut comp = Vec::new();
                Self::dfs_collect(u, &rev_adj, &mut visited, &mut comp);
                components.push(comp);
            }
        }
        components
    }

    fn dfs_for_order(&self, u: usize, visited: &mut [bool], order: &mut Vec<usize>) {
        visited[u] = true;
        for &(v, _) in &self.adj[u] {
            if !visited[v] {
                self.dfs_for_order(v, visited, order);
            }
        }
        order.push(u);
    }

    fn dfs_collect(
        u: usize,
        adj: &[Vec<(usize, f64)>],
        visited: &mut [bool],
        comp: &mut Vec<usize>,
    ) {
        visited[u] = true;
        comp.push(u);
        for &(v, _) in &adj[u] {
            if !visited[v] {
                Self::dfs_collect(v, adj, visited, comp);
            }
        }
    }

    pub fn dijkstra(&self, start: usize) -> (Vec<Option<f64>>, Vec<Option<usize>>) {
        let n = self.n;
        let mut distances = vec![None; n];
        let mut prev = vec![None; n];
        let mut heap = BinaryHeap::new();

        distances[start] = Some(0.0);
        heap.push(State {
            cost: 0.0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            let cost = -cost;
            if matches!(distances[position], Some(d) if cost > d) {
                continue;
            }

            for &(neighbor, weight) in &self.adj[position] {
                let next_cost = cost + weight;
                if distances[neighbor].is_none() || next_cost < distances[neighbor].unwrap() {
                    distances[neighbor] = Some(next_cost);
                    prev[neighbor] = Some(position);
                    heap.push(State {
                        cost: -next_cost,
                        position: neighbor,
                    });
                }
            }
        }
        (distances, prev)
    }
}

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
