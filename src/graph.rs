use std::collections::{VecDeque, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: i32,
    dist: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug)]
pub struct Graph{
    pub nodes: HashMap<i32, Vec<i32>>,
}

impl Graph{
    pub fn new() -> Self{
        Graph{
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: i32, dest: i32){
        self.nodes.entry(src).or_insert_with(Vec::new).push(dest);
    }

    pub fn in_degree(&self, node:i32) -> usize{
        self.nodes.iter().filter(|(_,edges)| edges.contains(&node)).count()
    }

    pub fn out_degree(&self, node:i32) -> usize{
        self.nodes.get(&node).map_or(0, |edges| edges.len())
    }

    pub fn deg_centrality(&self) -> HashMap<i32, f64>{
        let num_nodes = self.nodes.len() as f64;
        let mut centrality = HashMap::new();
        for node in self.nodes.keys(){
            let in_deg = self.in_degree(*node) as f64;
            let out_deg = self.out_degree(*node) as f64;            
            let degree = in_deg+out_deg;
            let c = degree/ (num_nodes-1.0);
            centrality.insert(*node, c);
        }
        centrality
    }

    pub fn shortest_path(&self, start: i32, end: i32) -> Option<Vec<i32>> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        let mut path = HashMap::new();
    
        visited.insert(start, true);
        distances.insert(start, 0);
        queue.push_back(start);
    
        while let Some(node) = queue.pop_front() {
            for neighbor in self.nodes.get(&node).unwrap_or(&vec![]) {
                if !visited.contains_key(neighbor) {
                    visited.insert(*neighbor, true);
                    distances.insert(*neighbor, distances.get(&node).unwrap() + 1);
                    path.insert(*neighbor, node);
                    queue.push_back(*neighbor);
                }
            }
        }
    
        if !visited.contains_key(&end) {
            return None;
        }
    
        let mut shortest_path = vec![end];
        let mut current_node = end;
        while let Some(prev_node) = path.get(&current_node) {
            shortest_path.push(*prev_node);
            current_node = *prev_node;
            if *prev_node == start {
                break;
            }
        }
    
        shortest_path.reverse();
        Some(shortest_path)
    }

}