use std::collections::{HashMap, HashSet};

struct Graph {
    adjacency_list: HashMap<u32, HashSet<u32>>,
    verticles: HashSet<u32>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            verticles: HashSet::new(),
        }
    }
    fn add_vertex(&mut self, vertex: u32) {
        self.verticles.insert(vertex);
        self.adjacency_list.insert(vertex, HashSet::new());
    }
    fn remove_vertex(&mut self, vertex: u32) {
        self.verticles.remove(&vertex);
        self.adjacency_list.remove(&vertex);
        for (_, neighbours) in self.adjacency_list.iter_mut() {
            neighbours.remove(&vertex);
        }
    }
    fn add_edge(&mut self, v1: u32, v2: u32) {
        self.adjacency_list
            .entry(v1)
            .or_insert(HashSet::new())
            .insert(v2);
        self.adjacency_list
            .entry(v2)
            .or_insert(HashSet::new())
            .insert(v1);
    }

    fn remove_edge(&mut self, v1: u32, v2: u32) {
        if let Some(neighbors) = self.adjacency_list.get_mut(&v1) {
            neighbors.remove(&v2);
        }

        if let Some(neighbors) = self.adjacency_list.get_mut(&v2) {
            neighbors.remove(&v1);
        }
    }

    fn has_edge(&self, v1: u32, v2: u32) -> bool {
        if let Some(neighbors) = self.adjacency_list.get(&v1) {
            neighbors.contains(&v2)
        } else {
            false
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);

    println!("Verticles: {:?}", graph.verticles);
    println!("Adjacency List: {:?}", graph.adjacency_list);
    graph.remove_edge(1, 2);
    graph.remove_vertex(2);

    println!("Verticles: {:?}", graph.verticles);
    println!("Adjacency List: {:?}", graph.adjacency_list);

    println!("Edge (1, 3) exists: {}", graph.has_edge(1, 3));
    println!("Edge (2, 3) exists: {}", graph.has_edge(2, 3));
}
